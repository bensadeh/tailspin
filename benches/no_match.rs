use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tailspin::Highlighter;
use tailspin::config::*;
use tailspin::style::{Color, Style};

const LOG_LINE: &str = r#"INFO  [server::handler] next request - Loss exceeded max. threshold - status n/a - re-run completed 200 OK user=admin "all good""#;

fn bench_no_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("no_match");

    group.bench_function("json", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_json_highlighter(JsonConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("date_time", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_date_time_highlighters(DateTimeConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("url", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_url_highlighter(UrlConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("ip_v4", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_ip_v4_highlighter(IpV4Config::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("ip_v6", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_ip_v6_highlighter(IpV6Config::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("uuid", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_uuid_highlighter(UuidConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("pointer", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_pointer_highlighter(PointerConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("unix_path", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_unix_path_highlighter(UnixPathConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("unix_process", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_unix_process_highlighter(UnixProcessConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("key_value", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_key_value_highlighter(KeyValueConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("number", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_number_highlighter(NumberConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("quote", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_quote_highlighter(QuotesConfig::default());
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("keyword", |b| {
        let h = {
            let mut builder = Highlighter::builder();
            builder.with_keyword_highlighter(vec![
                KeywordConfig {
                    words: vec![
                        "ERROR".into(),
                        "WARN".into(),
                        "INFO".into(),
                        "DEBUG".into(),
                        "TRACE".into(),
                    ],
                    style: Style::new().fg(Color::Red),
                },
                KeywordConfig {
                    words: vec!["GET".into(), "POST".into(), "PUT".into(), "DELETE".into()],
                    style: Style::new().fg(Color::Green),
                },
                KeywordConfig {
                    words: vec!["null".into(), "false".into(), "true".into()],
                    style: Style::new().fg(Color::Yellow),
                },
            ]);
            builder.build().unwrap()
        };
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.finish();
}

criterion_group!(benches, bench_no_match);
criterion_main!(benches);
