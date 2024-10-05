use crate::theme::*;

impl From<TomlTheme> for Theme {
    fn from(toml: TomlTheme) -> Self {
        Theme {
            numbers: toml.numbers.map_or_else(NumberConfig::default, NumberConfig::from),
            uuids: toml.uuids.map_or_else(UuidConfig::default, UuidConfig::from),
            quotes: toml.quotes.map_or_else(QuotesConfig::default, QuotesConfig::from),
            ip_v4_addresses: toml.ip_addresses.map_or_else(IpV4Config::default, IpV4Config::from),
            ip_v6_addresses: toml.ip_addresses.map_or_else(IpV6Config::default, IpV6Config::from),
            dates: toml.dates.map_or_else(DateTimeConfig::default, DateTimeConfig::from),
            paths: toml.paths.map_or_else(UnixPathConfig::default, UnixPathConfig::from),
            urls: toml.urls.map_or_else(UrlConfig::default, UrlConfig::from),
            pointers: toml.pointers.map_or_else(PointerConfig::default, PointerConfig::from),
            processes: toml
                .processes
                .map_or_else(UnixProcessConfig::default, UnixProcessConfig::from),
            key_value_pairs: toml
                .key_value_pairs
                .map_or_else(KeyValueConfig::default, KeyValueConfig::from),
        }
    }
}

impl From<NumberToml> for NumberConfig {
    fn from(number_toml: NumberToml) -> Self {
        let default_config = NumberConfig::default();

        NumberConfig {
            style: number_toml.number.unwrap_or(default_config.style),
        }
    }
}

impl From<UuidToml> for UuidConfig {
    fn from(uuid_toml: UuidToml) -> Self {
        let default_config = UuidConfig::default();

        UuidConfig {
            number: uuid_toml.number.unwrap_or(default_config.number),
            letter: uuid_toml.letter.unwrap_or(default_config.letter),
            dash: uuid_toml.dash.unwrap_or(default_config.dash),
        }
    }
}

impl From<QuotesToml> for QuotesConfig {
    fn from(quotes_toml: QuotesToml) -> Self {
        let default_config = QuotesConfig::default();

        QuotesConfig {
            quotes_token: quotes_toml.quotes_token.unwrap_or(default_config.quotes_token),
            style: quotes_toml.style.unwrap_or(default_config.style),
        }
    }
}

impl From<IpToml> for IpV4Config {
    fn from(ip_toml: IpToml) -> Self {
        let default_config = IpV4Config::default();

        IpV4Config {
            number: ip_toml.number.unwrap_or(default_config.number),
            separator: ip_toml.separator.unwrap_or(default_config.separator),
        }
    }
}

impl From<IpToml> for IpV6Config {
    fn from(ip_toml: IpToml) -> Self {
        let default_config = IpV6Config::default();

        IpV6Config {
            number: ip_toml.number.unwrap_or(default_config.number),
            letter: ip_toml.letter.unwrap_or(default_config.letter),
            separator: ip_toml.separator.unwrap_or(default_config.separator),
        }
    }
}

impl From<DateToml> for DateTimeConfig {
    fn from(date_toml: DateToml) -> Self {
        let default_config = DateTimeConfig::default();

        DateTimeConfig {
            date: date_toml.date.unwrap_or(default_config.date),
            time: date_toml.time.unwrap_or(default_config.time),
            zone: date_toml.zone.unwrap_or(default_config.zone),
            separator: date_toml.separator.unwrap_or(default_config.separator),
        }
    }
}

impl From<PathToml> for UnixPathConfig {
    fn from(path_toml: PathToml) -> Self {
        let default_config = UnixPathConfig::default();

        UnixPathConfig {
            segment: path_toml.segment.unwrap_or(default_config.segment),
            separator: path_toml.separator.unwrap_or(default_config.separator),
        }
    }
}

impl From<UrlToml> for UrlConfig {
    fn from(url_toml: UrlToml) -> Self {
        let default_config = UrlConfig::default();

        UrlConfig {
            http: url_toml.http.unwrap_or(default_config.http),
            https: url_toml.https.unwrap_or(default_config.https),
            host: url_toml.host.unwrap_or(default_config.host),
            path: url_toml.path.unwrap_or(default_config.path),
            query_params_key: url_toml.query_params_key.unwrap_or(default_config.query_params_key),
            query_params_value: url_toml.query_params_value.unwrap_or(default_config.query_params_value),
            symbols: url_toml.symbols.unwrap_or(default_config.symbols),
        }
    }
}

impl From<PointerToml> for PointerConfig {
    fn from(pointer_toml: PointerToml) -> Self {
        let default_config = PointerConfig::default();

        PointerConfig {
            number: pointer_toml.number.unwrap_or(default_config.number),
            letter: pointer_toml.letter.unwrap_or(default_config.letter),
            separator: pointer_toml.separator.unwrap_or(default_config.separator),
            separator_token: pointer_toml.separator_token.unwrap_or(default_config.separator_token),
            x: pointer_toml.x.unwrap_or(default_config.x),
        }
    }
}

impl From<KeyValueToml> for KeyValueConfig {
    fn from(key_value_toml: KeyValueToml) -> Self {
        let default_config = KeyValueConfig::default();

        KeyValueConfig {
            key: key_value_toml.key.unwrap_or(default_config.key),
            separator: key_value_toml.separator.unwrap_or(default_config.separator),
        }
    }
}

impl From<UnixProcessToml> for UnixProcessConfig {
    fn from(process_toml: UnixProcessToml) -> Self {
        let default_config = UnixProcessConfig::default();

        UnixProcessConfig {
            name: process_toml.name.unwrap_or(default_config.name),
            id: process_toml.id.unwrap_or(default_config.id),
            bracket: process_toml.bracket.unwrap_or(default_config.bracket),
        }
    }
}
