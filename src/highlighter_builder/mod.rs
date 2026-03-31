pub mod builtins;
pub mod groups;

use crate::highlighter_builder::groups::HighlighterGroups;
use crate::theme::Theme;
use tailspin::Highlighter;
use tailspin::config::KeywordConfig;

pub fn get_highlighter(groups: HighlighterGroups, theme: Theme, keywords: Vec<KeywordConfig>) -> Highlighter {
    let mut builder = Highlighter::builder();

    if groups.json {
        builder = builder.with_json_highlighter(theme.json);
    }

    for regex_config in theme.regexes {
        builder = builder.with_regex_highlighter(regex_config);
    }

    if groups.dates {
        builder = builder.with_date_time_highlighters(theme.dates);
    }

    if groups.ip_v4 {
        builder = builder.with_ip_v4_highlighter(theme.ip_v4_addresses);
    }

    if groups.ip_v6 {
        builder = builder.with_ip_v6_highlighter(theme.ip_v6_addresses);
    }

    if groups.urls {
        builder = builder.with_url_highlighter(theme.urls);
    }

    if groups.emails {
        builder = builder.with_email_highlighter(theme.emails);
    }

    if groups.paths {
        builder = builder.with_unix_path_highlighter(theme.paths);
    }

    if groups.key_value_pairs {
        builder = builder.with_key_value_highlighter(theme.key_value_pairs);
    }

    if groups.uuids {
        builder = builder.with_uuid_highlighter(theme.uuids);
    }

    if groups.pointers {
        builder = builder.with_pointer_highlighter(theme.pointers);
    }

    if groups.processes {
        builder = builder.with_unix_process_highlighter(theme.processes);
    }

    if groups.numbers {
        builder = builder.with_number_highlighter(theme.numbers);
    }

    builder = builder.with_keyword_highlighter(keywords);

    if groups.quotes {
        builder = builder.with_quote_highlighter(theme.quotes);
    }

    builder
        .build()
        .expect("Theme configuration should produce a valid highlighter")
}
