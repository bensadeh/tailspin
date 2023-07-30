// use crate::io_stream::traits::AsyncLineReader;
// use async_trait::async_trait;
// use linemux::MuxedLines;
// use std::fs::File;
// use std::io;
// use std::io::{BufRead, BufReader};
// use std::path::Path;
// use tokio::sync::oneshot::Sender;
//
// pub struct LinemuxReader {
//     file_path: String,
//     number_of_lines: usize,
//     current_line: usize,
//     reached_eof_tx: Option<Sender<()>>,
//     lines: MuxedLines,
// }
//
// impl LinemuxReader {
//     pub async fn new(
//         file_path: String,
//         number_of_lines: usize,
//         reached_eof_tx: Option<Sender<()>>,
//     ) -> io::Result<Self> {
//         let mut lines = MuxedLines::new()?;
//         lines.add_file_from_start(&file_path).await?;
//
//         Ok(Self {
//             file_path,
//             number_of_lines,
//             current_line: 1,
//             reached_eof_tx,
//             lines,
//         })
//     }
// }
//
// async fn count_lines(file_path: &str) -> io::Result<usize> {
//     let file_path = file_path.to_owned();
//
//     let line_count = tokio::task::spawn_blocking(move || {
//         let path = Path::new(&file_path);
//         let file = File::open(&path).expect("Could not open file");
//         let reader = BufReader::new(file);
//         reader.lines().count()
//     })
//     .await
//     .unwrap();
//
//     Ok(line_count)
// }
//
// #[async_trait]
// impl AsyncLineReader for LinemuxReader {
//     async fn next_line(&mut self) -> io::Result<Option<String>> {
//         if let Ok(Some(line)) = self.lines.next_line().await {
//             if self.current_line == self.number_of_lines {
//                 if let Some(reached_eof) = self.reached_eof_tx.take() {
//                     reached_eof
//                         .send(())
//                         .expect("Failed sending EOF signal to oneshot channel");
//                 }
//             }
//             self.current_line += 1;
//             return Ok(Some(line.line().to_owned()));
//         }
//
//         Ok(None)
//     }
// }
