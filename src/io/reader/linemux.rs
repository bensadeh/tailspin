use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use linemux::MuxedLines;
use std::io;
use terminal_size::{terminal_size, Height, Width};
use tokio::sync::oneshot::Sender;

pub struct Linemux {
    custom_message: Option<String>,
    number_of_lines: Option<usize>,
    current_line: usize,
    reached_eof_tx: Option<Sender<()>>,
    lines: MuxedLines,
}

impl Linemux {
    pub async fn get_reader_single(
        file_path: String,
        number_of_lines: usize,
        follow: bool,
        tail: bool,
        mut reached_eof_tx: Option<Sender<()>>,
    ) -> Box<dyn AsyncLineReader + Send> {
        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        if tail {
            if let Some(reached_eof) = reached_eof_tx.take() {
                reached_eof
                    .send(())
                    .expect("Failed sending EOF signal to oneshot channel");
            }

            lines
                .add_file(&file_path)
                .await
                .expect("Could not add file to linemux");
        } else {
            lines
                .add_file_from_start(&file_path)
                .await
                .expect("Could not add file to linemux");
        }

        let number_of_lines = if follow {
            Some(1)
        } else {
            Some(number_of_lines)
        };

        Box::new(Self {
            custom_message: None,
            number_of_lines,
            current_line: 1,
            reached_eof_tx,
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
        let custom_message = format!(
            "Watching {} \n{}\n{}\n",
            folder_name.green(),
            file_list,
            dimmed_separator
        );

        for file_path in file_paths {
            lines
                .add_file(&file_path)
                .await
                .expect("Could not add file to linemux");
        }

        Box::new(Self {
            custom_message: Some(custom_message),
            number_of_lines: None,
            current_line: 1,
            reached_eof_tx,
            lines,
        })
    }

    fn send_eof_signal(&mut self) {
        if let Some(reached_eof) = self.reached_eof_tx.take() {
            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }
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
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        if let Some(custom_message) = self.custom_message.take() {
            return Ok(Some(custom_message));
        }

        let line = match self.lines.next_line().await {
            Ok(Some(line)) => line,
            _ => return Ok(None),
        };

        if let Some(number_of_lines) = self.number_of_lines {
            if self.current_line == number_of_lines {
                self.send_eof_signal();
            }
        }

        self.current_line += 1;
        Ok(Some(line.line().to_owned()))
    }
}
