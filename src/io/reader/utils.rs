use miette::{Context, IntoDiagnostic, Result};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pub async fn read_bytes_until_newline<R>(reader: &mut R) -> Result<Vec<u8>>
where
    R: AsyncBufRead + Unpin,
{
    let mut buffer = Vec::new();

    reader
        .read_until(b'\n', &mut buffer)
        .await
        .into_diagnostic()
        .wrap_err("Could not read from stream")?;

    Ok(buffer)
}

pub fn strip_newline_character(mut buffer: Vec<u8>) -> Vec<u8> {
    if buffer.last() == Some(&b'\n') {
        buffer.pop();
    }

    buffer
}
