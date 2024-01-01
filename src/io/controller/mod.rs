use crate::io::reader::command::CommandReader;
use crate::io::reader::linemux::Linemux;
use crate::io::reader::stdin::StdinReader;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::stdout::StdoutWriter;

use crate::io::writer::AsyncLineWriter;
use crate::types::{Config, Input, Output};

use super::reader::EOFSignaler;
use super::writer::minus::Minus;

pub type Reader = Box<dyn AsyncLineReader + Send>;
pub type Writer = Box<dyn AsyncLineWriter + Send>;

pub async fn get_reader_and_writer(config: Config, eof_signaler: EOFSignaler) -> (Reader, Writer) {
    let reader = get_reader(config.input, config.follow, config.tail, eof_signaler).await;
    let writer = get_writer(config.output).await;

    (reader, writer)
}

async fn get_reader(input: Input, follow: bool, tail: bool, eof_signaler: EOFSignaler) -> Reader {
    match input {
        Input::File(file_info) => {
            Linemux::get_reader_single(file_info.path, file_info.line_count, follow, tail, eof_signaler).await
        }
        Input::Folder(info) => Linemux::get_reader_multiple(info.folder_name, info.file_paths, eof_signaler).await,
        Input::Stdin => StdinReader::get_reader(eof_signaler),
        Input::Command(cmd) => CommandReader::get_reader(cmd, eof_signaler).await,
    }
}

async fn get_writer(output: Output) -> Writer {
    match output {
        Output::Pager => Minus::init().await,
        Output::Stdout => StdoutWriter::init(),
    }
}
