use crate::time::convert_epoch_to_datetime;
use log::{Log, Metadata, Record};
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
            self.output
                .lock()
                .recover()
                .write_all(
                    format!(
                        "[{} UTC] <{}> {}\n",
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
