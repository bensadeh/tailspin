use crate::config::{OutputTarget, Source};
use crate::io::presenter::custom_pager::CustomPager;
use crate::io::presenter::empty::NoPresenter;
use crate::io::presenter::less::Less;
use crate::io::reader::command::CommandReader;
use crate::io::reader::linemux::Linemux;
use crate::io::reader::stdin::StdinReader;
use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;
use tokio::sync::oneshot::Sender;

pub struct Io {
    pub reader: Reader,
    pub writer: Writer,
}

pub enum Reader {
    Linemux(Linemux),
    Stdin(StdinReader),
    Command(CommandReader),
}

pub enum Writer {
    TempFile(TempFile),
    Stdout(StdoutWriter),
}

pub enum Presenter {
    Less(Less),
    CustomPager(CustomPager),
    None(NoPresenter),
}

pub async fn get_io_and_presenter(
    input: Source,
    output: OutputTarget,
    reached_eof_tx: Option<Sender<()>>,
) -> (Io, Presenter) {
    let reader = get_reader(input, reached_eof_tx).await;
    let (writer, presenter) = get_writer_and_presenter(output).await;

    (Io { reader, writer }, presenter)
}

async fn get_reader(input: Source, reached_eof_tx: Option<Sender<()>>) -> Reader {
    match input {
        Source::File(file_info) => Linemux::get_reader(file_info.path, file_info.line_count, reached_eof_tx).await,
        Source::Stdin => StdinReader::get_reader(reached_eof_tx),
        Source::Command(cmd) => CommandReader::get_reader(cmd, reached_eof_tx).await,
    }
}

async fn get_writer_and_presenter(output: OutputTarget) -> (Writer, Presenter) {
    match output {
        OutputTarget::Less(opts) => {
            let temp_file = TempFile::new().await;
            let less = Less::new(temp_file.path.clone(), opts.follow);

            (Writer::TempFile(temp_file), Presenter::Less(less))
        }
        OutputTarget::CustomPager(cmd) => {
            let temp_file = TempFile::new().await;
            let custom_pager = CustomPager::new(temp_file.path.clone(), cmd.command);

            (Writer::TempFile(temp_file), Presenter::CustomPager(custom_pager))
        }
        OutputTarget::Stdout => {
            let writer = StdoutWriter::init();
            let presenter = NoPresenter::get_presenter();

            (writer, presenter)
        }
    }
}
