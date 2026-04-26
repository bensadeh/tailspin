pub mod builtins;

use crate::cli::resolution::BaseSet;
use crate::cli::{Base, Extra};
use crate::theme::Theme;
use std::collections::HashSet;
use tailspin::Highlighter;
use tailspin::config::{
    DateTimeConfig, EmailConfig, IpV4Config, IpV6Config, JsonConfig, KeyValueConfig, KeywordConfig, NumberConfig,
    PointerConfig, QuoteConfig, RegexConfig, UnixPathConfig, UnixProcessConfig, UrlConfig, UuidConfig,
};

#[derive(Debug)]
pub(crate) enum Stage {
    Json(JsonConfig),
    Regexes(Vec<RegexConfig>),
    Dates(DateTimeConfig),
    Ipv4(IpV4Config),
    Ipv6(IpV6Config),
    Urls(UrlConfig),
    Emails(EmailConfig),
    Paths(UnixPathConfig),
    KeyValuePairs(KeyValueConfig),
    Uuids(UuidConfig),
    Pointers(PointerConfig),
    Processes(UnixProcessConfig),
    Numbers(NumberConfig),
    Keywords(Vec<KeywordConfig>),
    Quotes(QuoteConfig),
}

pub(crate) fn build_pipeline(
    base: &BaseSet,
    extras: &HashSet<Extra>,
    theme: Theme,
    keywords: Vec<KeywordConfig>,
) -> Vec<Stage> {
    let Theme {
        keywords: _, // merged ahead of build_pipeline by cli::keywords::collect_keywords
        regexes,
        numbers,
        uuids,
        quotes,
        ip_v4_addresses,
        ip_v6_addresses,
        dates,
        paths,
        urls,
        emails,
        pointers,
        processes,
        key_value_pairs,
        json,
    } = theme;

    let mut stages = Vec::new();

    if base.contains(Base::Json) {
        stages.push(Stage::Json(json));
    }
    stages.push(Stage::Regexes(regexes));
    if base.contains(Base::Dates) {
        stages.push(Stage::Dates(dates));
    }
    if base.contains(Base::Ipv4) {
        stages.push(Stage::Ipv4(ip_v4_addresses));
    }
    if extras.contains(&Extra::Ipv6) {
        stages.push(Stage::Ipv6(ip_v6_addresses));
    }
    if base.contains(Base::Urls) {
        stages.push(Stage::Urls(urls));
    }
    if base.contains(Base::Emails) {
        stages.push(Stage::Emails(emails));
    }
    if base.contains(Base::Paths) {
        stages.push(Stage::Paths(paths));
    }
    if base.contains(Base::KeyValuePairs) {
        stages.push(Stage::KeyValuePairs(key_value_pairs));
    }
    if base.contains(Base::Uuids) {
        stages.push(Stage::Uuids(uuids));
    }
    if base.contains(Base::Pointers) {
        stages.push(Stage::Pointers(pointers));
    }
    if base.contains(Base::Processes) {
        stages.push(Stage::Processes(processes));
    }
    if base.contains(Base::Numbers) {
        stages.push(Stage::Numbers(numbers));
    }
    stages.push(Stage::Keywords(keywords));
    if base.contains(Base::Quotes) {
        stages.push(Stage::Quotes(quotes));
    }

    stages
}

pub(crate) fn build_highlighter(stages: Vec<Stage>) -> Result<Highlighter, tailspin::Error> {
    let mut b = Highlighter::builder();
    for stage in stages {
        b = match stage {
            Stage::Json(c) => b.with_json_highlighter(c),
            Stage::Regexes(rs) => rs
                .into_iter()
                .fold(b, tailspin::HighlighterBuilder::with_regex_highlighter),
            Stage::Dates(c) => b.with_date_time_highlighters(c),
            Stage::Ipv4(c) => b.with_ip_v4_highlighter(c),
            Stage::Ipv6(c) => b.with_ip_v6_highlighter(c),
            Stage::Urls(c) => b.with_url_highlighter(c),
            Stage::Emails(c) => b.with_email_highlighter(c),
            Stage::Paths(c) => b.with_unix_path_highlighter(c),
            Stage::KeyValuePairs(c) => b.with_key_value_highlighter(c),
            Stage::Uuids(c) => b.with_uuid_highlighter(c),
            Stage::Pointers(c) => b.with_pointer_highlighter(c),
            Stage::Processes(c) => b.with_unix_process_highlighter(c),
            Stage::Numbers(c) => b.with_number_highlighter(c),
            Stage::Keywords(ks) => b.with_keyword_highlighter(ks),
            Stage::Quotes(c) => b.with_quote_highlighter(c),
        };
    }
    b.build()
}
