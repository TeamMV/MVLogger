mod logger;
mod time;

use std::io::Write;
use log::LevelFilter;
use mvutils::create_once;
use crate::logger::Logger;

create_once! {
    static LOGGER: Logger;
}

pub fn init(output: impl Write + 'static, max_level: LevelFilter) {
    LOGGER.try_create(|| Logger::new(output)).expect("Logger is already initialized!");
    log::set_logger(&*LOGGER).expect("Logger is already set!");
    log::set_max_level(max_level);
}

#[cfg(test)]
mod tests {
    use log::{debug, error, info, LevelFilter, Log, trace, warn};
    use crate::init;

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
