mod builtins;
pub mod groups;

use crate::highlighter::builtins::get_builtin_keywords;
use crate::highlighter::groups::HighlighterGroups;
use crate::theme::Theme;
use inlet_manifold::highlighter::HighlightBuilder;
use inlet_manifold::*;

pub fn get_highlighter(
    highlighter_groups: HighlighterGroups,
    theme: Theme,
    keyword_configs_from_cli: Vec<KeywordConfig>,
    disable_builtin_keywords: bool,
) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if highlighter_groups.json {
        builder.with_json_highlighter(theme.json);
    }

    if highlighter_groups.dates {
        builder.with_date_time_highlighters(theme.dates);
    }

    if highlighter_groups.ip_addresses {
        builder.with_ip_v4_highlighter(theme.ip_v4_addresses);
        builder.with_ip_v6_highlighter(theme.ip_v6_addresses);
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

    add_keywords(
        theme.keywords,
        keyword_configs_from_cli,
        disable_builtin_keywords,
        &mut builder,
    );

    for regex in theme.regexes {
        builder.with_regex_highlighter(regex);
    }

    if highlighter_groups.quotes {
        builder.with_quote_highlighter(theme.quotes);
    }

    builder.build()
}

fn add_keywords(
    keywords_from_toml: Vec<KeywordConfig>,
    keyword_from_cli: Vec<KeywordConfig>,
    disable_builtin_keywords: bool,
    builder: &mut HighlightBuilder,
) {
    let builtin_keywords = get_builtin_keywords(disable_builtin_keywords);

    let keywords = vec![]
        .into_iter()
        .chain(builtin_keywords)
        .chain(keywords_from_toml)
        .chain(keyword_from_cli)
        .collect();

    builder.with_keyword_highlighter(keywords);
}
