pub mod command;
pub mod file_reader;
mod line_batcher;
pub mod stdin;

use crate::io::reader::command::CommandReader;
use crate::io::reader::file_reader::FileReader;
use crate::io::reader::stdin::StdinReader;
use anyhow::Result;
use shared_child::SharedChild;
use std::sync::Arc;

pub use line_batcher::LineBatch;

pub enum Reader {
    File(FileReader),
    Stdin(StdinReader),
    Command(CommandReader),
}

/// Events produced by [`Reader::next`].
#[derive(Debug)]
pub enum StreamEvent {
    /// Emitted exactly once, always before `Ended`. File readers send it
    /// after draining the content that existed at startup; stdin and command
    /// readers send it immediately. The pager path spawns its pager on this
    /// event, so a file's existing content is fully written to the temp file
    /// before the pager opens, while `--exec` output streams in live.
    InitialReadComplete,

    /// The stream is exhausted; no more events follow.
    Ended,

    /// A batch of complete lines to highlight.
    Lines(LineBatch),
}

impl Reader {
    pub fn next(&mut self) -> Result<StreamEvent> {
        match self {
            Reader::File(r) => r.next(),
            Reader::Stdin(r) => r.next(),
            Reader::Command(r) => r.next(),
        }
    }

    /// Handle for killing an `--exec` child from another thread.
    pub fn exec_child(&self) -> Option<Arc<SharedChild>> {
        match self {
            Reader::Command(r) => Some(r.child()),
            Reader::File(_) | Reader::Stdin(_) => None,
        }
    }
}
