/// MIDI message type.
#[derive(Clone, Copy, Debug)]
pub enum MidiMessage {
    /// Control change: channel, control number, value.
    ControlChange(u8, u8, u8),

    /// Note off: channel, note number, velocity.
    NoteOff(u8, u8, u8),

    /// Note on: channel, note number, velocity.
    NoteOn(u8, u8, u8),
}
impl MidiMessage {
    /// Parses raw bytes to a MIDI message, returns [None] if unsupported.
    pub fn from_bytes(raw_bytes: &[u8]) -> Option<MidiMessage> {
        if let [status, data1, data2] = raw_bytes {
            match status & 0xF0 {
                0x80 => Some(MidiMessage::NoteOff(status & 0x0F, *data1, *data2)),
                0x90 => Some(MidiMessage::NoteOn(status & 0x0F, *data1, *data2)),
                0xB0 => Some(MidiMessage::ControlChange(status & 0x0F, *data1, *data2)),
                _ => None,
            }
        } else {
            None
        }
    }
}
