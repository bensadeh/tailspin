use crate::theme;
use crate::theme::processed::{
    Date, FilePath, IpV4, IpV6, KeyValue, Number, Pointer, Process, Quotes, Time, Url, Uuid,
};
use crate::theme::raw::{Keyword, Regexp};
use nu_ansi_term::{Color, Style};

pub fn map_or_exit_early(raw: theme::raw::Theme) -> theme::processed::Theme {
    theme::processed::Theme {
        date: Date {
            style: raw.date.style.map_or(Date::default().style, to_style),
            disabled: raw.date.disabled,
        },
        time: Time {
            time: raw.time.time.map_or(Time::default().time, to_style),
            zone: raw.time.zone.map_or(Time::default().zone, to_style),
            disabled: raw.time.disabled,
        },
        number: Number {
            style: raw.number.style.map_or(Number::default().style, to_style),
            disabled: raw.number.disabled,
        },
        url: Url {
            http: raw.url.http.map_or(Url::default().http, to_style),
            https: raw.url.https.map_or(Url::default().https, to_style),
            host: raw.url.host.map_or(Url::default().host, to_style),
            path: raw.url.path.map_or(Url::default().path, to_style),
            query_params_key: raw
                .url
                .query_params_key
                .map_or(Url::default().query_params_key, to_style),
            query_params_value: raw
                .url
                .query_params_value
                .map_or(Url::default().query_params_value, to_style),
            symbols: raw.url.symbols.map_or(Url::default().symbols, to_style),
            disabled: raw.url.disabled,
        },
        path: FilePath {
            segment: raw.path.segment.map_or(FilePath::default().segment, to_style),
            separator: raw.path.separator.map_or(FilePath::default().separator, to_style),
            disabled: raw.path.disabled,
        },
        process: Process {
            name: raw.process.name.map_or(Process::default().name, to_style),
            id: raw.process.id.map_or(Process::default().id, to_style),
            separator: raw.process.separator.map_or(Process::default().separator, to_style),
            disabled: raw.process.disabled,
        },
        ip_v4: IpV4 {
            segment: raw.ip_v4.segment.map_or(IpV4::default().segment, to_style),
            separator: raw.ip_v4.separator.map_or(IpV4::default().separator, to_style),
            disabled: raw.ip_v4.disabled,
        },
        ip_v6: IpV6 {
            number: raw.ip_v6.number.map_or(IpV6::default().number, to_style),
            letter: raw.ip_v6.letter.map_or(IpV6::default().letter, to_style),
            separator: raw.ip_v6.separator.map_or(IpV6::default().separator, to_style),
            disabled: raw.ip_v6.disabled,
        },
        key_value: KeyValue {
            key: raw.key_value.key.map_or(KeyValue::default().key, to_style),
            separator: raw.key_value.separator.map_or(KeyValue::default().separator, to_style),
            disabled: raw.key_value.disabled,
        },
        uuid: Uuid {
            number: raw.uuid.number.map_or(Uuid::default().number, to_style),
            letter: raw.uuid.letter.map_or(Uuid::default().letter, to_style),
            dash: raw.uuid.dash.map_or(Uuid::default().dash, to_style),
            disabled: raw.uuid.disabled,
        },
        quotes: Quotes {
            style: raw.quotes.style.map_or(Quotes::default().style, to_style),
            token: raw.quotes.token.map_or(Quotes::default().token, |c| c),
            disabled: raw.quotes.disabled,
        },
        pointer: Pointer {
            number: raw.pointer.number.map_or(Pointer::default().number, to_style),
            letter: raw.pointer.letter.map_or(Pointer::default().letter, to_style),
            separator: raw.pointer.separator.map_or(Pointer::default().separator, to_style),
            x: raw.pointer.x.map_or(Pointer::default().x, to_style),
            disabled: raw.pointer.disabled,
        },
        keywords: process_keywords(raw.keywords),
        regexps: process_regexps(raw.regexps),
    }
}

fn to_style(style: theme::raw::Style) -> Style {
    let fg_color = map_to_color_or_exit_early(&style.fg);
    let bg_color = map_to_color_or_exit_early(&style.bg);

    let mut ansi_style = Style::new().fg(fg_color).on(bg_color);

    if style.bold {
        ansi_style = ansi_style.bold();
    }
    if style.faint {
        ansi_style = ansi_style.dimmed();
    }
    if style.italic {
        ansi_style = ansi_style.italic();
    }
    if style.underline {
        ansi_style = ansi_style.underline();
    }

    ansi_style
}

fn map_to_color_or_exit_early(color: &str) -> Color {
    let color = match color.to_lowercase().as_str() {
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "purple" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),
        "black" => Ok(Color::Black),
        "" => Ok(Color::Default),
        _ => Err(color),
    };

    color.unwrap_or_else(|color| {
        eprintln!(
            "{}: {} is not a valid color",
            Style::new().bold().paint("Could not parse config.toml"),
            Color::Red.paint(color)
        );
        std::process::exit(1);
    })
}

fn process_keywords(raw_keywords: Option<Vec<Keyword>>) -> Vec<theme::processed::Keyword> {
    let mut keywords = Vec::new();

    if let Some(raw_kws) = raw_keywords {
        for raw_keyword in raw_kws {
            let words = raw_keyword.words;
            let style = to_style(raw_keyword.style);
            let border = raw_keyword.border;

            keywords.push(theme::processed::Keyword { style, words, border });
        }
    }

    keywords
}

fn process_regexps(raw_regexps: Option<Vec<Regexp>>) -> Vec<theme::processed::Regexp> {
    let mut regexps = Vec::new();

    if let Some(raw_rxs) = raw_regexps {
        for raw_regexp in raw_rxs {
            let regular_expression = raw_regexp.regular_expression;
            let style = to_style(raw_regexp.style);
            let border = raw_regexp.border;

            regexps.push(theme::processed::Regexp {
                regular_expression,
                style,
                border,
            });
        }
    }

    regexps
}
