use base64::Engine;

use crate::Tool;

#[derive(Debug, Default, Clone)]
pub struct Base64Encoder {
    input: Option<String>,
    output: Option<String>,
}

impl Base64Encoder {
    pub fn new() -> Self {
        Self {
            input: None,
            output: None,
        }
    }
}

impl Tool for Base64Encoder {
    fn set_input(&mut self, input: &str) {
        self.input = Some(input.to_string());
    }

    fn get_input(&self) -> &str {
        self.input.as_deref().unwrap_or_default()
    }

    fn get_output(&self) -> String {
        self.output.clone().unwrap_or_default()
    }

    fn update_output(&mut self) -> String {
        if let Some(ref i) = self.input {
            let encoded = base64::engine::general_purpose::STANDARD.encode(i);
            self.output = Some(encoded);
        }

        self.get_output()
    }
}
