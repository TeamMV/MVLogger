use crate::time::convert_epoch_to_datetime;
use log::{Level, Log, Metadata, Record};
use mvutils::utils::{Recover, Time};
use std::io::Write;
use std::sync::Mutex;

pub(crate) struct Logger {
    output: Box<Mutex<dyn Write + 'static>>,
    formatted: bool,
}

impl Logger {
    pub(crate) fn new(output: impl Write + 'static, formatted: bool) -> Self {
        Logger {
            output: Box::new(Mutex::new(output)),
            formatted,
        }
    }
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.formatted {
            let color = match record.level() {
                Level::Error => "\x1B[91m",
                Level::Warn => "\x1B[93m",
                Level::Info => "\x1B[0m",
                Level::Debug => "\x1B[37m",
                Level::Trace => "\x1B[90m",
            };
            let reset = "\x1B[0m";
            self.output
                .lock()
                .recover()
                .write_all(
                    format!(
                        "{color}[{} UTC] <{}> {}{reset}\n",
                        convert_epoch_to_datetime(u128::time_millis()),
                        record.metadata().level(),
                        record.args()
                    )
                    .as_bytes(),
                )
                .unwrap()
        } else {
            self.output
                .lock()
                .recover()
                .write_all(format!("{}\n", record.args()).as_bytes())
                .unwrap()
        }
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
