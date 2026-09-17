use std::collections::HashMap;
use std::{fs, thread};
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use rand::RngExt;

pub struct FilesCreatorConfiguration {
    pub filename: String,
    pub file_creation_delay_in_milliseconds_low_limit: u64,
    pub file_creation_delay_in_milliseconds_high_limit: u64,
    pub path_to_directory: String,
    pub depth: i32,
    pub content_variants: HashMap<i32, String>
}

impl FilesCreatorConfiguration {
    pub fn new(filename: String,
               file_creation_delay_in_milliseconds_low_limit: u64,
               file_creation_delay_in_milliseconds_high_limit: u64,
               path_to_directory: String, depth: i32) -> Self {
        Self {
            filename,
            file_creation_delay_in_milliseconds_low_limit,
            file_creation_delay_in_milliseconds_high_limit,
            path_to_directory,
            depth,
            content_variants: HashMap::new()
        }
    }
}

pub struct FilesCreator {
    pub out_string: String,
}

impl<'a> FilesCreator {
    pub fn new() -> Self {
        Self {
            out_string: String::new(),
        }
    }

    pub fn create_files(&mut self, current_depth: i32, current_directory_path: String, configuration: &'a FilesCreatorConfiguration) {
        if current_depth > configuration.depth {
            return;
        }

        if !Path::new(current_directory_path.as_str()).exists() {
            return;
        }

        let entities = fs::read_dir(&current_directory_path);
        if entities.is_err() {
            return;
        }

        for entry in entities.unwrap() {
            if entry.is_err() {
                continue;
            }

            if let Ok(e) = entry && e.file_type().unwrap().is_dir() {
                self.create_files(current_depth + 1, e.path().to_str().unwrap().to_string(), configuration);
            }
        }

        let path_to_file = format!("{}\\{}", current_directory_path, configuration.filename);

        let file_result = fs::File::create(&path_to_file);
        let mut out = format!("file created in\t{}", path_to_file);

        if self.out_string.len() != 0 {
            self.out_string.push_str("\n");
        }

        if file_result.is_ok() {
            let random_content = get_random_item(&configuration.content_variants);
            file_result.unwrap().write_all(random_content.as_bytes()).unwrap();
            out.push_str(format!("\nwrite in\t{}\n", path_to_file).as_str());
        }

        let mut delay = 0;
        if configuration.file_creation_delay_in_milliseconds_low_limit != configuration.file_creation_delay_in_milliseconds_high_limit {
            delay = rand::rng().random_range(configuration.file_creation_delay_in_milliseconds_low_limit..configuration.file_creation_delay_in_milliseconds_high_limit);
        }

        out.push_str(format!("delay: {}\n", delay).as_str());

        self.out_string.push_str(out.as_str());

        thread::sleep(Duration::from_millis(delay));
    }
}

fn get_random_item(items: &HashMap<i32, String>) -> String {
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