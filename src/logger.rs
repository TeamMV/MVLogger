use std::io::Write;
use std::sync::Mutex;
use log::{Log, Metadata, Record};
use mvutils::utils::{Recover, Time};
use crate::time::convert_epoch_to_datetime;

pub(crate) struct Logger {
    output: Box<Mutex<dyn Write + 'static>>
}

impl Logger {
    pub(crate) fn new(output: impl Write + 'static) -> Self {
        Logger {
            output: Box::new(Mutex::new(output)),
        }
    }
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        self.output.lock().recover().write_all(format!("[{} UTC] <{}> {}\n", convert_epoch_to_datetime(u128::time_millis()), record.metadata().level(), record.args()).as_bytes()).unwrap()
    }

    fn flush(&self) {
        self.output.lock().recover().flush().unwrap()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush()
    }
}

unsafe impl Sync for Logger {}
unsafe impl Send for Logger {}