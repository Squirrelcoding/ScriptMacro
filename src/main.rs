use clap::{Arg, ArgMatches, Command};
mod functions;
use functions::lib;
fn main() {
    functions::init::init();
    let matches = Command::new("ScriptManager")
        .version("1.0")
        .about("A CLI tool that runs script macros")
        .author("Squirrelcoding")
        .subcommand(
            Command::new("macro")
                .about("Subcommand which takes an argument to run a macro")
                .arg(
                    Arg::new("macro")
                        .short('M')
                        .long("macro")
                        .takes_value(true)
                        .value_name("MACRO_NAME")
                        .help("Name of macro to run"),
                ),
        )
        .subcommand(Command::new("menu").about("Launches the TUI menu"))
        .get_matches();
    run(matches);
}

fn run(m: ArgMatches) {
    match m.subcommand() {
        Some(command) => match command {
            ("macro", args) => {
                let a = lib::get_macros();
                let input: Vec<_> = args.values_of("macro").unwrap().collect();
                let macro_name = a.get(input[0]).unwrap();
                functions::run_macro::run(macro_name);
            }
            ("menu", _args) => {
                functions::tui::menu();
            }
            _ => {
                println!("No arugements provided");
            }
        },
        None => {
            println!("No arguments provided, run scrmng --help ");
        }
    }
}
