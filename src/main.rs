use clap::{command, Arg, Command};
mod commands;

fn main() {
    let match_result = command!()
        .about("This application replicates Windows OS' del and rmdir commands.")
        .subcommand(
            Command::new("delete-file")
                .arg(
                    Arg::new("file-path")
                        .short('d')
                        .long("del-file")
                        .required(true)
                        .help("The requested file to be deleted.")
                )
        )
        .subcommand(
            Command::new("delete-folder")
                .arg(
                    Arg::new("folder-path")
                        .long("del-folder")
                        .required(true)
                        .help("The requested folder to be deleted.")
                )
        )
    .get_matches();

    match match_result.subcommand_matches("delete-file") {
        Some(file_args) => {
            let file_path = file_args.get_one::<String>("file-path").unwrap();
            commands::delete::remove_file(file_path);
        },
        None => (),
    };

    match match_result.subcommand_matches("delete-folder") {
        Some(folder_args) => {
            let folder_path = folder_args.get_one::<String>("folder-path").unwrap();
            commands::delete::remove_folder(folder_path);
        },
        None => (),
    };
}