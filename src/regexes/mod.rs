use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DATE_REGEX: Regex = {
        Regex::new(
            r"(?x)                 # Enable comments and whitespace insensitivity
            \b                     # Word boundary, ensures we are at the start of a date/time string
            (                      # Begin capturing group for the entire date/time string
                \d{4}-\d{2}-\d{2}  # Matches date in the format: yyyy-mm-dd
                (?:                # Begin non-capturing group for the time and timezone
                    (?:\s|T)       # Matches either a whitespace or T (separator between date and time)
                    \d{2}:\d{2}:\d{2}  # Matches time in the format: hh:mm:ss
                    ([.,]\d+)?     # Optionally matches fractional seconds
                    (Z|[+-]\d{2})? # Optionally matches Z or timezone offset in the format: +hh or -hh
                )?                 # End non-capturing group for the time and timezone
                |                  # Alternation, matches either the pattern above or  below
                \d{2}:\d{2}:\d{2}  # Matches time in the format: hh:mm:ss
                ([.,]\d+)?         # Optionally matches fractional seconds
            )                      # End capturing group for the entire date/time string
            \b                     # Word boundary, ensures we are at the end of a date/time string
            ").expect("Invalid regex pattern")
    };
    pub static ref IP_ADDRESS_REGEX: Regex = {
        Regex::new(r"(\b\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3}\b)")
            .expect("Invalid IP address regex pattern")
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
            )"
        ).expect("Invalid regex pattern")
    };
    pub static ref URL_REGEX: Regex = {
        Regex::new(
        r"(?P<protocol>http|https)(:)(//)(?P<host>[^:/\n\s]+)(?P<path>[/a-zA-Z0-9\-_.]*)?(?P<query>\?[^#\n ]*)?")
        .expect("Invalid regex pattern")
    };
    pub static ref QUERY_PARAMS_REGEX: Regex = {
        Regex::new(r"(?P<delimiter>[?&])(?P<key>[^=]*)(?P<equal>=)(?P<value>[^&]*)")
            .expect("Invalid query params regex pattern")
    };
}
