use std::process::Command;

pub fn run_wpctl(args: &[&str]) -> Result<(), String> {
    let out = Command::new("wpctl")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run wpctl {:?}: {}", args, e))?;

    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "wpctl {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

pub fn get_wpctl_status() -> Result<String, String> {
    let out = Command::new("wpctl")
        .arg("status")
        .output()
        .map_err(|e| format!("Failed to run wpctl status: {}", e))?;

    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}

// Returns (sinks, sources) as Vec<(id, label)>
pub fn parse_wpctl_status_for_devices(text: &str) -> (Vec<(i32, String)>, Vec<(i32, String)>) {
    let mut sinks: Vec<(i32, String)> = Vec::new();
    let mut sources: Vec<(i32, String)> = Vec::new();

    enum Section {
        None,
        Sinks,
        Sources,
    }
    let mut section = Section::None;

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("├─ Sinks:") || trimmed.starts_with("└─ Sinks:") {
            section = Section::Sinks;
            continue;
        }
        if trimmed.starts_with("├─ Sources:") || trimmed.starts_with("└─ Sources:") {
            section = Section::Sources;
            continue;
        }

        if trimmed.starts_with("├─ ") || trimmed.starts_with("└─ ") {
            match trimmed {
                "├─ Sinks:" | "└─ Sinks:" | "├─ Sources:" | "└─ Sources:" => {}
                _ => section = Section::None,
            }
            continue;
        }

        let in_section = matches!(section, Section::Sinks | Section::Sources);
        if !in_section {
            continue;
        }

        let mut row = trimmed.replace('*', "").trim().to_string();
        row = row.trim_start_matches('│').trim().to_string();

        let Some(dot) = row.find('.') else { continue };

        let id_str = row[..dot].trim();
        let id: i32 = match id_str.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let rest = row[dot + 1..].trim();

        let label = match rest.rfind(" [vol:") {
            Some(idx) => rest[..idx].trim(),
            None => rest,
        }
        .to_string();

        match section {
            Section::Sinks => sinks.push((id, label)),
            Section::Sources => sources.push((id, label)),
            Section::None => {}
        }
    }

    (sinks, sources)
}

pub fn wpctl_inspect(id: i32) -> Result<String, String> {
    let out = Command::new("wpctl")
        .args(["inspect", &id.to_string()])
        .output()
        .map_err(|e| format!("Failed to run wpctl inspect {}: {}", id, e))?;

    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).to_string())
    }
}

// Parse node.name = "..." from wpctl inspect output
pub fn parse_node_name_from_inspect(text: &str) -> Option<String> {
    for line in text.lines() {
        // common formats seen in inspect:
        //  node.name = "alsa_input.usb-...."
        //  node.name = alsa_input.usb-....   (sometimes unquoted)
        let t = line.trim();
        if t.starts_with("node.name") {
            if let Some(eq) = t.find('=') {
                let v = t[eq + 1..].trim().trim_matches('"').to_string();
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }
    }
    None
}

// Run `wpctl status --name` and find ID for a given node.name
pub fn resolve_id_by_node_name(node_name: &str) -> Result<Option<i32>, String> {
    let out = Command::new("wpctl")
        .args(["status", "--name"])
        .output()
        .map_err(|e| format!("Failed to run wpctl status --name: {}", e))?;

    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }

    let text = String::from_utf8_lossy(&out.stdout).to_string();

    // We’ll match rows like:
    // "│  * 104. alsa_input.usb-0d8c_C-Media_USB_Audio_Device-00.mono-fallback [vol: ...]"
    for line in text.lines() {
        let mut row = line.trim().replace('*', "");
        row = row.trim_start_matches('│').trim().to_string();

        let Some(dot) = row.find('.') else { continue };
        let id_str = row[..dot].trim();
        let id: i32 = match id_str.parse() { Ok(v) => v, Err(_) => continue };
        let rest = row[dot + 1..].trim();

        // rest starts with the node name in --name mode
        // stop at first whitespace or " [vol:"
        let name_part = rest
            .split(" [vol:")
            .next()
            .unwrap_or(rest)
            .trim()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim();

        if name_part == node_name {
            return Ok(Some(id));
        }
    }

    Ok(None)
}
