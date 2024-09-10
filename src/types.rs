use std::borrow::Cow;

use crate::line_info::LineInfo;

pub trait Highlight {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool;
    fn only_apply_to_segments_not_already_highlighted(&self) -> bool;
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str>;
}

pub enum ExitType {
    Success,
    General,
    ShellBuiltinMisuse,
}

impl ExitType {
    pub const fn code(&self) -> i32 {
        match self {
            Self::Success => 0,
            Self::General => 1,
            Self::ShellBuiltinMisuse => 2,
        }
    }
}

pub struct Error {
    pub exit_type: ExitType,
    pub message: String,
}

pub struct Config {
    pub input: Input,
    pub output: Output,
    pub follow: bool,
    pub start_at_end: bool,
}

pub struct PathAndLineCount {
    pub path: String,
    pub line_count: usize,
}

pub struct FolderInfo {
    pub folder_name: String,
    pub file_paths: Vec<String>,
}

pub enum Input {
    File(PathAndLineCount),
    Folder(FolderInfo),
    Command(String),
    Stdin,
}

pub enum Output {
    TempFile,
    Stdout,
    Suppress,
}
