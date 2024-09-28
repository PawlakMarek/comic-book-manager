use clap::{Arg, Command};

pub fn run() {
    let matches = Command::new("Comic Book Manager")
        .version("0.1.0")
        .author("PawlakMarek <26022173+PawlakMarek@users.noreply.github.com>")
        .about("Manages your comic book collection")
        .subcommand(
            Command::new("list").about("List entities").arg(
                Arg::new("entity")
                    .help("The type of entity to list")
                    .required(true)
                    .value_parser([
                        "publishers",
                        "series",
                        "issues",
                        "characters",
                        "creators",
                        "events",
                    ])
                    .index(1),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        let entity = matches
            .get_one::<String>("entity")
            .expect("required argument");
        println!("Listing {}", entity);
        // Here we'll call the appropriate function to list the entities
    } else {
        println!("No subcommand was used. Use --help for more information.");
    }
}
