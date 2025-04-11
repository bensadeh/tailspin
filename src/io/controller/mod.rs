use crate::config::{OutputTarget, Source};
use crate::eof_signal::{EofSignalReceiver, EofSignalSender, eof_signal_channel};
use crate::io::presenter::custom_pager::CustomPager;
use crate::io::presenter::empty::NoPresenter;
use crate::io::presenter::less::Less;
use crate::io::reader::command::CommandReader;
use crate::io::reader::linemux::Linemux;
use crate::io::reader::stdin::StdinReader;
use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;

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

pub async fn get_io_and_presenter_and_eof_receiver(
    input: Source,
    output: OutputTarget,
) -> (Io, Presenter, EofSignalReceiver) {
    let (eof_signal_sender, eof_signal_receiver) = eof_signal_channel();

    let reader = get_reader(input, eof_signal_sender).await;
    let (writer, presenter) = get_writer_and_presenter(output).await;

    (Io { reader, writer }, presenter, eof_signal_receiver)
}

async fn get_reader(input: Source, eof_signal_sender: EofSignalSender) -> Reader {
    match input {
        Source::File(file_info) => Linemux::get_reader(file_info.path, file_info.line_count, eof_signal_sender).await,
        Source::Stdin => StdinReader::get_reader(eof_signal_sender),
        Source::Command(cmd) => CommandReader::get_reader(cmd, eof_signal_sender).await,
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
