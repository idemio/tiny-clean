# Tiny Clean
A light-weight high-performance string sanitizer with customizable rules. 
Prevent xss by dropping in a simple utility to clean your input.

## Features
- Use built in configurations to sanitize HTML, JavaScript, XML, and more. 
- Allows for custom rules to define your own encoders.

## Usage
```rust
use tiny_clean::encoder::{Encoder, ValidAsciiRange};
use tiny_clean::java_script_encoder::{
    JavaScriptEncoder, JavaScriptEncoderMode, JavaScriptEncoderSettings,
};

let my_unsafe_data = r#"..."#

// Javascript 'Block' Encoding
let encoder = JavaScriptEncoder::create(
    '\\',
    ValidAsciiRange::ASCII,
    JavaScriptEncoderSettings {
        mode: JavaScriptEncoderMode::Block,
    },
);

let my_clean_data = encoder.encode(&my_unsafe_data);


```

## Performance