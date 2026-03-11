mod config;
mod format;
mod input;
mod render;

use clap::Parser;
use config::Cli;
use std::io::Read;

fn main() {
    let cli = Cli::parse();

    if cli.list {
        config::print_list();
        return;
    }

    let elements = config::resolve_elements(&cli);
    let icon_mode = config::resolve_icon_mode(&cli);

    let input: input::Input = if cli.demo {
        input::demo()
    } else {
        let mut buf = String::new();
        if std::io::stdin().read_to_string(&mut buf).is_err() {
            return;
        }
        match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(_) => return,
        }
    };

    let out: String = elements
        .iter()
        .filter_map(|e| render::render(*e, &input, icon_mode))
        .collect::<Vec<_>>()
        .join("  ");

    print!("{out}");
}
