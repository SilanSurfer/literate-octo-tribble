use env_logger::fmt::Color;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn configure_logger(verbose: u8) {
    let level_filter = match verbose {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| {
            let mut level_style = buf.style();
            let mut args_style = buf.style();
            match record.level() {
                log::Level::Error => {
                    level_style.set_color(Color::Red).set_bold(true);
                    args_style.set_color(Color::Red);
                }
                log::Level::Warn => {
                    level_style.set_color(Color::Yellow);
                    args_style.set_color(Color::Yellow);
                }
                log::Level::Info => {
                    level_style.set_color(Color::Green);
                }
                log::Level::Debug => {
                    level_style.set_color(Color::Blue);
                }
                _ => {}
            }
            writeln!(
                buf,
                "[{}] - {}",
                level_style.value(record.level()),
                args_style.value(record.args())
            )
        })
        .filter(None, level_filter)
        .init();
}
