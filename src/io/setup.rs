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
use tempfile::TempPath;
use tokio::fs::File;
use tokio::io::BufWriter;

pub struct IoSetup {
    pub reader: Reader,
    pub writer: Writer,
    pub presenter: Presenter,
}

pub async fn initialize_io(source: Source, target: Target) -> Result<IoSetup> {
    let reader = get_reader(source).await?;
    let (writer, presenter) = get_writer_and_presenter(target)?;

    Ok(IoSetup {
        reader,
        writer,
        presenter,
    })
}

async fn get_reader(input: Source) -> Result<Reader> {
    let reader = match input {
        Source::File(file) => Reader::File(FileReader::new(file.path, file.terminate_after_first_read).await?),
        Source::Stdin => Reader::Stdin(StdinReader::new()),
        Source::Command(cmd) => Reader::Command(CommandReader::new(cmd).await?),
    };

    Ok(reader)
}

fn get_writer_and_presenter(output: Target) -> Result<(Writer, Presenter)> {
    let pager_opts = match output {
        Target::Less(opts) => PagerOptions::Less(opts),
        Target::CustomPager(opts) => PagerOptions::Custom(opts),
        Target::Stdout => return Ok((Writer::Stdout, Presenter::StdOut)),
    };

    let (path, buf_writer) = create_temp_file()?;
    let writer = Writer::TempFile(TempFile::new(buf_writer));
    let pager = Pager::new(path, pager_opts);

    Ok((writer, Presenter::Pager(pager)))
}

fn create_temp_file() -> Result<(TempPath, BufWriter<File>)> {
    let temp_file = tempfile::Builder::new()
        .prefix("tailspin.")
        .tempfile()
        .context("Could not create temporary file")?;

    let (file, path) = temp_file.into_parts();

    Ok((path, BufWriter::new(File::from_std(file))))
}
