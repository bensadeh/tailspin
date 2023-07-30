// use crate::io_stream::traits::AsyncLineWriter;
// use async_trait::async_trait;
// use rand::random;
// use std::path::PathBuf;
// use tempfile::TempDir;
// use tokio::fs::File;
// use tokio::io;
// use tokio::io::{AsyncWriteExt, BufWriter};
//
// pub struct TempFileWriter {
//     temp_dir: TempDir,
//     output_path: PathBuf,
//     output_writer: BufWriter<File>,
// }
//
// impl TempFileWriter {
//     pub async fn new() -> Self {
//         let (temp_dir, output_path, output_writer) = create_temp_file().await;
//         Self {
//             temp_dir,
//             output_path,
//             output_writer,
//         }
//     }
// }
//
// #[async_trait]
// impl AsyncLineWriter for TempFileWriter {
//     async fn write_line(&mut self, line: &str) -> io::Result<()> {
//         self.output_writer.write_all(line.as_bytes()).await?;
//         self.output_writer.flush().await?;
//
//         Ok(())
//     }
// }
//
// async fn create_temp_file() -> (TempDir, PathBuf, BufWriter<File>) {
//     let unique_id: u32 = random();
//     let filename = format!("tailspin.temp.{}", unique_id);
//
//     let temp_dir = tempfile::tempdir().unwrap();
//
//     let output_path = temp_dir.path().join(filename);
//     let output_file = File::create(&output_path).await.unwrap();
//     let output_writer = BufWriter::new(output_file);
//
//     (temp_dir, output_path, output_writer)
// }
