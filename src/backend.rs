use qmetaobject::prelude::*;
use qmetaobject::QStringList;
use qmetaobject::qtcore::core_application::QCoreApplication;
use std::{thread, time::Duration};

use crate::config::{AppConfig, ProfileConfig};
use crate::config::{load_config_file, save_config_file};
use crate::wpctl::{get_wpctl_status, parse_wpctl_status_for_devices, run_wpctl};

fn apply_profile_resolving_ids(p: &ProfileConfig) -> Result<(), String> {
    let status = get_wpctl_status().map_err(|e| format!("wpctl status failed: {}", e))?;
    let (sinks, sources) = parse_wpctl_status_for_devices(&status);

    let sink_id_live = resolve_id(p.sink_id, &p.sink_label, &sinks)
        .ok_or_else(|| format!("Could not resolve sink '{}'", p.sink_label))?;

    let source_id_live = resolve_id(p.source_id, &p.source_label, &sources)
        .ok_or_else(|| format!("Could not resolve source '{}'", p.source_label))?;

    // Apply defaults
    run_wpctl(&["set-default", &sink_id_live.to_string()])?;
    run_wpctl(&["set-default", &source_id_live.to_string()])?;

    thread::sleep(Duration::from_millis(1000)); // Required sleep here or set-volume doesn't work?

    run_wpctl(&["set-volume", &sink_id_live.to_string(), &format!("{}", p.sink_volume)])?;
    run_wpctl(&["set-volume", &source_id_live.to_string(), &format!("{}", p.source_volume)])?;

    Ok(())
}


    


fn apply_profile_using_node_names(p: &ProfileConfig) -> Result<(), String> {
    // Resolve sink/source IDs by node.name if possible
    let sink_id = if !p.sink_node_name.is_empty() {
        crate::wpctl::resolve_id_by_node_name(&p.sink_node_name)?
            .ok_or_else(|| format!("Sink node not found: {}", p.sink_node_name))?
    } else {
        p.sink_id
    };

    let source_id = if !p.source_node_name.is_empty() {
        crate::wpctl::resolve_id_by_node_name(&p.source_node_name)?
            .ok_or_else(|| format!("Source node not found: {}", p.source_node_name))?
    } else {
        p.source_id
    };

    run_wpctl(&["set-default", &sink_id.to_string()])?;
    run_wpctl(&["set-default", &source_id.to_string()])?;

    run_wpctl(&["set-volume", "@DEFAULT_AUDIO_SINK@", &format!("{}", p.sink_volume)])?;
    run_wpctl(&["set-volume", "@DEFAULT_AUDIO_SOURCE@", &format!("{}", p.source_volume)])?;
    Ok(())
}


// Prefer ID if it exists; else fallback to label match (exact; then "contains" fallback)
fn resolve_id(saved_id: i32, saved_label: &str, live: &[(i32, String)]) -> Option<i32> {
    // exact ID match
    if live.iter().any(|(id, _)| *id == saved_id) {
        return Some(saved_id);
    }

    // exact label match
    if let Some((id, _)) = live.iter().find(|(_, label)| label == saved_label) {
        return Some(*id);
    }

    // contains fallback (helps if label changes slightly, e.g. "Mono" suffix)
    let needle = saved_label.to_lowercase();
    if let Some((id, _)) = live
        .iter()
        .find(|(_, label)| label.to_lowercase().contains(&needle))
    {
        return Some(*id);
    }

    None
}

#[derive(QObject, Default)]
pub struct AudioToggler {
    base: qt_base_class!(trait QObject),

    // device lists for dropdowns
    sinks: qt_property!(QStringList; NOTIFY sinks_changed),
    sinks_changed: qt_signal!(),

    sources: qt_property!(QStringList; NOTIFY sources_changed),
    sources_changed: qt_signal!(),

    // Profile A (ids + labels)
    a_sink_id: qt_property!(i32; NOTIFY a_changed),
    a_sink_label: qt_property!(QString; NOTIFY a_changed),
    a_source_id: qt_property!(i32; NOTIFY a_changed),
    a_source_label: qt_property!(QString; NOTIFY a_changed),
    a_sink_volume: qt_property!(f32; NOTIFY a_changed),
    a_source_volume: qt_property!(f32; NOTIFY a_changed),
    a_changed: qt_signal!(),
    a_sink_node_name: qt_property!(QString; NOTIFY a_changed),
    a_source_node_name: qt_property!(QString; NOTIFY a_changed),

    // Profile B (ids + labels)
    b_sink_id: qt_property!(i32; NOTIFY b_changed),
    b_sink_label: qt_property!(QString; NOTIFY b_changed),
    b_source_id: qt_property!(i32; NOTIFY b_changed),
    b_source_label: qt_property!(QString; NOTIFY b_changed),
    b_sink_volume: qt_property!(f32; NOTIFY b_changed),
    b_source_volume: qt_property!(f32; NOTIFY b_changed),
    b_changed: qt_signal!(),
    b_sink_node_name: qt_property!(QString; NOTIFY b_changed),
    b_source_node_name: qt_property!(QString; NOTIFY b_changed),    

    // state + errors
    current_profile: qt_property!(QString; NOTIFY current_profile_changed),
    current_profile_changed: qt_signal!(),

    last_error: qt_property!(QString; NOTIFY last_error_changed),
    last_error_changed: qt_signal!(),

    // ---- QML methods ----
    refresh_devices: qt_method!(fn refresh_devices(&mut self) {
        self.refresh_devices_internal();
    }),

    load_config: qt_method!(fn load_config(&mut self) {
        self.load_config_internal();
    }),

    save_config: qt_method!(fn save_config(&mut self) {
        let cfg = self.properties_to_config();
        match save_config_file(&cfg) {
            Ok(_) => self.set_error(String::new()),
            Err(e) => self.set_error(e),
        }
    }),

    toggle: qt_method!(fn toggle(&mut self) {
        let mut cfg = self.properties_to_config();
        let next = if cfg.current_profile == "B" { "A" } else { "B" };
        cfg.current_profile = next.to_string();
        self.apply_config_to_properties(&cfg);

        let prof_ref: &ProfileConfig = if next == "A" { &cfg.profile_a } else { &cfg.profile_b };

        match apply_profile_resolving_ids(prof_ref) {
            Ok(_) => {
                let _ = save_config_file(&cfg);
                self.set_error(String::new());
            }
            Err(e) => self.set_error(e),
        }
    }),

    apply_a_now: qt_method!(fn apply_a_now(&mut self) {
        let cfg = self.properties_to_config();
        match apply_profile_resolving_ids(&cfg.profile_a) {
            Ok(_) => self.set_error(String::new()),
            Err(e) => self.set_error(e),
        }
    }),

    apply_b_now: qt_method!(fn apply_b_now(&mut self) {
        let cfg = self.properties_to_config();
        match apply_profile_resolving_ids(&cfg.profile_b) {
            Ok(_) => self.set_error(String::new()),
            Err(e) => self.set_error(e),
        }
    }),

    get_node_name: qt_method!(fn get_node_name(&mut self, id: i32) -> QString {
        match crate::wpctl::wpctl_inspect(id)
            .ok()
            .and_then(|txt| crate::wpctl::parse_node_name_from_inspect(&txt))
        {
            Some(name) => QString::from(name),
            None => QString::from(""),
        }
    }),

    quit: qt_method!(fn quit(&mut self) {
        QCoreApplication::quit();
    }),
}

impl AudioToggler {
    fn set_error(&mut self, msg: String) {
        self.last_error = QString::from(msg);
        self.last_error_changed();
    }

    fn apply_config_to_properties(&mut self, cfg: &AppConfig) {
        self.a_sink_id = cfg.profile_a.sink_id;
        self.a_sink_label = QString::from(cfg.profile_a.sink_label.clone());
        self.a_source_id = cfg.profile_a.source_id;
        self.a_source_label = QString::from(cfg.profile_a.source_label.clone());
        self.a_sink_volume = cfg.profile_a.sink_volume;
        self.a_source_volume = cfg.profile_a.source_volume;
        self.a_sink_node_name = QString::from(cfg.profile_a.sink_node_name.clone());
        self.a_source_node_name = QString::from(cfg.profile_a.source_node_name.clone());
        self.a_changed();

        self.b_sink_id = cfg.profile_b.sink_id;
        self.b_sink_label = QString::from(cfg.profile_b.sink_label.clone());
        self.b_source_id = cfg.profile_b.source_id;
        self.b_source_label = QString::from(cfg.profile_b.source_label.clone());
        self.b_sink_volume = cfg.profile_b.sink_volume;
        self.b_source_volume = cfg.profile_b.source_volume;
        self.b_sink_node_name = QString::from(cfg.profile_b.sink_node_name.clone());
        self.b_source_node_name = QString::from(cfg.profile_b.source_node_name.clone());
        self.b_changed();

        self.current_profile = QString::from(cfg.current_profile.clone());
        self.current_profile_changed();
    }

    fn properties_to_config(&self) -> AppConfig {
        AppConfig {
            profile_a: ProfileConfig {
                sink_id: self.a_sink_id,
                sink_label: self.a_sink_label.to_string(),
                sink_node_name: self.a_sink_node_name.to_string(),

                source_id: self.a_source_id,
                source_label: self.a_source_label.to_string(),
                source_node_name: self.a_source_node_name.to_string(),

                sink_volume: self.a_sink_volume,
                source_volume: self.a_source_volume,
            },
            profile_b: ProfileConfig {
                sink_id: self.b_sink_id,
                sink_label: self.b_sink_label.to_string(),
                sink_node_name: self.b_sink_node_name.to_string(),

                source_id: self.b_source_id,
                source_label: self.b_source_label.to_string(),
                source_node_name: self.b_source_node_name.to_string(),

                sink_volume: self.b_sink_volume,
                source_volume: self.b_source_volume,
            },
            current_profile: self.current_profile.to_string(),
        }
    }


    pub fn load_config_internal(&mut self) {
        let cfg = load_config_file();
        self.apply_config_to_properties(&cfg);
        self.set_error(String::new());
    }

    pub fn refresh_devices_internal(&mut self) {
        match get_wpctl_status() {
            Ok(text) => {
                let (sinks, sources) = parse_wpctl_status_for_devices(&text);

                let sink_strings: Vec<String> = sinks.iter().map(|(id, label)| format!("{}: {}", id, label)).collect();
                let source_strings: Vec<String> = sources.iter().map(|(id, label)| format!("{}: {}", id, label)).collect();

                self.sinks = QStringList::from(sink_strings);
                self.sources = QStringList::from(source_strings);
                self.sinks_changed();
                self.sources_changed();
            }
            Err(e) => self.set_error(e),
        }
    }

}
