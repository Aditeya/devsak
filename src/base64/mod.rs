mod encoder;
pub use encoder::Base64Encoder;

mod decoder;
pub use decoder::Base64Decoder;


#[cfg(test)]
mod test {
    use crate::{
        base64::{Base64Decoder, Base64Encoder},
        Tool,
    };

    #[test]
    fn test_b64_encode() {
        let mut converter = Base64Encoder::new();
        assert_eq!("", converter.get_input());
        assert_eq!("", converter.get_output());

        converter.set_input("test");
        assert_eq!("test", converter.get_input());
        assert_eq!("", converter.get_output());

        assert_eq!("dGVzdA==", converter.update_output());
        assert_eq!("dGVzdA==", converter.get_output());
    }

    #[test]
    fn test_b64_decode() {
        let mut converter = Base64Decoder::new();
        assert_eq!("", converter.get_input());
        assert_eq!("", converter.get_output());

        converter.set_input("dGVzdA==");
        assert_eq!("dGVzdA==", converter.get_input());

        assert_eq!("test", converter.update_output());
        assert_eq!("test", converter.get_output());
    }
    
    #[test]
    fn test_box() {
        let mut tools: Vec<Box<dyn Tool>> = vec![Box::new(Base64Encoder::new()), Box::new(Base64Decoder::new())];

        tools[0].set_input("test");
        let i = tools[0].update_output();
        tools[1].set_input(&i);

        assert_eq!("test", tools[1].update_output());
        assert_eq!("test", tools[1].get_output());
    }
}
