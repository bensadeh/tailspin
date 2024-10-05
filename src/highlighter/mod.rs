pub mod config;

use crate::highlighter::config::HighlighterGroups;
use crate::theme::Theme;
use inlet_manifold::{Error, Highlighter, IpV4Config};

pub fn get_highlighter(highlighter_groups: HighlighterGroups, theme: Theme) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if highlighter_groups.numbers {
        builder.with_number_highlighter(theme.numbers);
    }

    if highlighter_groups.uuids {
        builder.with_uuid_highlighter(theme.uuids);
    }

    if highlighter_groups.quotes {
        builder.with_quote_highlighter(theme.quotes);
    }

    if highlighter_groups.ip_addresses {
        builder.with_ip_v6_highlighter(theme.ip_v6_addresses);
        builder.with_ip_v4_highlighter(theme.ip_v4_addresses);
    }

    if highlighter_groups.dates {
        builder.with_date_time_highlighters(theme.dates);
    }

    if highlighter_groups.paths {
        builder.with_unix_path_highlighter(theme.paths);
    }

    if highlighter_groups.urls {
        builder.with_url_highlighter(theme.urls);
    }

    if highlighter_groups.pointers {
        builder.with_pointer_highlighter(theme.pointers);
    }

    if highlighter_groups.processes {
        builder.with_unix_process_highlighter(theme.processes);
    }

    if highlighter_groups.key_value_pairs {
        builder.with_key_value_highlighter(theme.key_value_pairs);
    }

    builder.build()
}
