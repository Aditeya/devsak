use base64::Engine;

use crate::Tool;

#[derive(Debug, Default, Clone)]
pub struct Base64Decoder {
    input: Option<String>,
    output: Option<String>,
}

impl Base64Decoder {
    pub fn new() -> Self {
        Self {
            input: None,
            output: None,
        }
    }
}

impl Tool for Base64Decoder {
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
            let decode = base64::engine::general_purpose::STANDARD
                .decode(i)
                .ok()
                .as_deref()
                .map(String::from_utf8_lossy)
                .map(String::from);
            self.output = decode;
        }

        self.get_output()
    }
}
