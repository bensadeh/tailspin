use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DATE_REGEX: Regex = {
        Regex::new(
            r"(?x)                       
            \d{4}-\d{2}-\d{2}
    ",
        )
        .expect("Invalid regex pattern")
    };
    pub static ref TIME_REGEX: Regex = {
        Regex::new(
            r"(?x)                       
                (?:
                    (?P<T>[T\s])?                # Capture separator (either a space or T)
                    (?P<time>\d{2}:\d{2}:\d{2}) # Capture time alone
                    (?P<frac>[.,]\d+)?          # Capture fractional seconds
                    (?P<tz>Z)?                  # Capture timezone (Zulu time)
                )  
    ",
        )
        .expect("Invalid regex pattern")
    };
    pub static ref IP_ADDRESS_REGEX: Regex = {
        Regex::new(r"(\b\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3}\b)").expect("Invalid IP address regex pattern")
    };
    pub static ref NUMBER_REGEX: Regex = {
        Regex::new(
            r"(?x)       # Enable comments and whitespace insensitivity
            \b           # Word boundary, ensures we are at the start of a number
            \d+          # Matches one or more digits
            (\.          # Start a group to match a decimal part
            \d+          # Matches one or more digits after the dot
            )?           # The decimal part is optional
            \b           # Word boundary, ensures we are at the end of a number
            ",
        )
        .expect("Invalid regex pattern")
    };
    pub static ref PATH_REGEX: Regex = {
        Regex::new(
            r"(?x)                        # Enable extended mode for readability
            (?P<path>                     # Capture the path segment
                [~/.][\w./-]*             # Match zero or more word characters, dots, slashes, or hyphens
                /[\w.-]*                  # Match a path segment separated by a slash
            )",
        )
        .expect("Invalid regex pattern")
    };
    pub static ref URL_REGEX: Regex = {
        Regex::new(
            r"(?P<protocol>http|https)(:)(//)(?P<host>[^:/\n\s]+)(?P<path>[/a-zA-Z0-9\-_.]*)?(?P<query>\?[^#\n ]*)?",
        )
        .expect("Invalid regex pattern")
    };
    pub static ref QUERY_PARAMS_REGEX: Regex = {
        Regex::new(r"(?P<delimiter>[?&])(?P<key>[^=]*)(?P<equal>=)(?P<value>[^&]*)")
            .expect("Invalid query params regex pattern")
    };
    pub static ref UUID_REGEX: Regex = {
        Regex::new(
            r"(?x)
            (\b[0-9a-fA-F]{8}\b)    # Match first segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{4}\b)    # Match second segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{4}\b)    # Match third segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{4}\b)    # Match fourth segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{12}\b)   # Match last segment of UUID
        ",
        )
        .expect("Invalid UUID regex pattern")
    };
    pub static ref KEY_VALUE_REGEX: Regex =
        Regex::new(r"(?P<space_or_start>(^)|\s)(?P<key>\w+\b)(?P<equals>=)").expect("Invalid regex pattern");
    pub static ref PROCESS_REGEX: Regex =
        Regex::new(r"(?P<process_name>[\w-]+)\[(?P<process_num>\d+)\]").expect("Invalid regex pattern");
}
