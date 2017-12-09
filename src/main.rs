#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
Usage:
    cref
    cref import <import-repo>...
    cref list
    cref update [<update-repo>...]
    cref delete <delete-repo>
    cref (--help | --version)

Options:
    -h, --help     Show this screen
    -v, --version  Show version
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_import: bool,
    arg_import_repo: Vec<String>,
    cmd_list: bool,
    cmd_update: bool,
    arg_update_repo: Vec<String>,
    cmd_delete: bool,
    arg_delete_repo: String,
    flag_help: bool,
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
        println!("{:?}", args);
}
