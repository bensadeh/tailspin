use nu_ansi_term::Style;

pub struct Uuid {
    pub number: Style,
    pub letter: Style,
    pub dash: Style,
    pub disabled: bool,
}

pub struct Pointer {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
    pub separator_token: char,
    pub x: Style,
    pub disabled: bool,
}

pub struct Ip {
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
    pub number: Style,
    pub separator: Style,
    pub disabled: bool,
}

pub struct DateWord {
    pub day: Style,
    pub month: Style,
    pub number: Style,
    pub disabled: bool,
}

pub struct Time {
    pub time: Style,
    pub zone: Style,
    pub separator: Style,
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
    pub date_word: DateWord,
    pub ip: Ip,
    pub key_value: KeyValue,
    pub number: Number,
    pub path: FilePath,
    pub pointer: Pointer,
    pub process: Process,
    pub quotes: Quotes,
    pub time: Time,
    pub url: Url,
    pub uuid: Uuid,
    pub keywords: Vec<Keyword>,
    pub regexps: Vec<Regexp>,
}
