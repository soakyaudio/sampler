use crate::base::{MidiEngine, MidiMessage, MidiReceiver};

/// MIDI engine based on [midir].
pub struct MidirMidiEngine {
    /// MIDI input connections.
    _connections: Vec<midir::MidiInputConnection<()>>,
}
impl MidirMidiEngine {
    /// Creates a new midir MIDI engine, connects receiver to all available MIDI inputs.
    pub fn new(receiver: impl MidiReceiver + Clone + 'static) -> Self {
        // Create MIDI input client for fetching available input ports.
        let scanner = midir::MidiInput::new("sampler_scanner").expect("Failed to create MIDI input client.");
        let ports = scanner.ports();

        // Iterate inputs and open connections.
        let mut connections = Vec::<midir::MidiInputConnection<()>>::new();
        for (i, port) in ports.iter().enumerate() {
            let mut receiver = receiver.clone();
            let port_name = format!("sampler_{}", i);
            let midi_input = midir::MidiInput::new(&port_name).expect("Failed to create MIDI input client.");
            let conn = midi_input
                .connect(
                    port,
                    &port_name,
                    move |_timestamp, bytes, _data| {
                        if let Some(message) = MidiMessage::from_bytes(bytes) {
                            receiver.handle_midi_message(message);
                        }
                    },
                    (),
                )
                .expect("Failed to open MIDI input connection.");
            connections.push(conn);
        }

        MidirMidiEngine { _connections: connections }
    }
}
impl MidiEngine for MidirMidiEngine {}
