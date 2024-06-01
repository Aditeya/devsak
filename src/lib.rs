//! DevSAK

pub mod base64;

pub trait Tool {
    fn set_input(&mut self, input: &str);
    fn get_input(&self) -> &str;
    fn get_output(&self) -> String;
    fn update_output(&mut self) -> String;
}

#[cfg(test)]
mod test {
    use crate::{
        base64::{Base64Decoder, Base64Encoder},
        Tool,
    };

    #[test]
    fn test_multiple_tools() {
        let mut tools: Vec<Box<dyn Tool>> = vec![
            Box::new(Base64Encoder::new()),
            Box::new(Base64Decoder::new()),
            Box::new(Base64Encoder::new()),
            Box::new(Base64Decoder::new()),
            Box::new(Base64Encoder::new()),
            Box::new(Base64Decoder::new()),
        ];

        if let Some(i) = tools.first_mut() {
            i.set_input("test")
        }

        let output = tools
            .iter_mut()
            .fold(String::from("test"), |mut prev_output, curr_tool| {
                curr_tool.set_input(&prev_output);
                prev_output = curr_tool.update_output();
                prev_output
            });

        assert_eq!("test", &output);
        assert_eq!(
            "test",
            &tools.last_mut().map(|i| i.update_output()).unwrap()
        );
        assert_eq!("test", &tools.last_mut().map(|i| i.get_output()).unwrap());
    }
}
