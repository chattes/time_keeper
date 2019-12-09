extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let arguments = set_up_cli();
    parse_input(&arguments);
}

fn set_up_cli<'a>() -> ArgMatches<'a> {
    App::new("timekeeper")
        .version("1.0")
        .about("Lets you manage time spent in Projects")
        .author("Sourav Chatterjee")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new Project to Track")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .value_name("PROJECT_NAME")
                        .help("Creates a new Project for time tracking")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes the Project")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .value_name("PROJECT_NAME")
                        .help("Deletes the Project")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("show").about("Show All Current Projects"))
        .get_matches()
}

fn parse_input<'a>(arg: &ArgMatches<'a>) {
    match arg.subcommand_name() {
        Some("show") => println!("Show all Projects"),
        Some("create") => println!("Create a New Project"),
        Some("delete") => println!("Delete Selected Project"),
        _ => panic!("No Command provided"),
    }
}
