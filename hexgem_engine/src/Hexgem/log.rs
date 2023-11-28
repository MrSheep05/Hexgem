use crate::Hexgem::level::ColorLog;
use log::*;
use std::{panic, path::Path};

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
            let module = record.module_path();
            let file_path = {
                let target = record.target();
                let source: Vec<&str> = target.split(":").collect();
                if Path::new(source[0]).is_file() {
                    Some(String::from(target))
                } else {
                    match (record.file(), record.line()) {
                        (Some(file_path), Some(line)) => {
                            Some([file_path, &line.to_string()].join(":"))
                        }
                        _ => None,
                    }
                }
            };
            match (module, file_path) {
                (Some(mod_path), Some(path)) => {
                    let client = mod_path.get_client();

                    match level {
                        Level::Error | Level::Warn => {
                            println!(
                                "{} - {}: {} \x1b[90;3min {} at {}",
                                color, client, args, path, mod_path
                            )
                        }
                        _ => {
                            println!("{} - {}: {}", color, client, args)
                        }
                    }
                }
                (_, _) => println!("{} - {}", color, args),
            }
        }
    }

    fn flush(&self) {}
}

impl HexgemLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        panic::set_hook({
            Box::new(move |info| {
                if let Some(mess) = info.payload().downcast_ref::<&str>() {
                    match info.location() {
                        Some(location) => error!(target: &location.to_string(), "{}", mess),
                        None => error!("{}", mess),
                    };
                }
            })
        });
        println!("\x1bc");
        log::set_logger(&HexgemLogger).map(|()| log::set_max_level(LevelFilter::Debug))
    }
}
trait ClientGet {
    fn get_client(&self) -> &str;
}

impl ClientGet for &str {
    fn get_client(&self) -> &'static str {
        let parts = self.split("::").collect::<Vec<&str>>();
        if parts[0] == "hexgem_engine" {
            return "\x1b[36;4mHexgem engine\x1b[0m";
        } else {
            return "\x1b[36;4mHexgem application\x1b[0m";
        }
    }
}
