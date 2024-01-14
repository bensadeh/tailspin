use nu_ansi_term::Style;

pub struct Uuid {
    pub number: Style,
    pub letter: Style,
    pub dash: Style,
    pub disabled: bool,
}

pub struct IpV4 {
    pub segment: Style,
    pub separator: Style,
    pub disabled: bool,
}

pub struct IpV6 {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
    pub disabled: bool,
}

pub struct KeyValue {
    pub key: Style,
    pub separator: Style,
    pub disabled: bool,
}

pub struct FilePath {
    pub segment: Style,
    pub separator: Style,
    pub disabled: bool,
}

pub struct Date {
    pub style: Style,
    pub disabled: bool,
}

pub struct Time {
    pub time: Style,
    pub zone: Style,
    pub disabled: bool,
}

pub struct Process {
    pub name: Style,
    pub id: Style,
    pub separator: Style,
    pub disabled: bool,
}

pub struct Number {
    pub style: Style,
    pub disabled: bool,
}

pub struct Quotes {
    pub style: Style,
    pub token: char,
    pub disabled: bool,
}

pub struct Url {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
    pub disabled: bool,
}

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct Keyword {
    pub style: Style,
    pub words: Vec<String>,
    pub border: bool,
}

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct Regexp {
    pub regular_expression: String,
    pub style: Style,
    pub border: bool,
}

pub struct Theme {
    pub date: Date,
    pub time: Time,
    pub number: Number,
    pub quotes: Quotes,
    pub uuid: Uuid,
    pub url: Url,
    pub ip_v4: IpV4,
    pub ip_v6: IpV6,
    pub key_value: KeyValue,
    pub path: FilePath,
    pub process: Process,
    pub keywords: Vec<Keyword>,
    pub regexps: Vec<Regexp>,
}
