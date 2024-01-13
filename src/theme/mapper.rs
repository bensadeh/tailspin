use crate::theme;
use crate::theme::raw::Theme;

pub fn map(raw_theme: Theme) -> theme::processed::Theme {
    return theme::processed::Theme {
        date: Default::default(),
        time: Default::default(),
        number: Default::default(),
        url: Default::default(),
        path: Default::default(),
        process: Default::default(),
        keywords: None,
        ip: Default::default(),
        key_value: Default::default(),
        uuid: Default::default(),
        quotes: Default::default(),
        regexps: None,
    };
}
