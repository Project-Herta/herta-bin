use std::fmt::Display;

use fern_colored::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::{Level, Record};

pub fn setup() {
    let color_config = ColoredLevelConfig::new()
        .trace(Color::Green)
        .debug(Color::Magenta)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    Dispatch::new()
        .format(move |out, msg, record| {
            out.finish(format_args!(
                "\x1B[{}m[{}:{}]\x1B[0m [{}] {}",
                Color::Black.to_fg_str(),
                record.target(),
                record.line().unwrap_or(u32::MAX),
                wrap_color(record, &color_config),
                msg,
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("html5ever", log::LevelFilter::Off)
        .level_for("selectors", log::LevelFilter::Off)
        .level_for("reqwest", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .expect("Expected for logger to be created");
}

fn wrap_color(record: &Record, color_config: &ColoredLevelConfig) -> String {
    let level_str = record.level().to_string().chars().take(1).last().unwrap();
    let level_color = color_config.get_color(&record.level());

    format!("\x1B[{}m{}\x1B[0m", level_color.to_fg_str(), level_str)
}
