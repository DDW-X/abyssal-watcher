use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logger() {
    Builder::new()
        .format(|buf: &mut env_logger::fmt::Formatter, record: &log::Record| {
            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
