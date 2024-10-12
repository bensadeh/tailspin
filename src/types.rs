pub const OK: i32 = 0;
pub const GENERAL_ERROR: i32 = 1;
pub const MISUSE_SHELL_BUILTIN: i32 = 2;

pub struct Error {
    pub exit_code: i32,
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
