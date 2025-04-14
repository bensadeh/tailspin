use miette::{IntoDiagnostic, Result};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Reads all **currently available** complete lines from `reader`.
/// If there’s no newline in the buffer, it will block until
/// at least one newline is available (or until EOF).
/// Leaves any incomplete trailing line in the buffer.
pub async fn read_complete_lines<R>(reader: &mut R) -> Result<Vec<String>>
where
    R: AsyncBufRead + Unpin,
{
    let mut lines = Vec::new();

    loop {
        // 1) Peek at the buffered data without consuming it
        let buffer = reader.fill_buf().await.into_diagnostic()?;
        if buffer.is_empty() {
            // EOF reached; no more data can arrive
            // Optionally, if you want to handle a trailing partial line at EOF,
            // you could return it here. For example:
            //
            // if !lines.is_empty() {
            //     // We already have some lines; return them
            //     return Ok(lines);
            // }
            // // Or if you want to gather partial line:
            // // lines.push("<partial line>".to_string());
            //
            return Ok(lines);
        }

        // 2) Check how many newlines are present in the buffer
        let newline_count = buffer.iter().filter(|&&b| b == b'\n').count();
        if newline_count == 0 {
            // 3) If no newline is in the buffer, try to read more data.
            //    Using read_until() with b'\n' will block until a newline
            //    or EOF. That will add more data into the buffer (internally).
            let mut discard = Vec::new();
            let read = reader.read_until(b'\n', &mut discard).await.into_diagnostic()?;
            if read == 0 {
                // EOF (no more bytes). Return whatever lines we have so far.
                return Ok(lines);
            }
            // Now that we (likely) have at least one newline, loop again
            // to parse out the newly complete lines.
            continue;
        } else {
            // 4) Extract all lines up to the last `\n` within the buffer
            let mut consumed = 0;
            while let Some(pos) = buffer[consumed..].iter().position(|&b| b == b'\n') {
                // Everything up to `pos` is a complete line
                let line_end = consumed + pos;
                let line_bytes = &buffer[consumed..line_end];
                let line_str = String::from_utf8_lossy(line_bytes).to_string();
                lines.push(line_str);

                // Skip the '\n'
                consumed = line_end + 1;
            }

            // 5) Consume only the bytes that formed complete lines
            //    so that any partial line is left in the internal buffer.
            reader.consume(consumed);

            // Return all lines we extracted in this pass
            return Ok(lines);
        }
    }
}
