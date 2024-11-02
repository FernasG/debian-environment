mod datatypes;

use console::Term;
use std::{process, process::Command};
use datatypes::{Package, PACKAGES};
use dialoguer::Select;

fn main() {
    let term = Term::stdout();

    term.set_title("Debian Environment");
    term.write_line("Welcome to Debian Environemnt!").unwrap();
    term.write_line("Here you can config your development environment in Debian distros.")
        .unwrap();

    let items = vec!["Complete Installation", "Custom Installation", "Cancel"];

    let option = Select::new()
        .with_prompt("Choose one option")
        .items(&items)
        .interact()
        .expect("Something went wrong, try again later.");

    term.clear_screen().unwrap();

    match option {
        0 => run_complete_install(term),
        1 => run_custom_install(),
        2 => exit_program(),
        _ => exit_program(),
    }

    exit_program();
}

fn run_complete_install(term: Term) {
    install_packages(term, Vec::from(PACKAGES));
}

fn run_custom_install() {}

fn install_packages(term: Term, packages: Vec<Package>) {
    let scripts_folder_path = "src/scripts";
    let packages_len = packages.len();

    for (index, package) in packages.iter().enumerate() {
        let message = format!(
            "[{}/{}] Installing: {}...",
            index + 1,
            packages_len,
            package.name
        );

        term.write_line(&message).unwrap();

        let script_path = format!("{}/{}", scripts_folder_path, package.filename);
        let add_execute_permission = format!("chmod +x {}", script_path);
        let run_script_file = format!("./{}", script_path);

        Command::new("sh")
            .arg("-c")
            .arg(add_execute_permission)
            .output()
            .expect("Something went wrong!");

        Command::new("sh")
            .arg(run_script_file)
            .output()
            .expect("Something went wrong!");
    }
}

fn exit_program() {
    println!("Quiting...");
    process::exit(0);
}
