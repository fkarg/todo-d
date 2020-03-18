extern crate clap;

use clap::{crate_version, App};

use todod; // import lib

fn main() {
    todod::test();
    App::new("todo-d")
        .version(crate_version!())
        .about("deamon to help you synchronize your gtd system")
        .author("Felix Karg")
        .get_matches();
}
