use crate::BUFFER;
use mvutils::utils::Recover;
use std::io::Write;

pub(crate) struct BufferWriter;

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        BUFFER
            .lock()
            .recover()
            .push_str(&String::from_utf8_lossy(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
