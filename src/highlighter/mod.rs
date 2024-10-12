pub mod groups;

use crate::highlighter::groups::HighlighterGroups;
use crate::theme::Theme;
use inlet_manifold::*;

pub fn get_highlighter(
    highlighter_groups: HighlighterGroups,
    theme: Theme,
    enable_builtin_keywords: bool,
) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if highlighter_groups.json {
        builder.with_json_highlighter(theme.json);
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

    {
        let keywords = if enable_builtin_keywords {
            theme.keywords.into_iter().chain(get_builtin_keywords()).collect()
        } else {
            theme.keywords
        };

        builder.with_keyword_highlighter(keywords);
    }

    if !theme.regexes.is_empty() {
        for regex in theme.regexes {
            builder.with_regex_highlighter(regex);
        }
    }

    if highlighter_groups.quotes {
        builder.with_quote_highlighter(theme.quotes);
    }

    builder.build()
}

fn get_builtin_keywords() -> Vec<KeywordConfig> {
    let severity_levels = vec![
        KeywordConfig {
            words: vec!["ERROR".to_string()],
            style: Style::new().fg(Color::Red),
        },
        KeywordConfig {
            words: vec!["WARN".to_string(), "WARNING".to_string()],
            style: Style::new().fg(Color::Yellow),
        },
        KeywordConfig {
            words: vec!["INFO".to_string()],
            style: Style::new().fg(Color::White),
        },
        KeywordConfig {
            words: vec!["SUCCESS".to_string(), "DEBUG".to_string()],
            style: Style::new().fg(Color::Green),
        },
        KeywordConfig {
            words: vec!["TRACE".to_string()],
            style: Style::new().faint(),
        },
    ];

    let rest_keywords = vec![
        KeywordConfig {
            words: vec!["GET".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Green),
        },
        KeywordConfig {
            words: vec!["POST".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Yellow),
        },
        KeywordConfig {
            words: vec!["PUT".to_string(), "PATCH".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Magenta),
        },
        KeywordConfig {
            words: vec!["DELETE".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Red),
        },
    ];

    let booleans = [KeywordConfig {
        words: vec!["null".to_string(), "true".to_string(), "false".to_string()],
        style: Style::new().fg(Color::Red).italic(),
    }];

    vec![]
        .into_iter()
        .chain(severity_levels)
        .chain(rest_keywords)
        .chain(booleans)
        .collect()
}
