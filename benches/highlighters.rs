use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tailspin::Highlighter;
use tailspin::config::*;
use tailspin::style::{Color, Style};

const LOG_LINE: &str = r#"2025-03-07T14:32:01.123Z INFO  [server::handler] GET https://api.example.com/v2/users?status=active&limit=100 from 192.168.1.42/24 via fe80::1 ptr=0x7f4a2c00b340 pid=worker[1234] uuid=550e8400-e29b-41d4-a716-446655440000 key=value path=/var/log/app/server.log count=42 notify=admin@example.com "request completed" null true {"status": 200}"#;

fn bench_individual_highlighters(c: &mut Criterion) {
    let mut group = c.benchmark_group("highlighters");

    group.bench_function("json", |b| {
        let h = Highlighter::builder()
            .with_json_highlighter(JsonConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("json_match", |b| {
        let h = Highlighter::builder()
            .with_json_highlighter(JsonConfig::default())
            .build()
            .unwrap();
        let json_input = r#"{"status": 200, "message": "OK", "data": [1, 2, 3]}"#;
        b.iter(|| h.apply(black_box(json_input)));
    });

    group.bench_function("date_time", |b| {
        let h = Highlighter::builder()
            .with_date_time_highlighters(DateTimeConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("url", |b| {
        let h = Highlighter::builder()
            .with_url_highlighter(UrlConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("ip_v4", |b| {
        let h = Highlighter::builder()
            .with_ip_v4_highlighter(IpV4Config::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("ip_v6", |b| {
        let h = Highlighter::builder()
            .with_ip_v6_highlighter(IpV6Config::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("jvm_stack", |b| {
        let h = Highlighter::builder()
            .with_jvm_stack_trace_highlighter(JvmStackTraceConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("jvm_stack_match", |b| {
        let h = Highlighter::builder()
            .with_jvm_stack_trace_highlighter(JvmStackTraceConfig::default())
            .build()
            .unwrap();
        let stack_input =
            "        at com.example.notifications.email.MessageService.sendBrokerMessage(MessageService.kt:171)";
        b.iter(|| h.apply(black_box(stack_input)));
    });

    group.bench_function("email", |b| {
        let h = Highlighter::builder()
            .with_email_highlighter(EmailConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("uuid", |b| {
        let h = Highlighter::builder()
            .with_uuid_highlighter(UuidConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("pointer", |b| {
        let h = Highlighter::builder()
            .with_pointer_highlighter(PointerConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("unix_path", |b| {
        let h = Highlighter::builder()
            .with_unix_path_highlighter(UnixPathConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("unix_process", |b| {
        let h = Highlighter::builder()
            .with_unix_process_highlighter(UnixProcessConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("key_value", |b| {
        let h = Highlighter::builder()
            .with_key_value_highlighter(KeyValueConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("number", |b| {
        let h = Highlighter::builder()
            .with_number_highlighter(NumberConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("quote", |b| {
        let h = Highlighter::builder()
            .with_quote_highlighter(QuoteConfig::default())
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.bench_function("keyword", |b| {
        let h = Highlighter::builder()
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
            .build()
            .unwrap();
        b.iter(|| h.apply(black_box(LOG_LINE)));
    });

    group.finish();
}

criterion_group!(benches, bench_individual_highlighters);
criterion_main!(benches);
