use super::build_regex;
use memchr::memchr;
use regex::Regex;

use crate::core::config::JvmStackTraceConfig;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct JvmStackFinder {
    marker_regex: Regex,
    header_regex: Regex,
    frame_regex: Regex,
    file_line_regex: Regex,
    more_regex: Regex,
    config: JvmStackTraceConfig,
}

impl JvmStackFinder {
    pub fn new(config: JvmStackTraceConfig) -> Self {
        let marker_pattern = r"(?m)^\s*(?P<marker>(?:Caused by|Suppressed):)";
        let marker_regex = build_regex(marker_pattern);

        let header_pattern = r"(?x)
            \b
            (?P<package>(?:[a-z][a-zA-Z0-9_$]*\.)+)
            (?P<class>[A-Z][a-zA-Z0-9_$]*)
            (?P<colon>:)
        ";
        let header_regex = build_regex(header_pattern);

        let frame_pattern = r"(?xm)
            ^(?P<indent>\s+)
            (?P<at>at\s+)
            (?P<fqname>
                (?:[a-zA-Z_$][a-zA-Z0-9_$.]*/)?              # optional JDK module/loader prefix
                [a-zA-Z_$][a-zA-Z0-9_$]*
                (?:\.[a-zA-Z_$][a-zA-Z0-9_$]*)*
                \.(?:[a-zA-Z_$][a-zA-Z0-9_$\-]*|<(?:init|clinit)>) # final segment: '-' permits Kotlin inline-class mangling; <init>/<clinit> are constructor/static-init
            )
            (?P<open>\()
            (?P<contents>(?u:[^)\n]*))   # (?u:) because a byte-mode negated class could match invalid UTF-8, which the str API rejects
            (?P<close>\))
        ";
        let frame_regex = build_regex(frame_pattern);

        let file_line_pattern = r"(?x)
            ^
            (?P<file>[A-Za-z_$][A-Za-z0-9_$.]*\.[a-zA-Z][a-zA-Z0-9]*)
            (?:
                (?P<colon>:)(?P<line>\d+)
                (?:(?P<col_colon>:)(?P<col>\d+))?
            )?
            $
        ";
        let file_line_regex = build_regex(file_line_pattern);

        let more_pattern = r"(?m)^\s+(?P<ellipsis>\.\.\.)\s+(?P<count>\d+)\s+(?P<more>more)\s*$";
        let more_regex = build_regex(more_pattern);

        Self {
            marker_regex,
            header_regex,
            frame_regex,
            file_line_regex,
            more_regex,
            config,
        }
    }
}

impl Finder for JvmStackFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'.', input.as_bytes()).is_none() {
            return;
        }

        let JvmStackTraceConfig {
            caused_by,
            package,
            exception,
            frame,
            file,
            unknown_source,
            line_number,
        } = self.config;

        for caps in self.marker_regex.captures_iter(input) {
            let marker = caps.name("marker").unwrap();
            collector.push(marker.start(), marker.end(), caused_by);
        }

        for caps in self.header_regex.captures_iter(input) {
            let pkg = caps.name("package").unwrap();
            let cls = caps.name("class").unwrap();
            let colon = caps.name("colon").unwrap();
            let cls_text = cls.as_str();
            if !cls_text.ends_with("Exception") && !cls_text.ends_with("Error") {
                continue;
            }
            collector.push(pkg.start(), pkg.end(), package);
            collector.push(cls.start(), cls.end(), exception);
            collector.push(colon.start(), colon.end(), frame);
        }

        for caps in self.frame_regex.captures_iter(input) {
            let at = caps.name("at").unwrap();
            let open = caps.name("open").unwrap();
            let close = caps.name("close").unwrap();
            collector.push(at.start(), open.end(), frame);
            collector.push(close.start(), close.end(), frame);

            let contents = caps.name("contents").unwrap();
            let cstart = contents.start();
            let cstr = contents.as_str();

            if cstr == "Unknown Source" || cstr == "<generated>" || cstr == "Native Method" {
                collector.push(cstart, contents.end(), unknown_source);
            } else if let Some(inner) = self.file_line_regex.captures(cstr) {
                let file_match = inner.name("file").unwrap();
                collector.push(cstart + file_match.start(), cstart + file_match.end(), file);
                if let (Some(c), Some(ln)) = (inner.name("colon"), inner.name("line")) {
                    collector.push(cstart + c.start(), cstart + c.end(), frame);
                    collector.push(cstart + ln.start(), cstart + ln.end(), line_number);
                }
                if let (Some(c), Some(col)) = (inner.name("col_colon"), inner.name("col")) {
                    collector.push(cstart + c.start(), cstart + c.end(), frame);
                    collector.push(cstart + col.start(), cstart + col.end(), line_number);
                }
            }
        }

        for caps in self.more_regex.captures_iter(input) {
            let ellipsis = caps.name("ellipsis").unwrap();
            let count = caps.name("count").unwrap();
            let more = caps.name("more").unwrap();
            collector.push(ellipsis.start(), ellipsis.end(), frame);
            collector.push(count.start(), count.end(), line_number);
            collector.push(more.start(), more.end(), frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> JvmStackFinder {
        JvmStackFinder::new(JvmStackTraceConfig {
            caused_by: Style::new().bold(),
            package: Style::new().fg(Color::Red).faint(),
            exception: Style::new().fg(Color::Red),
            frame: Style::new().fg(Color::Red).faint(),
            file: Style::new().fg(Color::Yellow),
            unknown_source: Style::new().fg(Color::Yellow).faint(),
            line_number: Style::new().fg(Color::Cyan),
        })
    }

    fn spans(input: &str) -> Vec<(usize, usize, Style)> {
        let mut collector = Collector::new();
        make_finder().find_spans(input, &mut collector);
        collector
            .into_spans()
            .into_iter()
            .map(|s| (s.start, s.end, s.style))
            .collect()
    }

    fn span_text<'a>(input: &'a str, span: &(usize, usize, Style)) -> &'a str {
        &input[span.0..span.1]
    }

    #[test]
    fn header_splits_package_class_colon() {
        let input = "no.finntech.statistics.email.EmailNotSentException: No recipients";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert_eq!(texts, ["no.finntech.statistics.email.", "EmailNotSentException", ":"]);

        let f = make_finder();
        assert_eq!(result[0].2, f.config.package);
        assert_eq!(result[1].2, f.config.exception);
        assert_eq!(result[2].2, f.config.frame);
    }

    #[test]
    fn header_does_not_match_inside_frame() {
        let input = "        at no.finntech.statistics.email.EmailService.sendBrokerEmail(EmailService.kt:171)";
        let f = make_finder();
        let result = spans(input);
        let header_styles: Vec<&Style> = result
            .iter()
            .map(|s| &s.2)
            .filter(|s| **s == f.config.exception)
            .collect();
        assert!(
            header_styles.is_empty(),
            "exception style must not appear in a frame line"
        );
    }

    #[test]
    fn frame_with_non_ascii_contents_matches() {
        // Guards the (?u:) on the contents class: the rest of the pattern is
        // byte-mode, but parenthesized contents may hold arbitrary UTF-8.
        let input = "        at com.example.Soknad.behandle(Søknad.kt:42)";
        let texts: Vec<&str> = spans(input).iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.contains("com.example")));
    }

    #[test]
    fn frame_with_file_and_line() {
        let input = "        at no.finntech.statistics.email.EmailService.sendBrokerEmail(EmailService.kt:171)";
        let f = make_finder();
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        // prefix span includes "at <fqname>("
        assert!(texts.iter().any(|t| t.starts_with("at ") && t.ends_with('(')));
        assert!(texts.contains(&"EmailService.kt"));
        assert!(texts.contains(&":"));
        assert!(texts.contains(&"171"));
        assert!(texts.contains(&")"));

        let line_span = result.iter().find(|s| span_text(input, s) == "171").unwrap();
        assert_eq!(line_span.2, f.config.line_number);
        let file_span = result
            .iter()
            .find(|s| span_text(input, s) == "EmailService.kt")
            .unwrap();
        assert_eq!(file_span.2, f.config.file);
    }

    #[test]
    fn frame_with_dollar_in_fqname() {
        let input = "        at no.finntech.statistics.email.EmailService.access$sendBrokerEmail(EmailService.kt:32)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.contains("EmailService.access$sendBrokerEmail(")));
        assert!(texts.contains(&"32"));
    }

    #[test]
    fn frame_without_line_number() {
        let input =
            "        at no.finntech.statistics.email.EmailService$sendBrokerEmail$1.invokeSuspend(EmailService.kt)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.contains(&"EmailService.kt"));
        // no colon span between file and line
        assert!(!texts.contains(&"171"));
    }

    #[test]
    fn frame_with_unknown_source() {
        let input = "        at sun.reflect.NativeMethodAccessorImpl.invoke0(Unknown Source)";
        let f = make_finder();
        let result = spans(input);
        let unknown = result.iter().find(|s| span_text(input, s) == "Unknown Source").unwrap();
        assert_eq!(unknown.2, f.config.unknown_source);
    }

    #[test]
    fn frame_with_native_method_styled_as_unknown_source() {
        let input = "        at java.lang.reflect.Method.invoke(Native Method)";
        let f = make_finder();
        let result = spans(input);
        let native = result
            .iter()
            .find(|s| span_text(input, s) == "Native Method")
            .expect("Native Method contents should be styled");
        assert_eq!(native.2, f.config.unknown_source);
    }

    #[test]
    fn message_after_exception_is_not_highlighted() {
        let input = "no.foo.MyException: Could not connect to db";
        let result = spans(input);
        let total_styled: usize = result.iter().map(|s| s.1 - s.0).sum();
        // should highlight only "no.foo." + "MyException" + ":"
        let expected = "no.foo.".len() + "MyException".len() + ":".len();
        assert_eq!(total_styled, expected);
    }

    #[test]
    fn empty_input_no_spans() {
        let input = "";
        assert!(spans(input).is_empty());
    }

    #[test]
    fn line_without_dot_is_skipped() {
        let input = "no exceptions or stack frames here";
        assert!(spans(input).is_empty());
    }

    #[test]
    fn frame_with_kotlin_multiplatform_file() {
        // Kotlin Multiplatform files have a platform infix: EventLoop.common.kt, Foo.jvm.kt, etc.
        let input = "        at kotlinx.coroutines.EventLoopImplBase.processNextEvent(EventLoop.common.kt:263)";
        let f = make_finder();
        let result = spans(input);
        let file_span = result
            .iter()
            .find(|s| span_text(input, s) == "EventLoop.common.kt")
            .expect("multi-dot file name should be highlighted");
        assert_eq!(file_span.2, f.config.file);

        let line_span = result.iter().find(|s| span_text(input, s) == "263").unwrap();
        assert_eq!(line_span.2, f.config.line_number);
    }

    #[test]
    fn frame_with_jdk_module_prefix() {
        let input = "        at java.base/sun.nio.ch.SocketChannelImpl.read(SocketChannelImpl.java:276)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.starts_with("at java.base/") && t.ends_with('(')));
        assert!(texts.contains(&"SocketChannelImpl.java"));
        assert!(texts.contains(&"276"));
    }

    #[test]
    fn caused_by_header_matches() {
        let input = "Caused by: java.io.IOException: pipe closed";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.contains(&"Caused by:"));
        assert!(texts.contains(&"java.io."));
        assert!(texts.contains(&"IOException"));

        let f = make_finder();
        let marker = result.iter().find(|s| span_text(input, s) == "Caused by:").unwrap();
        assert_eq!(marker.2, f.config.caused_by);
    }

    #[test]
    fn frame_at_column_zero_is_not_matched() {
        // Stack frames are always indented; "at X.y(...)" at column 0 is prose.
        let input = "We meet at home.foo(now) for dinner";
        assert!(spans(input).is_empty(), "prose with 'at X.y(...)' must not match");
    }

    #[test]
    fn at_keyword_without_dotted_name_does_not_match() {
        let input = "        at run-stage(123)";
        // `run-stage` has no dot, fqname requires at least one — no match.
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(!texts.iter().any(|t| t.starts_with("at ")));
    }

    #[test]
    fn header_requires_exception_or_error_suffix() {
        // Dotted token followed by ":" is not enough — the class must look like an exception type.
        let cases = [
            "Connecting to db.example.com.MasterServer: ready",
            "Reading from /var/log/foo.bar.Baz: ok",
            "config: app.module.Name: production",
            "www.spring.io.Foo: redirect",
            "sentence ending with com.example.Class: trailing",
        ];
        for input in cases {
            assert!(spans(input).is_empty(), "should not match header in: {input}");
        }
    }

    #[test]
    fn header_matches_error_suffix() {
        let input = "java.lang.OutOfMemoryError: Java heap space";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert_eq!(texts, ["java.lang.", "OutOfMemoryError", ":"]);
    }

    #[test]
    fn frame_with_cglib_generated_proxy() {
        // Spring CGLIB proxy: class name has `$$SpringCGLIB$$N`, method body is `<generated>`.
        let input = "        at no.finntech.statistics.aggregator.adsIndex.AdsIndexEsStatisticsRepo$$SpringCGLIB$$0.indexAds(<generated>)";
        let f = make_finder();
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(
            texts.iter().any(|t| t.starts_with("at ") && t.ends_with('(')),
            "frame prefix should be styled; got {texts:?}"
        );
        let generated = result
            .iter()
            .find(|s| span_text(input, s) == "<generated>")
            .expect("<generated> contents should be styled");
        assert_eq!(generated.2, f.config.unknown_source);
    }

    #[test]
    fn frame_with_kotlin_inline_class_mangling() {
        // Kotlin inline-class member functions are mangled with `-XXXXXXXX` (hash) — the dash
        // is part of the JVM method name and must be allowed in the last fqname segment.
        let input = "        at no.finntech.statistics.aggregator.adsIndex.AdsIndexService.processFullDoc-FcBZHsI(AdsIndexService.kt:108)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(
            texts.iter().any(|t| t.contains("processFullDoc-FcBZHsI(")),
            "mangled method name should be in the styled prefix; got {texts:?}"
        );
        assert!(texts.contains(&"AdsIndexService.kt"));
        assert!(texts.contains(&"108"));
    }

    #[test]
    fn frame_with_kotlin_inline_class_suspend_impl() {
        // Kotlin inline-class suspend functions: `-HASH$suspendImpl` suffix.
        let input = "        at no.finntech.statistics.aggregator.adsIndex.AdsIndexService.processAd-WVa51mU$suspendImpl(AdsIndexService.kt:94)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.contains("processAd-WVa51mU$suspendImpl(")));
        assert!(texts.contains(&"AdsIndexService.kt"));
        assert!(texts.contains(&"94"));
    }

    #[test]
    fn frame_with_kotlin_inline_class_no_line_number() {
        let input = "        at no.finntech.statistics.aggregator.adsIndex.AdsIndexService.processAd-WVa51mU(AdsIndexService.kt)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.contains("processAd-WVa51mU(")));
        assert!(texts.contains(&"AdsIndexService.kt"));
    }

    #[test]
    fn caused_by_excludes_leading_whitespace() {
        let input = "    Caused by: java.io.IOException: x";
        let result = spans(input);
        let marker = result.iter().find(|s| span_text(input, s) == "Caused by:").unwrap();
        // marker should start at position 4, not 0
        assert_eq!(marker.0, 4);
    }

    #[test]
    fn frame_with_constructor_init() {
        let input = "        at com.foo.Bar.<init>(Bar.java:42)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(
            texts.iter().any(|t| t.contains("Bar.<init>(")),
            "constructor frame should match; got {texts:?}"
        );
        assert!(texts.contains(&"Bar.java"));
        assert!(texts.contains(&"42"));
    }

    #[test]
    fn frame_with_static_initializer_clinit() {
        let input = "        at com.foo.Bar.<clinit>(Bar.java:7)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.contains("Bar.<clinit>(")));
        assert!(texts.contains(&"7"));
    }

    #[test]
    fn frame_with_init_on_anonymous_inner() {
        let input = "        at com.foo.Bar$1.<init>(Bar.java:42)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.iter().any(|t| t.contains("Bar$1.<init>(")));
    }

    #[test]
    fn suppressed_marker_matches() {
        let input = "        Suppressed: java.io.IOException: pipe closed";
        let f = make_finder();
        let result = spans(input);
        let marker = result
            .iter()
            .find(|s| span_text(input, s) == "Suppressed:")
            .expect("Suppressed marker should be styled");
        assert_eq!(marker.2, f.config.caused_by);
    }

    #[test]
    fn more_frames_truncation_marker() {
        let input = "        ... 42 more";
        let f = make_finder();
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.contains(&"..."));
        assert!(texts.contains(&"42"));
        assert!(texts.contains(&"more"));

        let count = result.iter().find(|s| span_text(input, s) == "42").unwrap();
        assert_eq!(count.2, f.config.line_number);
        let ellipsis = result.iter().find(|s| span_text(input, s) == "...").unwrap();
        assert_eq!(ellipsis.2, f.config.frame);
    }

    #[test]
    fn more_frames_marker_requires_keyword() {
        // "... 42" without "more" must not match.
        let input = "        ... 42";
        let result = spans(input);
        assert!(result.is_empty(), "ellipsis without 'more' must not match");
    }

    #[test]
    fn file_with_line_and_column() {
        // Kotlin/Native and some test runners emit (File.kt:42:13).
        let input = "        at com.foo.Bar.method(Bar.kt:42:13)";
        let f = make_finder();
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(texts.contains(&"Bar.kt"));
        assert!(texts.contains(&"42"));
        assert!(texts.contains(&"13"));

        let col = result.iter().find(|s| span_text(input, s) == "13").unwrap();
        assert_eq!(col.2, f.config.line_number);
    }
}
