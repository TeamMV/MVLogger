mod buffered;
mod logger;
mod time;

use crate::buffered::BufferWriter;
use crate::logger::Logger;
use log::LevelFilter;
use mvutils::utils::Recover;
use mvutils::{create_once, lazy};
use std::io::Write;
use std::sync::Mutex;

create_once! {
    static LOGGER: Logger;
}

lazy! {
    static BUFFER: Mutex<String> = String::new().into();
}

pub fn init(output: impl Write + 'static, max_level: LevelFilter) {
    LOGGER
        .try_create(|| Logger::new(output))
        .expect("Logger is already initialized!");
    log::set_logger(&*LOGGER).expect("Logger is already set!");
    log::set_max_level(max_level);
}

pub fn init_buffered(max_level: LevelFilter) {
    init(BufferWriter, max_level)
}

pub fn get_buffer() -> String {
    BUFFER.lock().recover().clone()
}

pub fn clear_buffer() -> String {
    BUFFER.lock().recover().drain(..).collect()
}

#[cfg(test)]
mod tests {
    use crate::init;
    use log::{debug, error, info, trace, warn, LevelFilter, Log};

    #[test]
    fn it_works() {
        init(std::io::stdout(), LevelFilter::Trace);

        trace!("Syscall!");
        debug!("Hello! {}", 10);
        info!("Hi!");
        warn!("Oops!");
        error!("Woah!");
    }
}
