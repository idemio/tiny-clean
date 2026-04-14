use criterion::{criterion_group, criterion_main, Criterion};

use tiny_clean::{
    JavaScriptEncoder, JavaScriptEncoderConfig, JavaScriptEncoderMode, UriEncoder,
    UriEncoderConfig, UriEncoderMode, XmlEncoder, XmlEncoderConfig, XmlEncoderMode,
};

fn xml_encode_benches(c: &mut Criterion) {
    let bench_data1 = std::fs::read_to_string("./benches/data/benchmark-data-1.txt").unwrap();
    let bench_data2 = std::fs::read_to_string("./benches/data/benchmark-data-2.txt").unwrap();
    let encoder = XmlEncoder::new(
        XmlEncoderConfig::builder()
            .mode(XmlEncoderMode::Attribute)
            .build(),
    );
    c.bench_function("XmlEncoder - Attribute - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("XmlEncoder - Attribute - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
    let encoder = XmlEncoder::new(
        XmlEncoderConfig::builder()
            .mode(XmlEncoderMode::All)
            .build(),
    );
    c.bench_function("XmlEncoder - All - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("XmlEncoder - All - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
    let encoder = XmlEncoder::new(
        XmlEncoderConfig::builder()
            .mode(XmlEncoderMode::Content)
            .build(),
    );
    c.bench_function("XmlEncoder - Content - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("XmlEncoder - Content - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
    let encoder = XmlEncoder::new(
        XmlEncoderConfig::builder()
            .mode(XmlEncoderMode::SingleQuotedAttribute)
            .build(),
    );
    c.bench_function("XmlEncoder - SingleQuotedAttribute - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("XmlEncoder - SingleQuotedAttribute - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
    let encoder = XmlEncoder::new(
        XmlEncoderConfig::builder()
            .mode(XmlEncoderMode::DoubleQuotedAttribute)
            .build(),
    );
    c.bench_function("XmlEncoder - DoubleQuotedAttribute - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("XmlEncoder - DoubleQuotedAttribute - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
}

fn uri_encode_benches(c: &mut Criterion) {
    let bench_data1 = std::fs::read_to_string("./benches/data/benchmark-data-1.txt").unwrap();
    let bench_data2 = std::fs::read_to_string("./benches/data/benchmark-data-2.txt").unwrap();
    let encoder = UriEncoder::new(
        UriEncoderConfig::builder()
            .mode(UriEncoderMode::FullUri)
            .build(),
    );
    c.bench_function("UriEncoder - FullUri - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("UriEncoder - FullUri - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
    let encoder = UriEncoder::new(
        UriEncoderConfig::builder()
            .mode(UriEncoderMode::Component)
            .build(),
    );
    c.bench_function("UriEncoder - Component - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("UriEncoder - Component - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });
}

fn java_script_encode_benches(c: &mut Criterion) {
    let bench_data1 = std::fs::read_to_string("./benches/data/benchmark-data-1.txt").unwrap();
    let bench_data2 = std::fs::read_to_string("./benches/data/benchmark-data-2.txt").unwrap();

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Block)
            .ascii_only(true)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Block - ASCII Only - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("JavaScriptEncoder - Block - ASCII Only - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Block)
            .ascii_only(false)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Block - ASCII Extended - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });

    c.bench_function("JavaScriptEncoder - Block - ASCII Extended - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    ///////////// block end

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Source)
            .ascii_only(true)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Source - ASCII Only - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("JavaScriptEncoder - Source - ASCII Only - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Source)
            .ascii_only(false)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Source - ASCII Extended - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("JavaScriptEncoder - Source - ASCII Extended - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    ///////////// source end

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Html)
            .ascii_only(true)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Html - ASCII Only - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("JavaScriptEncoder - Html - ASCII Only - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Html)
            .ascii_only(false)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Html - ASCII Extended - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("JavaScriptEncoder - Html - ASCII Extended - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    ///////////// html end

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Attribute)
            .ascii_only(true)
            .build(),
    );
    c.bench_function("JavaScriptEncoder - Attribute - ASCII Only - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
    c.bench_function("JavaScriptEncoder - Attribute - ASCII Only - DS2", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data2)))
    });

    let encoder = JavaScriptEncoder::new(
        JavaScriptEncoderConfig::builder()
            .mode(JavaScriptEncoderMode::Attribute)
            .ascii_only(false)
            .build(),
    );
    c.bench_function(
        "JavaScriptEncoder - Attribute - ASCII Extended - DS1",
        |b| b.iter(|| encoder.encode(std::hint::black_box(&bench_data1))),
    );
    c.bench_function(
        "JavaScriptEncoder - Attribute - ASCII Extended - DS2",
        |b| b.iter(|| encoder.encode(std::hint::black_box(&bench_data2))),
    );
    ///////////// attribute end
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(60);
    targets = java_script_encode_benches,uri_encode_benches,xml_encode_benches
}
criterion_main!(benches);
