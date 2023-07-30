// mod linemux_reader;
// mod temp_file_writer;
// mod template_io;
// mod traits;
//
// use crate::io_stream::traits::{AsyncLineReader, AsyncLineWriter};
//
// use async_trait::async_trait;
// use linemux::MuxedLines;
// use tokio::io::AsyncWriteExt;
// use tokio::sync::oneshot::Sender;
// use tokio::{fs, io};
//
// pub use template_io::TemplateIOStream;
// pub use traits::LineIOStream;

// pub struct TailFileIoStream<W: AsyncLineWriter> {
//     reader: MuxedLinesWrapper,
//     writer: W,
//     line_count: usize,
//     reached_eof_tx: Option<Sender<()>>,
//     current_line: usize,
// }
//
// #[async_trait]
// impl<W: AsyncLineWriter + Send> LineIOStream for TailFileIoStream<W> {
//     async fn next_line(&mut self) -> io::Result<Option<String>> {
//         let line = self.reader.next_line().await?;
//
//         if self.current_line == self.line_count {
//             if let Some(reached_eof) = self.reached_eof_tx.take() {
//                 reached_eof
//                     .send(())
//                     .expect("Failed sending EOF signal to oneshot channel");
//             }
//         }
//         self.current_line += 1;
//
//         Ok(line)
//     }
//
//     async fn write_line(&mut self, line: &str) -> io::Result<()> {
//         self.writer.write_line(line).await
//     }
// }
//
// impl<W: AsyncLineWriter + Unpin + Send> TailFileIoStream<W> {
//     pub async fn new(
//         file_path: &str,
//         writer: W,
//         line_count: usize,
//         reached_eof_tx: Option<Sender<()>>,
//     ) -> io::Result<Self> {
//         let mut lines = MuxedLines::new()?;
//         dbg!(file_path.clone());
//         lines.add_file_from_start(file_path).await?;
//         let reader = MuxedLinesWrapper(lines);
//
//         Ok(Self {
//             reader,
//             writer,
//             line_count,
//             reached_eof_tx,
//             current_line: 1,
//         })
//     }
// }
//
// pub struct MuxedLinesWrapper(MuxedLines);
//
// #[async_trait]
// impl AsyncLineReader for MuxedLinesWrapper {
//     async fn next_line(&mut self) -> io::Result<Option<String>> {
//         self.0
//             .next_line()
//             .await
//             .map(|opt| opt.map(|line| line.line().to_string()))
//     }
// }
//
// #[async_trait]
// impl AsyncLineWriter for io::BufWriter<fs::File> {
//     async fn write_line(&mut self, line: &str) -> io::Result<()> {
//         self.write_all(line.as_bytes()).await?;
//         self.write_all(b"\n").await?;
//         self.flush().await
//     }
// }
