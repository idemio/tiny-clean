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
The following benchmarks were run on a machine with the following specs:
- CPU: Intel Core i9-10850K @ 4.91GHz
- Rust: 1.85.1

| **Encoder Type**        | **Dataset** | **Restriction** | **Time**                          |
|-------------------------|-------------|-----------------|-----------------------------------|
| **JavaScript - Block**  | DS1         | ASCII Only      | [83.311 µs, 83.408 µs, 83.522 µs] |
| **JavaScript - Block**  | DS1         | ASCII Extended  | [97.683 µs, 97.822 µs, 97.991 µs] |
| **JavaScript - Block**  | DS1         | No Restrict     | [92.088 µs, 92.234 µs, 92.401 µs] |
| **JavaScript - Block**  | DS2         | ASCII Only      | [303.65 µs, 304.06 µs, 304.57 µs] |
| **JavaScript - Block**  | DS2         | ASCII Extended  | [347.35 µs, 348.92 µs, 350.71 µs] |
| **JavaScript - Block**  | DS2         | No Restrict     | [338.12 µs, 338.58 µs, 339.10 µs] |
| **JavaScript - Source** | DS1         | ASCII Only      | [76.488 µs, 76.585 µs, 76.704 µs] |
| **JavaScript - Source** | DS1         | ASCII Extended  | [90.622 µs, 90.713 µs, 90.814 µs] |
| **JavaScript - Source** | DS1         | No Restrict     | [83.998 µs, 84.073 µs, 84.167 µs] |
| **JavaScript - Source** | DS2         | ASCII Only      | [285.13 µs, 285.91 µs, 286.82 µs] |
| **JavaScript - Source** | DS2         | ASCII Extended  | [324.21 µs, 324.76 µs, 325.38 µs] |
| **JavaScript - Source** | DS2         | No Restrict     | [309.12 µs, 309.62 µs, 310.23 µs] |
