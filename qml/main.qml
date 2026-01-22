import QtQuick 2.6
import QtQuick.Window 2.0
import QtQuick.Controls 2.0

Window {
    id: win
    visible: true
    width: 820
    height: 520
    title: "Audio Profile Toggle - Settings"

    SystemPalette { id: pal }
    color: pal.window

    // Tray is a separate component file
    TrayIcon {
        id: tray
        mainWindow: win
    }

    // Intercept close: hide instead of exit
    onClosing: function(close) {
        close.accepted = false
        win.visible = false
    }

    Component.onCompleted: {
        if (audio) {
            audio.refresh_devices()
        }
    }

    Column {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        // Top buttons
        Row {
            spacing: 12
            Button { text: "Refresh Devices"; onClicked: audio && audio.refresh_devices() }
            Button { text: "Load Config"; onClicked: audio && audio.load_config() }
            Button { text: "Save Config"; onClicked: audio && audio.save_config() }
            Button { text: "Toggle Now (A ↔ B)"; onClicked: audio && audio.toggle() }
        }

        Rectangle { width: parent.width; height: 1; opacity: 0.25 }

        Row {
            spacing: 24
            width: parent.width
            height: parent.height - 150

            // -------- Profile A --------
            GroupBox {
                title: "Profile A"
                width: parent.width/2 - 12
                height: parent.height

                Column {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 10

                    Text {
                        text: "Output (Sink)"
                        color: pal.windowText
                    }
                    ComboBox {
                        id: aSink
                        width: parent.width
                        model: audio ? audio.sinks : []
                        onActivated: {
                            var id = parseInt(currentText.split(":")[0])
                            var label = currentText.substring(currentText.indexOf(":") + 1).trim()
                            if (audio && !isNaN(id)) {
                                audio.a_sink_id = id
                                audio.a_sink_label = label
                                audio.a_sink_node_name = audio.get_node_name(id)
                            }
                        }

                        function sync() {
                            if (!audio) return
                            for (var i=0; i<count; i++) {
                                var id = parseInt(textAt(i).split(":")[0])
                                if (id === audio.a_sink_id) { currentIndex = i; break; }
                            }
                        }
                        Component.onCompleted: sync()
                        onModelChanged: sync()
                        Connections { target: audio; function onA_changed(){ aSink.sync() } }
                    }

                    Text { 
                        text: "Input (Source)"
                        color: pal.windowText
                     }
                    ComboBox {
                        id: aSource
                        width: parent.width
                        model: audio ? audio.sources : []
                        onActivated: {
                            var id = parseInt(currentText.split(":")[0])
                            var label = currentText.substring(currentText.indexOf(":") + 1).trim()
                            if (audio && !isNaN(id)) {
                                audio.a_source_id = id
                                audio.a_source_label = label
                                audio.a_source_node_name = audio.get_node_name(id)
                            }
                        }

                        function sync() {
                            if (!audio) return
                            for (var i=0; i<count; i++) {
                                var id = parseInt(textAt(i).split(":")[0])
                                if (id === audio.a_source_id) { currentIndex = i; break; }
                            }
                        }
                        Component.onCompleted: sync()
                        onModelChanged: sync()
                        Connections { target: audio; function onA_changed(){ aSource.sync() } }
                    }

                    Text {
                        text: "Sink Volume: " + (audio ? audio.a_sink_volume.toFixed(2) : "")
                        color: pal.windowText    
                    }
                    Slider {
                        id: aSinkVol
                        from: 0.0; to: 1.0; stepSize: 0.01
                        value: 1.0
                        onMoved: if (audio) audio.a_sink_volume = value
                        Component.onCompleted: if (audio) value = audio.a_sink_volume
                        Connections { target: audio; function onA_changed(){ if (audio) aSinkVol.value = audio.a_sink_volume } }
                        width: parent.width
                    }

                    Text {
                        text: "Source Volume: " + (audio ? audio.a_source_volume.toFixed(2) : "")
                        color: pal.windowText
                    }
                    Slider {
                        id: aSourceVol
                        from: 0.0; to: 1.0; stepSize: 0.01
                        value: 1.0
                        onMoved: if (audio) audio.a_source_volume = value
                        Component.onCompleted: if (audio) value = audio.a_source_volume
                        Connections { target: audio; function onA_changed(){ if (audio) aSourceVol.value = audio.a_source_volume } }
                        width: parent.width
                    }

                    Row {
                        spacing: 10
                        Button { text: "Apply A Now"; onClicked: audio && audio.apply_a_now() }
                    }
                }
            }

            // -------- Profile B --------
            GroupBox {
                title: "Profile B"
                width: parent.width/2 - 12
                height: parent.height

                Column {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 10

                    Text {
                        text: "Output (Sink)"
                        color: pal.windowText
                    }
                    ComboBox {
                        id: bSink
                        width: parent.width
                        model: audio ? audio.sinks : []
                        onActivated: {
                            var id = parseInt(currentText.split(":")[0])
                            var label = currentText.substring(currentText.indexOf(":") + 1).trim()
                            if (audio && !isNaN(id)) {
                                audio.b_sink_id = id
                                audio.b_sink_label = label
                                audio.b_sink_node_name = audio.get_node_name(id)
                            }
                        }


                        function sync() {
                            if (!audio) return
                            for (var i=0; i<count; i++) {
                                var id = parseInt(textAt(i).split(":")[0])
                                if (id === audio.b_sink_id) { currentIndex = i; break; }
                            }
                        }
                        Component.onCompleted: sync()
                        onModelChanged: sync()
                        Connections { target: audio; function onB_changed(){ bSink.sync() } }
                    }

                    Text {
                        text: "Input (Source)"
                        color: pal.windowText
                    }
                    ComboBox {
                        id: bSource
                        width: parent.width
                        model: audio ? audio.sources : []
                        onActivated: {
                            var id = parseInt(currentText.split(":")[0])
                            var label = currentText.substring(currentText.indexOf(":") + 1).trim()
                            if (audio && !isNaN(id)) {
                                audio.b_source_id = id
                                audio.b_source_label = label
                                audio.b_source_node_name = audio.get_node_name(id)
                            }
                        }


                        function sync() {
                            if (!audio) return
                            for (var i=0; i<count; i++) {
                                var id = parseInt(textAt(i).split(":")[0])
                                if (id === audio.b_source_id) { currentIndex = i; break; }
                            }
                        }
                        Component.onCompleted: sync()
                        onModelChanged: sync()
                        Connections { target: audio; function onB_changed(){ bSource.sync() } }
                    }

                    Text {
                        text: "Sink Volume: " + (audio ? audio.b_sink_volume.toFixed(2) : "")
                        color: pal.windowText
                    }
                    Slider {
                        id: bSinkVol
                        from: 0.0; to: 1.0; stepSize: 0.01
                        value: 1.0
                        onMoved: if (audio) audio.b_sink_volume = value
                        Component.onCompleted: if (audio) value = audio.b_sink_volume
                        Connections { target: audio; function onB_changed(){ if (audio) bSinkVol.value = audio.b_sink_volume } }
                        width: parent.width
                    }

                    Text {
                        text: "Source Volume: " + (audio ? audio.b_source_volume.toFixed(2) : "")
                        color: pal.windowText
                    }
                    Slider {
                        id: bSourceVol
                        from: 0.0; to: 1.0; stepSize: 0.01
                        value: 1.0
                        onMoved: if (audio) audio.b_source_volume = value
                        Component.onCompleted: if (audio) value = audio.b_source_volume
                        Connections { target: audio; function onB_changed(){ if (audio) bSourceVol.value = audio.b_source_volume } }
                        width: parent.width
                    }

                    Row {
                        spacing: 10
                        Button { text: "Apply B Now"; onClicked: audio && audio.apply_b_now() }
                    }
                }
            }
        }

        Rectangle { width: parent.width; height: 1; opacity: 0.25 }

        // Status/Error line
        Text {
            width: parent.width
            wrapMode: Text.Wrap
            text: audio && audio.last_error && audio.last_error.length > 0
                  ? ("Error: " + audio.last_error)
                  : "(Close window to minimize to tray)"
            color: pal.windowText
        }
    }
}
