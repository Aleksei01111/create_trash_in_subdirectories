#![windows_subsystem = "windows"]
mod ui;

use std::fs;
use std::path::Path;
use ui::app::App;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .run()
}

pub struct FilesCreator {
    pub out_string: String,
    pub filename: String,
}

impl FilesCreator {
    pub fn new() -> Self {
        Self {
            out_string: String::new(),
            filename: String::new(),
        }
    }

    pub fn create_files(&mut self, path_to_directory: String, depth: i32, current_depth: i32) {
        if current_depth > depth {
            return;
        }

        if !Path::new(path_to_directory.as_str()).exists() {
            return;
        }

        let entities = fs::read_dir(&path_to_directory);
        if entities.is_err() {
            return;
        }

        for entry in entities.unwrap() {
            if entry.is_err() {
                continue;
            }

            if let Ok(e) = entry && e.file_type().unwrap().is_dir() {
                self.create_files(e.path().to_str().unwrap().to_string(), depth, current_depth + 1);
            }
        }

        let path_to_file = format!("{}\\{}", path_to_directory, self.filename);

        fs::File::create(&path_to_file);
        let out = format!("file created in\t{}", path_to_file);

        if self.out_string.len() != 0 {
            self.out_string.push_str("\n");
        }
        self.out_string.push_str(out.as_str());

        println!();
    }
}