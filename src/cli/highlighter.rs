use crate::cli::keywords::collect_keywords;
use crate::cli::resolution::BaseSet;
use crate::cli::{Base, Extra, KeywordColor};
use crate::theme::Theme;
use std::collections::HashSet;
use tailspin::{Highlighter, HighlighterBuilder};

// Registration order below is highlight precedence: earlier finders win overlaps.
pub(crate) fn build_highlighter(
    base: &BaseSet,
    extras: &HashSet<Extra>,
    theme: Theme,
    color_word: &[(KeywordColor, Vec<String>)],
) -> Result<Highlighter, tailspin::Error> {
    let Theme {
        keywords,
        regexes,
        numbers,
        uuids,
        quotes,
        ip_addresses,
        dates,
        durations,
        paths,
        urls,
        emails,
        pointers,
        processes,
        key_value_pairs,
        json,
        jvm_stack_traces,
    } = theme;

    let keywords = collect_keywords(color_word, base.contains(Base::Keywords), keywords);

    let mut b = Highlighter::builder();

    if base.contains(Base::Json) {
        b = b.with_json_highlighter(json);
    }

    b = regexes.into_iter().fold(b, HighlighterBuilder::with_regex_highlighter);

    if base.contains(Base::Dates) {
        b = b.with_date_time_highlighters(dates);
    }
    if base.contains(Base::Ipv4) {
        b = b.with_ip_v4_highlighter(ip_addresses.into());
    }
    if extras.contains(&Extra::Ipv6) {
        b = b.with_ip_v6_highlighter(ip_addresses.into());
    }
    if extras.contains(&Extra::JvmStackTrace) {
        b = b.with_jvm_stack_trace_highlighter(jvm_stack_traces);
    }
    if base.contains(Base::Urls) {
        b = b.with_url_highlighter(urls);
    }
    if base.contains(Base::Emails) {
        b = b.with_email_highlighter(emails);
    }
    if base.contains(Base::Paths) {
        b = b.with_unix_path_highlighter(paths);
    }
    if base.contains(Base::KeyValuePairs) {
        b = b.with_key_value_highlighter(key_value_pairs);
    }
    if base.contains(Base::Uuids) {
        b = b.with_uuid_highlighter(uuids);
    }
    if base.contains(Base::Pointers) {
        b = b.with_pointer_highlighter(pointers);
    }
    if base.contains(Base::Processes) {
        b = b.with_unix_process_highlighter(processes);
    }
    if base.contains(Base::Durations) {
        b = b.with_duration_highlighter(durations);
    }
    if base.contains(Base::Numbers) {
        b = b.with_number_highlighter(numbers.into());
    }

    b = b.with_keyword_highlighter(keywords);

    if base.contains(Base::Quotes) {
        b = b.with_quote_highlighter(quotes.into());
    }

    b.build()
}
