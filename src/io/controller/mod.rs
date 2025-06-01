use crate::cli::get_config;
use crate::config::{Source, Target};
use crate::initial_read::{InitialReadCompleteReceiver, InitialReadCompleteSender, initial_read_complete_channel};
use crate::io::presenter::pager::{CustomPagerOptions, LessPagerOptions, Pager, PagerOptions};
use crate::io::presenter::stdout::StdoutPresenter;
use crate::io::reader::command::CommandReader;
use crate::io::reader::linemux::Linemux;
use crate::io::reader::stdin::StdinReader;
use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;
use miette::{Context, IntoDiagnostic, Result};
use std::path::PathBuf;
use tailspin::Highlighter;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io::BufWriter;
use uuid::Uuid;

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
    Pager(Pager),
    StdOut(StdoutPresenter),
}

pub async fn initialize_io() -> Result<(
    Reader,
    Writer,
    Presenter,
    Highlighter,
    InitialReadCompleteSender,
    InitialReadCompleteReceiver,
    Option<TempDir>,
)> {
    let config = get_config()?;
    let (read_complete_sender, read_complete_receiver) = initial_read_complete_channel();

    let reader = get_reader(config.source).await?;
    let (writer, presenter, temp_dir) = get_writer_presenter_and_temp_dir(config.target).await?;

    Ok((
        reader,
        writer,
        presenter,
        config.highlighter,
        read_complete_sender,
        read_complete_receiver,
        temp_dir,
    ))
}

async fn get_reader(input: Source) -> Result<Reader> {
    let reader = match input {
        Source::File(file) => Reader::Linemux(Linemux::new(file.path, file.terminate_after_first_read).await?),
        Source::Stdin => Reader::Stdin(StdinReader::new()),
        Source::Command(cmd) => Reader::Command(CommandReader::new(cmd).await?),
    };

    Ok(reader)
}

async fn get_writer_presenter_and_temp_dir(output: Target) -> Result<(Writer, Presenter, Option<TempDir>)> {
    match output {
        Target::Less(opts) => {
            let less_pager_options = LessPagerOptions { follow: opts.follow };
            let pager_opts = PagerOptions::Less(less_pager_options);
            let (writer, presenter, temp_dir) = get_temp_file_and_pager(pager_opts).await?;

            Ok((writer, presenter, Some(temp_dir)))
        }
        Target::CustomPager(opts) => {
            let custom_pager_options = CustomPagerOptions {
                command: opts.command,
                args: opts.args,
            };
            let pager_options = PagerOptions::Custom(custom_pager_options);
            let (writer, presenter, temp_dir) = get_temp_file_and_pager(pager_options).await?;

            Ok((writer, presenter, Some(temp_dir)))
        }
        Target::Stdout => Ok((
            Writer::Stdout(StdoutWriter::new()),
            Presenter::StdOut(StdoutPresenter::new()),
            None,
        )),
    }
}

async fn get_temp_file_and_pager(pager_opts: PagerOptions) -> Result<(Writer, Presenter, TempDir)> {
    let (temp_dir, path_buf, buf_writer) = create_temp_file().await?;
    let temp_file = TempFile::new(buf_writer).await;
    let pager = Pager::new(path_buf, pager_opts);

    Ok((Writer::TempFile(temp_file), Presenter::Pager(pager), temp_dir))
}

async fn create_temp_file() -> Result<(TempDir, PathBuf, BufWriter<File>)> {
    let filename = format!("tailspin.temp.{}", Uuid::new_v4());

    let temp_dir = tempfile::tempdir()
        .into_diagnostic()
        .wrap_err("Could not locate temporary directory")?;

    let temp_file_path = temp_dir.path().join(filename);
    let output_file = File::create(&temp_file_path)
        .await
        .into_diagnostic()
        .wrap_err("Could not create temporary file")?;

    let buf_writer = BufWriter::new(output_file);

    Ok((temp_dir, temp_file_path, buf_writer))
}
