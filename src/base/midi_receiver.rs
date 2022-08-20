use crate::base::MidiMessage;

/// Defines a generic MIDI receiver, e.g. an instrument or effect that can process MIDI.
pub trait MidiReceiver: Send {
    /// Handles a MIDI message.
    fn handle_midi_message(&mut self, message: MidiMessage);
}
