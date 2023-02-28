// TODO: look at clap-verbosity-flag, clap-cargo, and concolor-clap
// TODO: look into clap multicall
// TODO: look into clap_complete
// TODO: look into clap_mangen
// TODO: look into argfile
// TODO: think about using clap Parse feature

mod install;
mod remove;
mod search;
mod upgrade;

use clap::{arg, command, Command};

pub fn begin_cli() -> crate::Result<()> {
    let _file_arg = arg!(-f --file ""); // TODO: add input file support for package lists

    let mut matches = command!()
            .propagate_version(true)
            .subcommand(
                Command::new("install")
                    .aliases(["i", "sync", "snc", "get"])
                    .about("Install package(s)")
                    .arg(arg!([package] ... "Package(s) to install"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("remove")
                    .aliases(["rem", "r"])
                    .about("Remove package(s) from the system")
                    .arg(arg!([package] ... "Package(s) to remove"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("search")
                    .aliases(["s", "sch"])
                    .about("Search the repositories for package(s)")
                    .arg(arg!([package] ... "Package(s) to search for"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("upgrade")
                    .aliases(["up", "u"])
                    .about("Upgrade selected package(s). Default is all")
                    .arg(arg!([package] ... "Package(s) to upgrade (default is all)")),
            )
            .get_matches();

    let (command, mut packages) =
        matches.remove_subcommand().unwrap_or_default();

    let packages: Vec<String> = packages
        .try_remove_many("package")
        .unwrap_or_default()
        .unwrap_or_default()
        .collect();

    match command.as_str() {
        "install" => install::install(packages)?,
        "remove " => remove::remove(packages)?,
        "search" => search::search(packages)?,
        _ => upgrade::upgrade(packages)?,
    }

    Ok(())
}

fn is_installed(p: &String) -> bool {
    todo!();
}
