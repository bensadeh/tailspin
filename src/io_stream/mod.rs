use async_trait::async_trait;
use linemux::MuxedLines;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot::Sender;
use tokio::{fs, io};

pub struct IoStream<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> IoStream<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    pub async fn next_line(&mut self) -> io::Result<Option<String>>
    where
        R: AsyncLineReader + Unpin,
    {
        self.reader.next_line().await
    }

    pub async fn write_line(&mut self, line: &str) -> io::Result<()>
    where
        W: AsyncLineWriter + Unpin,
    {
        self.writer.write_line(line).await
    }
}

#[async_trait]
pub trait AsyncLineReader: Unpin {
    async fn next_line(&mut self) -> io::Result<Option<String>>;
}

#[async_trait]
pub trait AsyncLineWriter: Unpin {
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}

pub struct TailFileIoStream<W: AsyncLineWriter> {
    io_stream: IoStream<MuxedLinesWrapper, W>,
    line_count: usize,
    reached_eof_tx: Option<Sender<()>>,
    current_line: usize,
}

impl<W: AsyncLineWriter> TailFileIoStream<W> {
    pub async fn new(
        file_path: &str,
        writer: W,
        line_count: usize,
        reached_eof_tx: Option<Sender<()>>,
    ) -> io::Result<Self> {
        let mut lines = MuxedLines::new()?;
        dbg!(file_path.clone());
        lines.add_file_from_start(file_path).await?;
        let reader = MuxedLinesWrapper(lines);

        Ok(Self {
            io_stream: IoStream::new(reader, writer),
            line_count,
            reached_eof_tx,
            current_line: 1,
        })
    }

    pub async fn next_line(&mut self) -> io::Result<Option<String>> {
        let line = self.io_stream.next_line().await?;
        dbg!(line.clone());

        if self.current_line == self.line_count {
            if let Some(reached_eof) = self.reached_eof_tx.take() {
                reached_eof
                    .send(())
                    .expect("Failed sending EOF signal to oneshot channel");
            }
        }
        self.current_line += 1;

        Ok(line)
    }

    pub async fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.io_stream.write_line(line).await
    }
}

pub struct MuxedLinesWrapper(MuxedLines);

#[async_trait]
impl AsyncLineReader for MuxedLinesWrapper {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        self.0
            .next_line()
            .await
            .map(|opt| opt.map(|line| line.line().to_string()))
    }
}

#[async_trait]
impl AsyncLineWriter for io::BufWriter<fs::File> {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.write_all(line.as_bytes()).await?;
        self.write_all(b"\n").await?;
        self.flush().await
    }
}
