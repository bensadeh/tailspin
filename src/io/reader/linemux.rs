use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use linemux::MuxedLines;
use std::cmp::min;
use std::io;
use terminal_size::{terminal_size, Height, Width};
use tokio::sync::oneshot::Sender;

pub struct Linemux {
    startup_message: Option<String>,
    number_of_lines: Option<usize>,
    bucket_size: usize,
    current_line: usize,
    reached_eof_tx: Option<Sender<()>>,
    reached_eof: bool,
    lines: MuxedLines,
}

impl Linemux {
    pub async fn get_reader_single(
        file_path: String,
        number_of_lines: usize,
        start_at_end: bool,
        mut reached_eof_tx: Option<Sender<()>>,
    ) -> Box<dyn AsyncLineReader + Send> {
        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        if start_at_end {
            lines.add_file(&file_path).await.expect("Could not add file to linemux");

            if let Some(reached_eof) = reached_eof_tx.take() {
                reached_eof
                    .send(())
                    .expect("Failed sending EOF signal to oneshot channel");
            }
        } else {
            lines
                .add_file_from_start(&file_path)
                .await
                .expect("Could not add file to linemux");
        }

        let bucket_size = min(number_of_lines - 1, 10000);

        Box::new(Self {
            startup_message: None,
            number_of_lines: Some(number_of_lines),
            bucket_size,
            current_line: 0,
            reached_eof_tx,
            reached_eof: false,
            lines,
        })
    }

    pub async fn get_reader_multiple(
        folder_name: String,
        file_paths: Vec<String>,
        mut reached_eof_tx: Option<Sender<()>>,
    ) -> Box<dyn AsyncLineReader + Send> {
        use std::path::Path;

        if let Some(reached_eof) = reached_eof_tx.take() {
            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }

        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        let file_list = file_paths
            .iter()
            .enumerate()
            .map(|(index, path)| {
                let file_name = Path::new(path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or(path);

                if index == file_paths.len() - 1 {
                    format!("         └─ {}", file_name.bold())
                } else {
                    format!("         ├─ {}", file_name.bold())
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let separator = get_separator();
        let dimmed_separator = separator.dimmed();
        let startup_message = format!(
            "Watching {} \n{}\n{}\n",
            folder_name.green(),
            file_list,
            dimmed_separator
        );

        for file_path in file_paths {
            lines.add_file(&file_path).await.expect("Could not add file to linemux");
        }

        Box::new(Self {
            startup_message: Some(startup_message),
            number_of_lines: None,
            bucket_size: 1,
            current_line: 0,
            reached_eof_tx,
            reached_eof: true,
            lines,
        })
    }

    fn send_eof_signal(&mut self) {
        if let Some(reached_eof) = self.reached_eof_tx.take() {
            self.reached_eof = true;

            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }
    }

    async fn read_lines_until_eof(&mut self) -> io::Result<Option<Vec<String>>> {
        let mut bucket = Vec::new();
        let number_of_lines = self.number_of_lines.expect("Number of lines not set");

        while bucket.len() < self.bucket_size {
            let line = match self.lines.next_line().await {
                Ok(Some(line)) => line,
                _ => break,
            };

            let next_line = line.line().to_owned();
            bucket.push(next_line);
            self.current_line += 1;

            if self.current_line >= number_of_lines {
                self.send_eof_signal();
                self.bucket_size = 1;
            }
        }

        if bucket.is_empty() {
            Ok(None)
        } else {
            Ok(Some(bucket))
        }
    }

    async fn read_line_by_line(&mut self) -> io::Result<Option<Vec<String>>> {
        let line = match self.lines.next_line().await {
            Ok(Some(line)) => line,
            _ => return Ok(None),
        };

        let next_line = line.line().to_owned();

        Ok(Some(vec![next_line]))
    }
}

fn get_separator() -> String {
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        "▁".repeat(w as usize)
    } else {
        "".to_string()
    }
}

#[async_trait]
impl AsyncLineReader for Linemux {
    async fn next_line_batch(&mut self) -> io::Result<Option<Vec<String>>> {
        if let Some(custom_message) = self.startup_message.take() {
            return Ok(Some(vec![custom_message]));
        }

        match self.reached_eof {
            true => self.read_line_by_line().await,
            false => self.read_lines_until_eof().await,
        }
    }
}
