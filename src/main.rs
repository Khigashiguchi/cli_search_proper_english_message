#![feture(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

docopt!(Args derive Debug, "
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
");

fn main() {
    let args: Args = Args::docopt().deserialize().unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
