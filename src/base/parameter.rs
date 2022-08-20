/// Unique identifier of a processor parameter.
pub type ParameterId = u32;

/// Process parameter metadata.
#[derive(Debug)]
pub struct Parameter {
    /// Identifier.
    pub id: ParameterId,

    /// String representation.
    pub name: &'static str,
}
impl Parameter {
    /// Creates a new parameter.
    pub const fn new(id: ParameterId, name: &'static str) -> Self {
        Parameter { id, name }
    }
}

/// Process parameter value of specific type.
#[derive(Clone, Copy, Debug)]
pub enum ParameterValue {
    /// Float parameter value.
    Float(f32),
}
