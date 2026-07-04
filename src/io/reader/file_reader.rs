use crate::io::reader::StreamEvent;
use crate::io::reader::StreamEvent::{Ended, InitialReadComplete};
use crate::io::reader::line_batcher::{BUF_READER_CAPACITY, ReadResult, decode_line, read_lines};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::thread;
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_millis(100);

enum Stage {
    InitialRead,
    Following,
    Terminated,
}

pub struct FileReader {
    reader: BufReader<File>,
    buf: Vec<u8>,
    stage: Stage,
    terminate_after_first_read: bool,
}

impl FileReader {
    pub fn new<P: AsRef<Path>>(file_path: P, terminate_after_first_read: bool) -> Result<FileReader> {
        let file_path = std::fs::canonicalize(file_path.as_ref()).context("Could not canonicalize file path")?;

        let file = File::open(&file_path).context("Could not open file")?;

        let reader = BufReader::with_capacity(BUF_READER_CAPACITY, file);

        Ok(Self {
            reader,
            buf: Vec::new(),
            stage: Stage::InitialRead,
            terminate_after_first_read,
        })
    }

    fn next_line(&mut self) -> Result<String> {
        loop {
            let bytes_read = self
                .reader
                .read_until(b'\n', &mut self.buf)
                .context("Could not read next line")?;

            if bytes_read == 0 {
                // Detect file truncation: if the file shrank past our position, restart from the beginning
                let file_size = self.reader.get_ref().metadata().context("Could not stat file")?.len();
                let position = self.reader.stream_position().context("Could not get stream position")?;

                if file_size < position {
                    self.reader
                        .seek(SeekFrom::Start(0))
                        .context("Could not seek to start after truncation")?;
                    self.buf.clear();
                }

                thread::sleep(POLL_INTERVAL);
                continue;
            }

            if self.buf.ends_with(b"\n") {
                let line = decode_line(&self.buf);
                self.buf.clear();
                return Ok(line);
            }

            // Partial line at EOF — wait for more data
            thread::sleep(POLL_INTERVAL);
        }
    }

    pub fn next(&mut self) -> Result<StreamEvent> {
        match self.stage {
            Stage::InitialRead => match read_lines(&mut self.reader)? {
                ReadResult::Batch(lines) => Ok(StreamEvent::Lines(lines)),
                ReadResult::Eof => {
                    self.stage = if self.terminate_after_first_read {
                        Stage::Terminated
                    } else {
                        Stage::Following
                    };
                    Ok(InitialReadComplete)
                }
            },
            Stage::Following => Ok(StreamEvent::Lines(vec![self.next_line()?])),
            Stage::Terminated => Ok(Ended),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::reader::StreamEvent::*;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
    use tempfile::tempdir;

    /// Drives the reader on a background thread so tests can assert on
    /// blocking behavior with timeouts. The thread ends at `Ended` or on
    /// error; a reader still blocked in follow mode dies with the test
    /// process.
    fn events_of(mut reader: FileReader) -> Receiver<Result<StreamEvent>> {
        let (tx, rx) = channel();
        thread::spawn(move || {
            loop {
                let event = reader.next();
                let done = matches!(event, Ok(Ended) | Err(_));
                if tx.send(event).is_err() || done {
                    break;
                }
            }
        });
        rx
    }

    fn next_event(events: &Receiver<Result<StreamEvent>>) -> StreamEvent {
        events
            .recv_timeout(Duration::from_secs(5))
            .expect("timed out waiting for an event")
            .expect("reader errored")
    }

    fn assert_no_event(events: &Receiver<Result<StreamEvent>>) {
        match events.recv_timeout(Duration::from_millis(200)) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected no event, got {other:?}"),
        }
    }

    #[test]
    fn test_read_exactly_n_lines() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.log");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "line1").unwrap();
        writeln!(file, "line2").unwrap();
        writeln!(file, "line3").unwrap();

        let events = events_of(FileReader::new(file_path, false)?);

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["line1", "line2", "line3"]),
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }
        assert!(matches!(next_event(&events), InitialReadComplete));

        assert_no_event(&events);

        Ok(())
    }

    #[test]
    fn test_terminate_after_first_read() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.log");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "only_line").unwrap();

        let mut reader = FileReader::new(file_path, true)?;

        match reader.next()? {
            Lines(lines) => assert_eq!(lines, vec!["only_line"]),
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }
        assert!(matches!(reader.next()?, InitialReadComplete));
        assert!(matches!(reader.next()?, Ended));

        Ok(())
    }

    #[test]
    fn test_append_new_lines_after_initial_read() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_append.log");

        let mut file = File::create(&file_path)?;
        writeln!(file, "initial1")?;
        writeln!(file, "initial2")?;

        let events = events_of(FileReader::new(file_path.as_path(), false)?);

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["initial1", "initial2"]),
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }
        assert!(matches!(next_event(&events), InitialReadComplete));

        let mut file = OpenOptions::new().append(true).open(&file_path)?;
        writeln!(file, "appended1")?;
        writeln!(file, "appended2")?;

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["appended1"]),
            other => panic!("Expected appended1, got {other:?}"),
        }
        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["appended2"]),
            other => panic!("Expected appended2, got {other:?}"),
        }

        Ok(())
    }

    #[test]
    fn test_empty_file() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.log");
        File::create(&file_path).unwrap();

        let mut reader = FileReader::new(file_path, true)?;

        assert!(matches!(reader.next()?, InitialReadComplete));
        assert!(matches!(reader.next()?, Ended));

        Ok(())
    }

    #[test]
    fn test_no_trailing_newline() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("no_trailing.log");

        let mut file = File::create(&file_path).unwrap();
        write!(file, "line1\nline2").unwrap();

        let mut reader = FileReader::new(file_path, true)?;

        let mut all_lines = Vec::new();
        while let Lines(lines) = reader.next()? {
            all_lines.extend(lines);
        }
        assert_eq!(all_lines, vec!["line1", "line2"]);

        Ok(())
    }

    #[test]
    fn test_crlf_line_endings() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("crlf.log");

        {
            let mut file = File::create(&file_path)?;
            file.write_all(b"line1\r\nline2\r\n")?;
        }

        let events = events_of(FileReader::new(file_path.as_path(), false)?);

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["line1", "line2"]),
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }
        assert!(matches!(next_event(&events), InitialReadComplete));

        // Append a CRLF line in follow mode
        {
            let mut file = OpenOptions::new().append(true).open(&file_path)?;
            file.write_all(b"appended\r\n")?;
        }

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["appended"]),
            other => panic!("Expected StreamEvent::Lines(\"appended\"), got {other:?}"),
        }

        Ok(())
    }

    #[test]
    fn test_non_utf8_content() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("non_utf8.log");

        {
            let mut file = File::create(&file_path).unwrap();
            // Write invalid UTF-8: 0xFF 0xFE are not valid UTF-8 byte sequences
            file.write_all(b"hello \xff\xfe world\n").unwrap();
        }

        let mut reader = FileReader::new(file_path, true)?;

        match reader.next()? {
            Lines(lines) => {
                assert_eq!(lines.len(), 1);
                assert!(lines[0].contains("hello"));
                assert!(lines[0].contains("world"));
                assert!(lines[0].contains('\u{FFFD}'));
            }
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }

        Ok(())
    }

    #[test]
    fn test_non_utf8_in_follow_mode() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("non_utf8_follow.log");

        {
            let mut file = File::create(&file_path)?;
            writeln!(file, "initial")?;
        }

        let events = events_of(FileReader::new(file_path.as_path(), false)?);

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["initial"]),
            other => panic!("Expected StreamEvent::Lines(\"initial\"), got {other:?}"),
        }
        assert!(matches!(next_event(&events), InitialReadComplete));

        // Append non-UTF-8 in follow mode
        {
            let mut file = OpenOptions::new().append(true).open(&file_path)?;
            file.write_all(b"caf\xe9\n")?;
        }

        match next_event(&events) {
            Lines(lines) => {
                assert_eq!(lines.len(), 1);
                assert!(lines[0].starts_with("caf"));
                assert!(lines[0].contains('\u{FFFD}'));
            }
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }

        Ok(())
    }

    #[test]
    fn test_truncation_detection() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("truncate.log");

        let mut file = File::create(&file_path)?;
        writeln!(file, "original1")?;
        writeln!(file, "original2")?;

        let events = events_of(FileReader::new(file_path.as_path(), false)?);

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["original1", "original2"]),
            other => panic!("Expected StreamEvent::Lines(...), got {other:?}"),
        }
        assert!(matches!(next_event(&events), InitialReadComplete));

        // Truncate the file and write new, shorter content
        let mut file = File::create(&file_path)?;
        writeln!(file, "new")?;

        match next_event(&events) {
            Lines(lines) => assert_eq!(lines, vec!["new"]),
            other => panic!("Expected StreamEvent::Lines(\"new\"), got {other:?}"),
        }

        Ok(())
    }

    #[test]
    fn test_large_file_streams_in_batches() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large.log");

        {
            let mut file = File::create(&file_path).unwrap();
            for i in 0..2000 {
                writeln!(
                    file,
                    "line {i:05} - padding to make this line reasonably long for testing"
                )
                .unwrap();
            }
        }

        let mut reader = FileReader::new(file_path, true)?;

        let mut event_count = 0;
        let mut total_lines = 0;

        loop {
            match reader.next()? {
                Lines(lines) => {
                    event_count += 1;
                    total_lines += lines.len();
                }
                InitialReadComplete => break,
                Ended => panic!("Unexpected Ended before InitialReadComplete"),
            }
        }

        assert_eq!(total_lines, 2000);
        assert!(
            event_count > 1,
            "Large file should produce multiple events, got {event_count}"
        );

        Ok(())
    }
}
