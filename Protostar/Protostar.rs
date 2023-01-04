// | (c) Tremeschin, AGPLv3-only License | Protostar Project | //
#![allow(non_snake_case)]
#![crate_type = "lib"]

pub use core::fmt::Error;
pub use itertools::Itertools;
pub use libm::*;
pub use num::complex::{Complex, ComplexFloat};
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::collections::BTreeMap;
pub use std::fs::File;
pub use std::io::BufReader;
pub use std::io::BufWriter;
pub use std::io::Write;
pub use std::io::Read;
pub use std::net::{TcpListener, TcpStream};
pub use std::path::PathBuf;
pub use std::process::exit;
pub use std::process::Command as Subprocess;
pub use std::process::Stdio;
pub use std::time::{Duration, Instant};
pub use toml;
pub use unordered_pair::UnorderedPair;

// Cache
pub use lru_cache::LruCache;

// Serde and CLI
pub use clap::Parser;
pub use serde_derive::{Deserialize, Serialize};
pub use strum::Display;
pub use toml::Value;

// Sets up Protostar logging, this shouldn't really crash..
pub use log::{debug, error, info, trace, warn};

pub fn setupLog() {
    use fern::colors::{Color, ColoredLevelConfig};

    // Time the program started
    let start = Instant::now();

    // Logging colors
    let logColors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::BrightYellow)
        .info(Color::White)
        .debug(Color::Blue)
        .trace(Color::BrightBlue);

    // Create fern log template
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{green}{:<6} µs{reset}]─[{level}{:<5}{reset}] ▸ {}",
                // start.elapsed().as_millis(),
                start.elapsed().as_micros(),
                record.level(),
                message,
                level = format_args!("\x1B[{}m", logColors.get_color(&record.level()).to_fg_str()),
                green = format_args!("\x1B[{}m", Color::Green.to_fg_str()),
                reset = "\x1B[0m"
            ))
        })
        .chain(std::io::stdout())
        // .level(log::LevelFilter::Info)
        .level(log::LevelFilter::Trace)
        // .chain(fern::log_file("output.log").expect("Failed to set logging file"))
        .apply()
        .expect("Failed to set up logging");
}
