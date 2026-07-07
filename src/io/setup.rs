use crate::io::presenter::Presenter;
use crate::io::presenter::pager::{Pager, PagerOptions};
use crate::io::reader::Reader;
use crate::io::reader::command::CommandReader;
use crate::io::reader::file_reader::FileReader;
use crate::io::reader::stdin::StdinReader;
use crate::io::routing::{Source, Target};
use crate::io::writer::Writer;
use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufWriter;
use tempfile::TempPath;

pub struct IoSetup {
    pub reader: Reader,
    pub writer: Writer,
    pub presenter: Presenter,
}

pub fn initialize_io(source: Source, target: Target) -> Result<IoSetup> {
    let reader = get_reader(source)?;
    let (writer, presenter) = get_writer_and_presenter(target)?;

    Ok(IoSetup {
        reader,
        writer,
        presenter,
    })
}

fn get_reader(input: Source) -> Result<Reader> {
    let reader = match input {
        Source::File(file) => Reader::File(FileReader::new(file.path, file.terminate_after_first_read)?),
        Source::Stdin => Reader::Stdin(StdinReader::new()),
        Source::Command(cmd) => Reader::Command(CommandReader::new(cmd)?),
    };

    Ok(reader)
}

fn get_writer_and_presenter(output: Target) -> Result<(Writer, Presenter)> {
    let pager_opts = match output {
        Target::Less(opts) => PagerOptions::Less(opts),
        Target::CustomPager(opts) => PagerOptions::Custom(opts),
        Target::Stdout => return Ok((Writer::Stdout(StdoutWriter::new()), Presenter::Stdout)),
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

    Ok((path, BufWriter::new(file)))
}
