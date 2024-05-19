//! DevSAK

mod base64_converter;

use base64::Engine;

pub trait Tool {
    fn set_input(&mut self, input: &str);
    fn get_input(&self) -> &str;
    fn get_output(&self) -> String;
    fn update_output(&mut self) -> String;
}
