use crate::cli::get_config;
use crate::config::{Source, Target};
use crate::initial_read::{InitialReadCompleteReceiver, InitialReadCompleteSender, initial_read_complete_channel};
use crate::io::presenter::custom_pager::CustomPager;
use crate::io::presenter::empty::NoPresenter;
use crate::io::presenter::less::Less;
use crate::io::reader::command::CommandReader;
use crate::io::reader::linemux::Linemux;
use crate::io::reader::stdin::StdinReader;
use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;
use miette::Result;
use tailspin::Highlighter;

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

pub async fn initialize_io() -> Result<(
    Reader,
    Writer,
    Presenter,
    Highlighter,
    InitialReadCompleteSender,
    InitialReadCompleteReceiver,
)> {
    let config = get_config()?;
    let (read_complete_sender, read_complete_receiver) = initial_read_complete_channel();

    let reader = get_reader(config.source).await?;
    let (writer, presenter) = get_writer_and_presenter(config.target).await?;

    Ok((
        reader,
        writer,
        presenter,
        config.highlighter,
        read_complete_sender,
        read_complete_receiver,
    ))
}

async fn get_reader(input: Source) -> Result<Reader> {
    let reader = match input {
        Source::File(file_info) => Linemux::get_reader(file_info.path, file_info.line_count).await?,
        Source::Stdin => StdinReader::get_reader(),
        Source::Command(cmd) => CommandReader::get_reader(cmd).await?,
    };

    Ok(reader)
}

async fn get_writer_and_presenter(output: Target) -> Result<(Writer, Presenter)> {
    let (writer, presenter) = match output {
        Target::Less(opts) => {
            let temp_file = TempFile::new().await?;
            let less = Less::new(temp_file.path.clone(), opts.follow);

            (Writer::TempFile(temp_file), Presenter::Less(less))
        }
        Target::CustomPager(cmd) => {
            let temp_file = TempFile::new().await?;
            let custom_pager = CustomPager::new(temp_file.path.clone(), cmd.command);

            (Writer::TempFile(temp_file), Presenter::CustomPager(custom_pager))
        }
        Target::Stdout => {
            let writer = StdoutWriter::init();
            let presenter = NoPresenter::get_presenter();

            (writer, presenter)
        }
    };

    Ok((writer, presenter))
}
