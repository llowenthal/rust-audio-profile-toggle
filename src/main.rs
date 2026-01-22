mod backend;
mod config;
mod wpctl;

use qmetaobject::prelude::*;
use backend::AudioToggler;

fn main() {
    let mut engine = QmlEngine::new();

    // Backend object exposed to QML as `audio`
    let backend_box = QObjectBox::new(AudioToggler::default());
    let backend = backend_box.pinned();

    // Initialize
    {
        let mut b = backend.borrow_mut();
        b.load_config_internal();
        b.refresh_devices_internal();
    }

    engine.set_object_property("audio".into(), backend);
    engine.load_file("qml/main.qml".into());
    engine.exec();

    // Keep backend_box alive until engine exits
    drop(backend_box);
}
