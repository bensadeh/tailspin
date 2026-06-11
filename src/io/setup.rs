use crate::config::{Source, Target};
use crate::io::presenter::Presenter;
use crate::io::presenter::pager::{Pager, PagerOptions};
use crate::io::reader::Reader;
use crate::io::reader::command::CommandReader;
use crate::io::reader::file_reader::FileReader;
use crate::io::reader::stdin::StdinReader;
use crate::io::writer::Writer;
use crate::io::writer::temp_file::TempFile;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io::BufWriter;

pub struct IoSetup {
    pub reader: Reader,
    pub writer: Writer,
    pub presenter: Presenter,
}

pub async fn initialize_io(source: Source, target: Target) -> Result<(IoSetup, Option<TempDir>)> {
    let reader = get_reader(source).await?;
    let (writer, presenter, temp_dir) = get_writer_presenter_and_temp_dir(target).await?;

    Ok((
        IoSetup {
            reader,
            writer,
            presenter,
        },
        temp_dir,
    ))
}

async fn get_reader(input: Source) -> Result<Reader> {
    let reader = match input {
        Source::File(file) => Reader::File(FileReader::new(file.path, file.terminate_after_first_read).await?),
        Source::Stdin => Reader::Stdin(StdinReader::new()),
        Source::Command(cmd) => Reader::Command(CommandReader::new(cmd).await?),
    };

    Ok(reader)
}

async fn get_writer_presenter_and_temp_dir(output: Target) -> Result<(Writer, Presenter, Option<TempDir>)> {
    let pager_opts = match output {
        Target::Less(opts) => PagerOptions::Less(opts),
        Target::CustomPager(opts) => PagerOptions::Custom(opts),
        Target::Stdout => return Ok((Writer::Stdout, Presenter::StdOut, None)),
    };

    let (writer, presenter, temp_dir) = get_temp_file_and_pager(pager_opts).await?;

    Ok((writer, presenter, Some(temp_dir)))
}

async fn get_temp_file_and_pager(pager_opts: PagerOptions) -> Result<(Writer, Presenter, TempDir)> {
    let (temp_dir, path_buf, buf_writer) = create_temp_file().await?;
    let temp_file = TempFile::new(buf_writer).await;
    let pager = Pager::new(path_buf, pager_opts);

    Ok((Writer::TempFile(temp_file), Presenter::Pager(pager), temp_dir))
}

async fn create_temp_file() -> Result<(TempDir, PathBuf, BufWriter<File>)> {
    let filename = "tailspin.temp";

    let temp_dir = tempfile::tempdir().context("Could not locate temporary directory")?;

    let temp_file_path = temp_dir.path().join(filename);
    let output_file = File::create(&temp_file_path)
        .await
        .context("Could not create temporary file")?;

    let buf_writer = BufWriter::new(output_file);

    Ok((temp_dir, temp_file_path, buf_writer))
}
