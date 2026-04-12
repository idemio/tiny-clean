use crate::java_script_encoder::{JavaScriptEncoder, JavaScriptEncoderMode};
use crate::uri_encoder::{UriEncoder, UriEncoderMode};
use crate::xml_encoder::{XmlEncoder, XmlEncoderMode};

pub struct NoBuilder;
pub struct JavaScriptEncoderBuilder {
    ascii_only: bool,
    mode: JavaScriptEncoderMode,
}
pub struct XmlEncoderBuilder {
    mode: XmlEncoderMode,
}

pub struct UriEncoderBuilder {
    mode: UriEncoderMode,
}

pub struct EncoderBuilder<S> {
    state: S,
}

impl EncoderBuilder<NoBuilder> {
    pub fn new() -> Self {
        EncoderBuilder { state: NoBuilder }
    }

    pub fn java_script(self) -> EncoderBuilder<JavaScriptEncoderBuilder> {
        EncoderBuilder {
            state: JavaScriptEncoderBuilder {
                ascii_only: false,
                mode: JavaScriptEncoderMode::Block,
            },
        }
    }

    pub fn xml(self) -> EncoderBuilder<XmlEncoderBuilder> {
        EncoderBuilder {
            state: XmlEncoderBuilder {
                mode: XmlEncoderMode::All,
            },
        }
    }

    pub fn uri(self) -> EncoderBuilder<UriEncoderBuilder> {
        EncoderBuilder {
            state: UriEncoderBuilder {
                mode: UriEncoderMode::FullUri,
            },
        }
    }
}

impl EncoderBuilder<JavaScriptEncoderBuilder> {
    pub fn with_ascii_only(self, ascii_only: bool) -> Self {
        let mut state = self.state;
        state.ascii_only = ascii_only;
        Self { state }
    }

    pub fn with_encoder_mode(self, mode: JavaScriptEncoderMode) -> Self {
        let mut state = self.state;
        state.mode = mode;
        Self { state }
    }

    pub fn build(self) -> JavaScriptEncoder {
        JavaScriptEncoder::new(self.state.mode, self.state.ascii_only)
    }
}

impl EncoderBuilder<XmlEncoderBuilder> {
    pub fn with_mode(self, mode: XmlEncoderMode) -> Self {
        let mut state = self.state;
        state.mode = mode;
        Self { state }
    }

    pub fn build(self) -> XmlEncoder {
        XmlEncoder::new(self.state.mode)
    }
}

impl EncoderBuilder<UriEncoderBuilder> {
    pub fn with_mode(self, mode: UriEncoderMode) -> Self {
        let mut state = self.state;
        state.mode = mode;
        Self { state }
    }

    pub fn build(self) -> UriEncoder {
        UriEncoder::new(self.state.mode)
    }
}

#[cfg(test)]
mod test {
    use crate::builder::EncoderBuilder;
    use crate::java_script_encoder::JavaScriptEncoderMode;
    use crate::java_script_encoder::JavaScriptEncoderMode::Attribute;
    use crate::uri_encoder::UriEncoderMode;
    use crate::xml_encoder::XmlEncoderMode;

    #[test]
    fn test_build_java_script_encoder() {
        let encoder = EncoderBuilder::new()
            .java_script()
            .with_encoder_mode(JavaScriptEncoderMode::Attribute)
            .with_ascii_only(true);

        assert_eq!(true, encoder.state.ascii_only);
        assert_eq!(Attribute, encoder.state.mode);

        let encoder = encoder.build();

        assert_eq!(true, encoder.ascii_only);
        assert_eq!(true, encoder.hex_encode_quotes);
    }

    #[test]
    fn test_build_xml_encoder() {
        let encoder = EncoderBuilder::new()
            .xml()
            .with_mode(XmlEncoderMode::Content);

        assert_eq!(XmlEncoderMode::Content, encoder.state.mode);
    }

    #[test]
    fn test_build_uri_encoder() {
        let encoder = EncoderBuilder::new().uri().with_mode(UriEncoderMode::Component);

        assert_eq!(UriEncoderMode::Component, encoder.state.mode);
    }
}
