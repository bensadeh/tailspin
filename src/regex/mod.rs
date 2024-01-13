use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
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
}
