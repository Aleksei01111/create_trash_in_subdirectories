#![windows_subsystem = "windows"]
mod ui;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use ui::app::App;
use rand::RngExt;
use std::thread;
use std::time::Duration;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .run()
}

pub struct FilesCreator {
    pub out_string: String,
    pub filename: String,
    pub file_creation_delay_in_milliseconds: u64,
}

impl FilesCreator {
    pub fn new() -> Self {
        Self {
            out_string: String::new(),
            filename: String::new(),
            file_creation_delay_in_milliseconds: 100,
        }
    }

    pub fn create_files(&mut self, path_to_directory: String, depth: i32, current_depth: i32, content_variants: &HashMap<i32, String>) {
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
                self.create_files(e.path().to_str().unwrap().to_string(), depth, current_depth + 1, content_variants);
            }
        }

        let path_to_file = format!("{}\\{}", path_to_directory, self.filename);

        let file_result = fs::File::create(&path_to_file);
        let mut out = format!("file created in\t{}", path_to_file);


        if file_result.is_ok() {
            let random_content = self.get_random_item(content_variants);
            file_result.unwrap().write_all(random_content.as_bytes()).unwrap();
            out.push_str(format!("\nwrite in\t{}\n", path_to_file).as_str());
        }

        if self.out_string.len() != 0 {
            self.out_string.push_str("\n");
        }
        self.out_string.push_str(out.as_str());

        thread::sleep(Duration::from_millis(self.file_creation_delay_in_milliseconds));
    }

    fn get_random_item(&self, items: &HashMap<i32, String>) -> String {
        let mut rng = rand::rng();

        if items.len() <= 0 {
            return String::new()
        }
        let random_index = rng.random_range(0..items.len());

        if let Some(founded) = items.iter().nth(random_index) {
            return founded.1.clone();
        }

        String::new()
    }
}