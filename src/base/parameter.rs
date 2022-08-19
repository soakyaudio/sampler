/// Unique identifier of a processor parameter.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ParameterId {
    /// String identifier.
    identifier: String,
}
impl ParameterId {
    /// Creates a new [ParameterId].
    pub fn new(identifier: &str) -> Self {
        ParameterId { identifier: String::from(identifier) }
    }
}

/// Process parameter value of specific type.
#[derive(Debug)]
pub enum ParameterValue {
    /// Float parameter value.
    Float(f32),
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_id_has_identifier() {
        let id = ParameterId::new("reverb");
        assert_eq!(id.identifier, "reverb");
    }

    #[test]
    fn parameter_id_not_equal() {
        let id1 = ParameterId::new("reverb");
        let id2 = ParameterId::new("volume");
        assert_ne!(id1, id2);
    }

    #[test]
    fn parameter_id_is_equal() {
        let id1 = ParameterId::new("reverb");
        let id2 = ParameterId::new("reverb");
        assert_eq!(id1, id2);
    }
}
