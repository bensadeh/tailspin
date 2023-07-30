pub mod config;

use crate::presenter::less::Less;
use crate::presenter::Present;
use crate::reader::linemux::Linemux;
use crate::reader::AsyncLineReader;
use crate::writer::temp_file::TempFile;
use crate::writer::AsyncLineWriter;
use async_trait::async_trait;
use tokio::io;

use crate::controller::config::{Config, Input, Output};
use crate::presenter::empty::NoPresenter;
use crate::reader::stdin::StdinReader;
use crate::writer::stdout::StdoutWriter;
use tokio::sync::oneshot::Sender;

pub struct Io {
    reader: Box<dyn AsyncLineReader + Send>,
    writer: Box<dyn AsyncLineWriter + Send>,
}

pub struct Presenter {
    presenter: Box<dyn Present + Send>,
}

pub async fn get_io_and_presenter(
    config: Config,
    reached_eof_tx: Option<Sender<()>>,
) -> (Io, Presenter) {
    let reader = get_reader(config.input, reached_eof_tx).await;
    let (writer, presenter) = get_writer(config.output).await;

    (Io { reader, writer }, Presenter { presenter })
}

async fn get_reader(
    input: Input,
    reached_eof_tx: Option<Sender<()>>,
) -> Box<dyn AsyncLineReader + Send> {
    match input {
        Input::File(file_info) => {
            Linemux::get_reader(file_info.path, file_info.line_count, reached_eof_tx).await
        }
        Input::Stdin => StdinReader::get_reader(reached_eof_tx),
        _ => unimplemented!(),
    }
}

async fn get_writer(output: Output) -> (Box<dyn AsyncLineWriter + Send>, Box<dyn Present + Send>) {
    match output {
        Output::TempFile => {
            let result = TempFile::get_writer_result().await;
            let writer = result.writer;
            let temp_file_path = result.temp_file_path;

            let presenter = Less::get_presenter(temp_file_path, false);

            (writer, presenter)
        }
        Output::Stdout => {
            let writer = StdoutWriter::new();
            let presenter = NoPresenter::get_presenter();

            (writer, presenter)
        }
    }
}

#[async_trait]
impl AsyncLineReader for Io {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        self.reader.next_line().await
    }
}

#[async_trait]
impl AsyncLineWriter for Io {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.writer.write_line(line).await
    }
}

impl Present for Presenter {
    fn present(&self) {
        self.presenter.present()
    }
}
