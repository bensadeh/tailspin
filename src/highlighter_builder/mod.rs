pub mod builtins;
pub mod groups;

use crate::highlighter_builder::groups::HighlighterGroups;
use crate::theme::Theme;
use tailspin::config::KeywordConfig;
use tailspin::*;

pub fn get_highlighter(
    highlighter_groups: HighlighterGroups,
    theme: Theme,
    keywords: Vec<KeywordConfig>,
) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if highlighter_groups.json {
        builder.with_json_highlighter(theme.json);
    }

    for regex in theme.regexes {
        builder.with_regex_highlighter(regex);
    }

    if highlighter_groups.dates {
        builder.with_date_time_highlighters(theme.dates);
    }

    if highlighter_groups.ip_addresses {
        builder.with_ip_v6_highlighter(theme.ip_v6_addresses);
        builder.with_ip_v4_highlighter(theme.ip_v4_addresses);
    }

    if highlighter_groups.urls {
        builder.with_url_highlighter(theme.urls);
    }

    if highlighter_groups.paths {
        builder.with_unix_path_highlighter(theme.paths);
    }

    if highlighter_groups.key_value_pairs {
        builder.with_key_value_highlighter(theme.key_value_pairs);
    }

    if highlighter_groups.uuids {
        builder.with_uuid_highlighter(theme.uuids);
    }

    if highlighter_groups.pointers {
        builder.with_pointer_highlighter(theme.pointers);
    }

    if highlighter_groups.processes {
        builder.with_unix_process_highlighter(theme.processes);
    }

    if highlighter_groups.numbers {
        builder.with_number_highlighter(theme.numbers);
    }

    builder.with_keyword_highlighter(keywords);

    if highlighter_groups.quotes {
        builder.with_quote_highlighter(theme.quotes);
    }

    builder.build()
}
