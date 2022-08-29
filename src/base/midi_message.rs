/// MIDI message type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes() {
        let note_off = MidiMessage::from_bytes(&[0x81, 0x48, 0x12]);
        assert_eq!(note_off, Some(MidiMessage::NoteOff(0x01, 0x48, 0x12)));

        let note_on = MidiMessage::from_bytes(&[0x9A, 0x52, 0x24]);
        assert_eq!(note_on, Some(MidiMessage::NoteOn(0x0A, 0x52, 0x24)));

        let control_change = MidiMessage::from_bytes(&[0xB3, 0x12, 0x36]);
        assert_eq!(control_change, Some(MidiMessage::ControlChange(0x03, 0x12, 0x36)));

        let invalid = MidiMessage::from_bytes(&[0x01, 0x12, 0x36]);
        assert_eq!(invalid, None);

        let invalid = MidiMessage::from_bytes(&[0x81, 0x48, 0x12, 0x01]);
        assert_eq!(invalid, None);

        let invalid = MidiMessage::from_bytes(&[0x81]);
        assert_eq!(invalid, None);
    }
}
