use log::*;

use crate::Hexgem::level::ColorLog;

pub struct HexgemLogger;

impl log::Log for HexgemLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let color = &level.format_color();
            let args = record.args();
            let path = record.module_path();
            let code_line = record.line().unwrap_or(0);
            match path {
                Some(mod_path) => {
                    let client = mod_path.get_client();

                    match level {
                        Level::Error | Level::Warn => {
                            println!(
                                "{} - {}: {} \x1b[90;3mat {}:{}",
                                color, client, args, mod_path, code_line
                            )
                        }
                        _ => {
                            println!("{} - {}: {}", color, client, args)
                        }
                    }
                }
                None => println!("{} - {}", color, args),
            }
        }
    }

    fn flush(&self) {}
}

impl HexgemLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        println!("\x1b[2J\x1b[H");
        log::set_logger(&HexgemLogger).map(|()| log::set_max_level(LevelFilter::Debug))
    }
}
trait Client {
    fn get_client(&self) -> &str;
}

impl Client for &str {
    fn get_client(&self) -> &'static str {
        let parts = self.split("::").collect::<Vec<&str>>();
        if parts[0] == "hexgem_engine" {
            return "\x1b[36;4mHexgem engine\x1b[0m";
        } else {
            return "\x1b[36;4mHexgem application\x1b[0m";
        }
    }
}
