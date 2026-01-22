import QtQuick 2.6
import Qt.labs.platform 1.1

SystemTrayIcon {
    id: tray
    visible: true

    // KDE theme icon. If blank, swap to "preferences-system" or use icon.source
    icon.name: "audio-volume-high"

    tooltip: "Audio Toggle (Profile " + (audio ? audio.current_profile : "?") + ")"

    // provided by parent
    property var mainWindow: null

    menu: Menu {
        MenuItem {
            text: "Toggle Profile (A ↔ B)"
            onTriggered: audio && audio.toggle()
        }

        MenuSeparator {}

        MenuItem { text: "Apply Profile A now"; onTriggered: audio && audio.apply_a_now() }
        MenuItem { text: "Apply Profile B now"; onTriggered: audio && audio.apply_b_now() }

        MenuSeparator {}

        MenuItem {
            text: "Show Settings"
            onTriggered: {
                if (mainWindow) {
                    mainWindow.visible = true
                    mainWindow.raise()
                    mainWindow.requestActivate()
                }
            }
        }

        MenuSeparator {}

        MenuItem { text: "Quit"; onTriggered: audio && audio.quit() }
    }
}
