use base64::Engine;

use crate::Tool;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConvertMode {
    Encode,
    Decode,
}

pub struct Base64Converter {
    mode: ConvertMode,
    input: Option<String>,
    output: Option<String>,
}

impl Base64Converter {
    fn new() -> Self {
        Self {
            mode: ConvertMode::Encode,
            input: None,
            output: None,
        }
    }

    fn get_mode(&self) -> ConvertMode {
        self.mode
    }

    fn set_mode(&mut self, mode: ConvertMode) {
        self.mode = mode;
    }
}

impl Tool for Base64Converter {
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
            match self.mode {
                ConvertMode::Encode => {
                    let encoded = base64::engine::general_purpose::STANDARD.encode(i);
                    self.output = Some(encoded);
                }
                ConvertMode::Decode => {
                    let decoded = base64::engine::general_purpose::STANDARD
                        .decode(i)
                        .ok()
                        .map(|i| String::from_utf8_lossy(&i).to_string());
                    self.output = decoded;
                }
            }
        }

        self.get_output()
    }
}

#[cfg(test)]
mod test {
    use crate::{base64_converter::{Base64Converter, ConvertMode}, Tool};

    #[test]
    fn test_b64() {
        let mut converter = Base64Converter::new();
        assert_eq!("", converter.get_input());
        assert_eq!("", converter.get_output());

        converter.set_input("test");
        assert_eq!("test", converter.get_input());
        assert_eq!("", converter.get_output());

        assert_eq!("dGVzdA==", converter.update_output());
        assert_eq!("dGVzdA==", converter.get_output());

        converter.set_mode(ConvertMode::Decode);
        assert_eq!(ConvertMode::Decode, converter.get_mode());

        converter.set_input("dGVzdA==");
        assert_eq!("dGVzdA==", converter.get_input());

        assert_eq!("test", converter.update_output());
        assert_eq!("test", converter.get_output());
    }
}
