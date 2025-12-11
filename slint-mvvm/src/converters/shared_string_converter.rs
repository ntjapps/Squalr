use slint::SharedString;

/// Converter for SharedString types
pub struct SharedStringConverter {}

impl SharedStringConverter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_shared_string(value: &str) -> SharedString {
        SharedString::from(value)
    }

    pub fn from_shared_string(value: &SharedString) -> String {
        value.to_string()
    }
}
