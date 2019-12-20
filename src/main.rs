extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

#[derive(Debug)]
struct TimeKeeper {
    project_name: String,
    task_name: String,
    uuid: i32,
}
#[derive(Debug)]
struct CreateTask {
    project_name: String,
    task_name: String,
}
fn main() {
    let arguments = set_up_cli();
    let (action, value) = parse_input(&arguments);

    match action.as_str() {
        "show" => println!("Show all Projects"),
        _ => println!("Unknown command...Sorry"),
    }
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

fn parse_input<'a>(arg: &ArgMatches<'a>) -> (String, Option<CreateTask>) {
    match arg.subcommand() {
        ("show", Some(show_matches)) => (String::from("show"), None),
        ("create", Some(create_matches)) => {
            ((
                String::from("create"),
                Some(CreateTask {
                    project_name: create_matches.value_of("name").unwrap().to_string(),
                    task_name: String::from("default1"),
                }),
            ))
        }

        ("delete", Some(delete_matches)) => (
            String::from("delete"),
            Some(CreateTask {
                project_name: delete_matches.value_of("name").unwrap().to_string(),
                task_name: String::from("default1"),
            }),
        ),
        _ => panic!("No Command provided"),
    }
}

// fn create_project(name: &String, task: &String) -> TimeKeeper  {
//
// }
