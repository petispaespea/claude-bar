mod config;
mod format;
mod input;
mod render;
mod setup;
mod style;
mod toml_config;

use clap::Parser;
use config::Cli;
use std::io::Read;

fn main() {
    let cli = Cli::parse();

    if let Some(ref shell) = cli.completions {
        let shell = shell.parse::<clap_complete::Shell>().unwrap_or_else(|_| {
            eprintln!("Unknown shell: {shell}. Use bash, zsh, fish, elvish or powershell.");
            std::process::exit(1);
        });
        clap_complete::generate(
            shell,
            &mut config::build_cli(),
            "claude-bar",
            &mut std::io::stdout(),
        );
        return;
    }

    if cli.setup {
        setup::run();
        return;
    }

    if cli.info {
        config::print_info();
        return;
    }

    if cli.print_default_config {
        print!("{}", toml_config::default_config_toml());
        return;
    }

    let config = toml_config::load_config(cli.config.as_ref().map(|p| p.to_str().unwrap()));
    let toml_layout = config.as_ref().map(|c| c.layout.elements.as_slice());
    let elements = config::resolve_elements(&cli, toml_layout);
    let icon_mode = config::resolve_icon_mode(&cli);
    let config = config.unwrap_or_default();

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
        .filter_map(|e| render::render(*e, &input, icon_mode, &config))
        .collect::<Vec<_>>()
        .join(&config.separator);

    print!("{out}");
}
