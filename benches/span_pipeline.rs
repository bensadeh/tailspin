use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tailspin::Highlighter;
use tailspin::config::*;
use tailspin::style::{Color, Style};

const LOG_LINE: &str = r#"2025-03-07T14:32:01.123Z INFO  [server::handler] GET https://api.example.com/v2/users?status=active&limit=100 from 192.168.1.42/24 via fe80::1 ptr=0x7f4a2c00b340 pid=worker[1234] uuid=550e8400-e29b-41d4-a716-446655440000 key=value path=/var/log/app/server.log count=42 notify=admin@example.com "request completed" null true {"status": 200}"#;

const NO_MATCH_LINE: &str =
    "just a plain boring log line with no patterns to match whatsoever and no special characters";

fn build_4h() -> Highlighter {
    Highlighter::builder()
        .with_number_highlighter(NumberConfig::default())
        .with_uuid_highlighter(UuidConfig::default())
        .with_keyword_highlighter(vec![
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
        ])
        .with_quote_highlighter(QuoteConfig::default())
        .build()
        .unwrap()
}

fn bench_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline");

    group.bench_function("4h_match", |b| {
        let h = build_4h();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("4h_no_match", |b| {
        let h = build_4h();
        b.iter(|| h.apply(black_box(NO_MATCH_LINE)));
    });

    group.finish();
}

criterion_group!(benches, bench_pipeline);
criterion_main!(benches);
