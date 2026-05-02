use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct JvmStackFinder {
    caused_by_regex: Regex,
    header_regex: Regex,
    frame_regex: Regex,
    file_line_regex: Regex,
    caused_by: Style,
    package: Style,
    exception: Style,
    frame: Style,
    file: Style,
    unknown_source: Style,
    line_number: Style,
}

impl JvmStackFinder {
    pub fn new(
        caused_by: Style,
        package: Style,
        exception: Style,
        frame: Style,
        file: Style,
        unknown_source: Style,
        line_number: Style,
    ) -> Self {
        let caused_by_pattern = r"(?m)^\s*Caused by:";
        let caused_by_regex = RegexBuilder::new(caused_by_pattern)
            .unicode(false)
            .build()
            .expect("hardcoded JVM stack trace caused-by regex must compile");

        let header_pattern = r"(?x)
            \b
            (?P<package>(?:[a-z][a-zA-Z0-9_$]*\.)+)
            (?P<class>[A-Z][a-zA-Z0-9_$]*)
            (?P<colon>:)
        ";
        let header_regex = RegexBuilder::new(header_pattern)
            .unicode(false)
            .build()
            .expect("hardcoded JVM stack trace header regex must compile");

        let frame_pattern = r"(?xm)
            ^(?P<indent>\s+)
            (?P<at>at\s+)
            (?P<fqname>
                (?:[a-zA-Z_$][a-zA-Z0-9_$.]*/)?      # optional JDK module/loader prefix
                [a-zA-Z_$][a-zA-Z0-9_$]*
                (?:\.[a-zA-Z_$][a-zA-Z0-9_$]*)+
            )
            (?P<open>\()
            (?P<contents>[^)\n]*)
            (?P<close>\))
        ";
        let frame_regex = RegexBuilder::new(frame_pattern)
            .build()
            .expect("hardcoded JVM stack trace frame regex must compile");

        let file_line_pattern = r"(?x)
            ^
            (?P<file>[A-Za-z_$][A-Za-z0-9_$.]*\.[a-zA-Z][a-zA-Z0-9]*)
            (?:(?P<colon>:)(?P<line>\d+))?
            $
        ";
        let file_line_regex = RegexBuilder::new(file_line_pattern)
            .unicode(false)
            .build()
            .expect("hardcoded JVM stack trace file/line regex must compile");

        Self {
            caused_by_regex,
            header_regex,
            frame_regex,
            file_line_regex,
            caused_by,
            package,
            exception,
            frame,
            file,
            unknown_source,
            line_number,
        }
    }
}

impl Finder for JvmStackFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'.', input.as_bytes()).is_none() {
            return;
        }

        for m in self.caused_by_regex.find_iter(input) {
            let marker_start = m.end() - "Caused by:".len();
            collector.push(marker_start, m.end(), self.caused_by);
        }

        for caps in self.header_regex.captures_iter(input) {
            let pkg = caps.name("package").unwrap();
            let cls = caps.name("class").unwrap();
            let colon = caps.name("colon").unwrap();
            collector.push(pkg.start(), pkg.end(), self.package);
            collector.push(cls.start(), cls.end(), self.exception);
            collector.push(colon.start(), colon.end(), self.frame);
        }

        for caps in self.frame_regex.captures_iter(input) {
            let at = caps.name("at").unwrap();
            let open = caps.name("open").unwrap();
            let close = caps.name("close").unwrap();
            collector.push(at.start(), open.end(), self.frame);
            collector.push(close.start(), close.end(), self.frame);

            let contents = caps.name("contents").unwrap();
            let cstart = contents.start();
            let cstr = &input[cstart..contents.end()];

            if cstr == "Unknown Source" {
                collector.push(cstart, contents.end(), self.unknown_source);
            } else if let Some(inner) = self.file_line_regex.captures(cstr) {
                let file = inner.name("file").unwrap();
                collector.push(cstart + file.start(), cstart + file.end(), self.file);
                if let (Some(c), Some(ln)) = (inner.name("colon"), inner.name("line")) {
                    collector.push(cstart + c.start(), cstart + c.end(), self.frame);
                    collector.push(cstart + ln.start(), cstart + ln.end(), self.line_number);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> JvmStackFinder {
        JvmStackFinder::new(
            Style::new().bold(),
            Style::new().fg(Color::Red).faint(),
            Style::new().fg(Color::Red),
            Style::new().fg(Color::Red).faint(),
            Style::new().fg(Color::Yellow),
            Style::new().fg(Color::Yellow).faint(),
            Style::new().fg(Color::Cyan),
        )
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
        assert_eq!(result[0].2, f.package);
        assert_eq!(result[1].2, f.exception);
        assert_eq!(result[2].2, f.frame);
    }

    #[test]
    fn header_does_not_match_inside_frame() {
        let input = "        at no.finntech.statistics.email.EmailService.sendBrokerEmail(EmailService.kt:171)";
        let f = make_finder();
        let result = spans(input);
        let header_styles: Vec<&Style> = result.iter().map(|s| &s.2).filter(|s| **s == f.exception).collect();
        assert!(
            header_styles.is_empty(),
            "exception style must not appear in a frame line"
        );
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
        assert_eq!(line_span.2, f.line_number);
        let file_span = result
            .iter()
            .find(|s| span_text(input, s) == "EmailService.kt")
            .unwrap();
        assert_eq!(file_span.2, f.file);
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
        assert_eq!(unknown.2, f.unknown_source);
    }

    #[test]
    fn frame_with_native_method_leaves_contents_plain() {
        // Native Method is unhandled — paren scaffold is styled, contents remain plain.
        let input = "        at java.lang.reflect.Method.invoke(Native Method)";
        let result = spans(input);
        let texts: Vec<&str> = result.iter().map(|s| span_text(input, s)).collect();
        assert!(!texts.contains(&"Native Method"));
        assert!(texts.contains(&")"));
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
        assert_eq!(file_span.2, f.file);

        let line_span = result.iter().find(|s| span_text(input, s) == "263").unwrap();
        assert_eq!(line_span.2, f.line_number);
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
        assert_eq!(marker.2, f.caused_by);
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
    fn caused_by_excludes_leading_whitespace() {
        let input = "    Caused by: java.io.IOException: x";
        let result = spans(input);
        let marker = result.iter().find(|s| span_text(input, s) == "Caused by:").unwrap();
        // marker should start at position 4, not 0
        assert_eq!(marker.0, 4);
    }
}
