mod backend;
mod config;
mod wpctl;

use backend::AudioToggler;
use qmetaobject::{QmlEngine, QObjectBox};
use std::path::PathBuf;

fn qml_dir() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| format!("current_exe failed: {e}"))?;
    let dir = exe
        .parent()
        .ok_or_else(|| "No parent dir for executable".to_string())?;
    Ok(dir.join("qml"))
}

fn main() {
    let mut engine = QmlEngine::new();

    // Backend object exposed to QML as `audio`
    let backend_box = QObjectBox::new(AudioToggler::default());
    let backend = backend_box.pinned();
    {
        let mut b = backend.borrow_mut();
        b.load_config_internal();
        b.refresh_devices_internal();
    }
    engine.set_object_property("audio".into(), backend);

    // Resolve paths relative to the executable
    let qml_dir = match qml_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("QML dir resolution failed: {e}");
            std::process::exit(1);
        }
    };

    let main_qml = qml_dir.join("main.qml");

    if !main_qml.exists() {
        eprintln!("ERROR: main.qml not found at {}", main_qml.display());
        eprintln!("Expected layout: <exe_dir>/qml/main.qml and <exe_dir>/qml/TrayIcon.qml");
        std::process::exit(1);
    }

    // IMPORTANT: allow QML to find TrayIcon.qml and other local components
    engine.add_import_path(qml_dir.to_string_lossy().to_string().into());

    // Load main QML
    engine.load_file(main_qml.to_string_lossy().to_string().into());

    engine.exec();

    drop(backend_box);
}
