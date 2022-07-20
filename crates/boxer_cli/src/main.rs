fn cli() -> clap::Command<'static> {
    clap::Command::new("boxer")
        .about("Some about text")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(clap::Command::new("init").about("Initialize new box"))
        .subcommand(clap::Command::new("create").about("Create new box"))
        .subcommand(clap::Command::new("run").about("Run script"))
        .subcommand(clap::Command::new("pack").about("Pack new box as template"))
}

fn main() {
    boxer_bridge::parse_configuration("config.ts");
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing new box");
        }
        Some(("create", _)) => {
            println!("Creating new box");
        }
        Some(("run", _)) => {
            println!("Running script");
        }
        Some(("pack", _)) => {
            println!("Packing box");
        }
        Some((ext, _)) => {
            println!("Calling out to {:?}", ext);
        }
        _ => unreachable!(),
    }
}
