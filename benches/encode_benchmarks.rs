use criterion::{Criterion, criterion_group, criterion_main};
use tiny_clean::encoder::{Encoder};
use tiny_clean::java_script_encoder::*;

fn java_script_encode_benches(c: &mut Criterion) {
    let bench_data1 = std::fs::read_to_string("./benches/data/benchmark-data-1.txt").unwrap();
    let bench_data2 = std::fs::read_to_string("./benches/data/benchmark-data-2.txt").unwrap();

    let encoder = JavaScriptEncoder::new(Mode::Block, true);
    c.bench_function("JavaScriptEncoder - Block - ASCII Only - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });

    let encoder = JavaScriptEncoder::new(Mode::Block, false);
    c.bench_function("JavaScriptEncoder - Block - ASCII Extended - DS1", |b| {
        b.iter(|| encoder.encode(std::hint::black_box(&bench_data1)))
    });
//    // Javascript Block Encoding
//
//    c.bench_function("JavaScriptEncoder - Block - No Restrict - DS1", |b| {
//        b.iter(|| encoder3.encode(std::hint::black_box(&bench_data1)))
//    });
//
//    // Javascript Block Encoding
//    let encoder1 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::ASCII,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Block,
//        },
//    );
//
//    c.bench_function("JavaScriptEncoder - Block - ASCII Only - DS2", |b| {
//        b.iter(|| encoder1.encode(std::hint::black_box(&bench_data2)))
//    });
//
//    let encoder2 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::ASCIIExtended,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Block
//        }
//    );
//
//    c.bench_function("JavaScriptEncoder - Block - ASCII Extended - DS2", |b| {
//        b.iter(|| encoder2.encode(std::hint::black_box(&bench_data2)))
//    });
//
//    let encoder3 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::NoRestrict,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Block
//        }
//    );
//
//    c.bench_function("JavaScriptEncoder - Block - No Restrict - DS2", |b| {
//        b.iter(|| encoder3.encode(std::hint::black_box(&bench_data2)))
//    });
//
//    // BLOCK END
//
//
//    // SOURCE START
//    let encoder1 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::ASCII,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Source,
//        },
//    );
//
//    c.bench_function("JavaScriptEncoder - Source - ASCII Only - DS1", |b| {
//        b.iter(|| encoder1.encode(std::hint::black_box(&bench_data1)))
//    });
//
//    let encoder2 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::ASCIIExtended,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Source
//        }
//    );
//
//    c.bench_function("JavaScriptEncoder - Source - ASCII Extended - DS1", |b| {
//        b.iter(|| encoder2.encode(std::hint::black_box(&bench_data1)))
//    });
//
//    let encoder3 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::NoRestrict,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Source
//        }
//    );
//
//    c.bench_function("JavaScriptEncoder - Source - No Restrict - DS1", |b| {
//        b.iter(|| encoder3.encode(std::hint::black_box(&bench_data1)))
//    });
//
//    // Javascript Block Encoding
//    let encoder1 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::ASCII,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Source,
//        },
//    );
//
//    c.bench_function("JavaScriptEncoder - Source - ASCII Only - DS2", |b| {
//        b.iter(|| encoder1.encode(std::hint::black_box(&bench_data2)))
//    });
//
//    let encoder2 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::ASCIIExtended,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Source
//        }
//    );
//
//    c.bench_function("JavaScriptEncoder - Source - ASCII Extended - DS2", |b| {
//        b.iter(|| encoder2.encode(std::hint::black_box(&bench_data2)))
//    });
//
//    let encoder3 = JavaScriptEncoder::create(
//        '\\',
//        ValidAsciiRange::NoRestrict,
//        JavaScriptEncoderSettings {
//            mode: JavaScriptEncoderMode::Source
//        }
//    );
//
//    c.bench_function("JavaScriptEncoder - Source - No Restrict - DS2", |b| {
//        b.iter(|| encoder3.encode(std::hint::black_box(&bench_data2)))
//    });
//    // SOURCE END
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(60);
    targets = java_script_encode_benches
}
criterion_main!(benches);
