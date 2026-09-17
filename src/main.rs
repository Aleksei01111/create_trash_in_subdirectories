#![windows_subsystem = "windows"]
mod ui;
mod service;

use std::env;
use ui::app::App;

fn main() -> iced::Result {
    let path_to_config = String::from("config.lenovo");

    let args = env::args().collect::<Vec<String>>();

    println!("{args:?}");

    if args[1] == "s" {
        println!("silent is running");

        let mut files_creator = service::files_creator::FilesCreator::new();

        let loaded =
            service::files_creator_configuration::FilesCreatorConfigurationSaver::load_from_file(&path_to_config);

        if loaded.is_ok() {
            let config = loaded.unwrap();
            files_creator.create_files(0, config.path_to_directory.clone(), &config, false);
            println!("silent is done");
        }


        Ok(())
    }

    else {
        println!("standard mode");

        iced::daemon(App::new, App::update, App::view)
            .subscription(App::subscription)
            .run()
    }
}