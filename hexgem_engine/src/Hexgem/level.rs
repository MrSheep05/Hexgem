use log::Level;

pub trait ColorLog {
    fn format_color(&self) -> String;
}

impl ColorLog for Level {
    fn format_color(&self) -> String {
        return match &self {
            Level::Error => format!("\x1b[31;1m[{}]\x1b[0m", &self),
            Level::Warn => format!("\x1b[33;1m[{}]\x1b[0m", &self),
            Level::Info => format!("\x1b[34;1m[{}]\x1b[0m", &self),
            _ => format!("\x1b[32;1m[{}]\x1b[0m", &self),
        };
    }
}
