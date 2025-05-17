# Tiny Clean
A light-weight high-performance string sanitizer with customizable rules. 
Prevent xss by dropping in a simple utility to clean your input.

## Features
- Use built in configurations to sanitize HTML, JavaScript, XML, and more. 
- Allows for custom rules to define your own encoders.

## Usage
```Rust
    use tiny_clean::encoder::{Encoder, ValidAsciiRange};
    use tiny_clean::java_script_encoder::{
        JavaScriptEncoder, JavaScriptEncoderMode,
    };
    
    let my_unsafe_data = r#"..."#
    
    // Javascript 'Block' Encoding
    let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Block, true);
    
    let my_clean_data = encoder.encode(&my_unsafe_data);


```

## Performance
The following benchmarks were run on a machine with the following specs:
- CPU: Intel Core i9-10850K @ 4.91GHz
- Rust: 1.85.1

### JavaScript Encoder
| Encoder Settings          | Data Set   | Time Range (µs) |
|---------------------------|------------|-----------------|
| Block, ASCII Only         | Data Set 1 | 72.064 - 72.357 |
| Block, ASCII Extended     | Data Set 1 | 72.348 - 72.696 |
| Block, ASCII Only         | Data Set 2 | 241.73 - 243.53 |
| Block, ASCII Extended     | Data Set 2 | 239.18 - 239.91 |
| Source, ASCII Only        | Data Set 1 | 66.790 - 67.023 |
| Source, ASCII Extended    | Data Set 1 | 67.016 - 67.246 |
| Source, ASCII Only        | Data Set 2 | 217.54 - 218.34 |
| Source, ASCII Extended    | Data Set 2 | 215.42 - 216.38 |
| Html, ASCII Only          | Data Set 1 | 74.583 - 75.096 |
| Html, ASCII Extended      | Data Set 1 | 74.237 - 74.447 |
| Html, ASCII Only          | Data Set 2 | 242.57 - 244.40 |
| Html, ASCII Extended      | Data Set 2 | 242.43 - 243.39 |
| Attribute, ASCII Only     | Data Set 1 | 69.931 - 70.145 |
| Attribute, ASCII Extended | Data Set 1 | 69.824 - 70.015 |
| Attribute, ASCII Only     | Data Set 2 | 221.90 - 222.51 |
| Attribute, ASCII Extended | Data Set 2 | 222.91 - 223.64 |

### Uri Encoder

| Encoder Settings | Data Set   | Time Range (µs) |
|------------------|------------|-----------------|
| FullUri          | Data Set 1 | 122.17 - 122.52 |
| FullUri          | Data Set 2 | 362.05 - 365.21 |
| Component        | Data Set 1 | 137.35 - 137.67 |
| Component        | Data Set 2 | 372.54 - 374.13 |

### Xml Encoder

| Encoder Settings      | Data Set   | Time Range (µs) |
|-----------------------|------------|-----------------|
| Attribute             | Data Set 1 | 138.35 - 139.26 |
| Attribute             | Data Set 2 | 298.04 - 299.57 |
| All                   | Data Set 1 | 138.99 - 139.79 |
| All                   | Data Set 2 | 300.50 - 301.73 |
| Content               | Data Set 1 | 135.55 - 135.95 |
| Content               | Data Set 2 | 295.20 - 296.15 |
| SingleQuotedAttribute | Data Set 1 | 137.40 - 138.60 |
| SingleQuotedAttribute | Data Set 2 | 293.27 - 294.44 |
| DoubleQuotedAttribute | Data Set 1 | 137.19 - 137.76 |
| DoubleQuotedAttribute | Data Set 2 | 296.32 - 297.35 |
