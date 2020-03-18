extern crate clap;

use clap::{crate_version, App, Arg, SubCommand, AppSettings};

use todod::test;

// fn buildSubCommand(&mut self, name: &str, about: &str, args: App) -> App {
//         self.subcommand(SubCommand::with_name(name)
//                     .version(crate_version!())
//                     .about(about)
//                     .args(args))
// }

fn main() {
    test();
    let matches = App::new("todo")
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequired)
        .version(crate_version!())
        .about("helps you organize your gtd system in a todo.txt")
        .author("Felix Karg")
        .arg(Arg::with_name("todo_config")
             .short("d")
             .long("todo_config")
             .default_value("~/.config/todo/config")
             .help("Use a configuration file other than the default"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .default_value("todo.txt")
            .help("your main todo file"))
        .subcommand(SubCommand::with_name("add")
                    .version(crate_version!())
                    .about("adding a new todo")
                    .arg(Arg::with_name("todo")
                         .help("The todo to add")
                         .value_name("TODO")
                         .takes_value(true)
                         .required(true)
                    ))
        .subcommand(SubCommand::with_name("addm")
                    .version(crate_version!())
                    .about("adding multiple new todos")
                    .arg(Arg::with_name("todo")
                         .help("The todos to add")
                         .value_name("TODOS")
                         .takes_value(true)
                         .required(true)
                    ))
        .subcommand(SubCommand::with_name("addto")
                    .version(crate_version!())
                    .about("add text to file")
                    .args(&[
                          Arg::from_usage("<DEST>     'The file you want the text to add'"),
                          Arg::from_usage("<TEXT>     'The text you want to add'"),
                    ]))
        .subcommand(SubCommand::with_name("append")
                    .version(crate_version!())
                    .about("append TEXT to ITEM")
                    .visible_alias("app")
                    .args(&[
                          Arg::from_usage("<ITEM#>   'The item you want to append text to'"),
                          Arg::from_usage("<TEXT>   'The text you want to add to the item'"),
                    ]))
        .subcommand(SubCommand::with_name("del")
                    .visible_alias("rm")
                    .version(crate_version!())
                    .about("delete an item")
                    .args(&[
                          Arg::from_usage("<ITEM#>   'Deletes the task on line ITEM# in todo.txt.'"),
                          Arg::from_usage("[TERM]       'If TERM specified, deletes only TERM from the task.'"),
                    ]))
        .subcommand(SubCommand::with_name("do")
                    .version(crate_version!())
                    .about("mark an item as done")
                    .arg(Arg::from_usage("<ITEMS#>    'The items that should be marked as done'")))
        .subcommand(SubCommand::with_name("list")
                    .version(crate_version!())
                    .about("listing current todos")
                    .visible_alias("ls")
                    .arg(Arg::with_name("context")
                         .help("context")
                         .value_name("CONTEXT")
                         .takes_value(true)
                    ))
        .subcommand(SubCommand::with_name("listall")
                    .visible_alias("lsa"))
        .subcommand(SubCommand::with_name("listcon")
                    .visible_alias("lsc"))
        .subcommand(SubCommand::with_name("listproj")
                    .visible_alias("lsp"))
        .subcommand(SubCommand::with_name("pri")
                    .version(crate_version!())
                    .about("Adds PRIORITY to task on line ITEM#.  If the task is already prioritized, replaces current priority with new PRIORITY.  PRIORITY must be a letter between A and Z.")
                    .visible_alias("p")
                    .args(&[
                          Arg::from_usage("<ITEM#>  'ITEM# to add PRIORITY to.'"),
                          Arg::from_usage("<PRIORITY>   'New PRIORITY to assign ITEM# to.'"),
                    ]))
        .subcommand(SubCommand::with_name("replace")
                    .version(crate_version!())
                    .about("Replaces task on line ITEM# with UPDATED TODO.")
                    .args(&[
                          Arg::from_usage("<ITEM#>  'Replaces task on line ITEM#.'"),
                          Arg::from_usage("<UPDATED TODO>   'Replaces line ITEM# with UPDATED TODO.'"),
                    ]))

        .get_matches();

    let todofile = matches.value_of("file").unwrap();
    println!("{}", todofile);
    if let Some(cmd) = matches.subcommand_name() {
        println!("selected {}!", cmd);
    }
}
