pub enum Input {
    FilePath(String),
    FolderPath(String),
    Stdin,
}

pub enum Output {
    TempFile,
    Stdout,
}
