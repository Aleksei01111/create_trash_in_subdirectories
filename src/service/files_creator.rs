use std::collections::HashMap;
use std::{fs, thread};
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use rand::RngExt;
use crate::service::files_creator_configuration::FilesCreatorConfiguration;

pub struct FilesCreator {
    pub out_string: String,
}

impl<'a> FilesCreator {
    pub fn new() -> Self {
        Self {
            out_string: String::new(),
        }
    }

    pub fn create_files(&mut self, current_depth: i32, current_directory_path: String, configuration: &'a FilesCreatorConfiguration, do_not_create_files: bool) {
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
                self.create_files(current_depth + 1, e.path().to_str().unwrap().to_string(), configuration, do_not_create_files);
            }
        }

        self.out_string.push_str(create_file(current_directory_path, &configuration.filename, &configuration, do_not_create_files).as_str());

        let delay = get_delay(&configuration);

        self.out_string.push_str(format!("delay: {}\n", delay).as_str());

        thread::sleep(Duration::from_millis(delay));
    }
}

fn get_delay(configuration: &FilesCreatorConfiguration) -> u64 {
    let mut delay = 0;
    if configuration.file_creation_delay_in_milliseconds_low_limit != configuration.file_creation_delay_in_milliseconds_high_limit {
        delay = rand::rng().random_range(configuration.file_creation_delay_in_milliseconds_low_limit..configuration.file_creation_delay_in_milliseconds_high_limit);
    }

    delay
}

fn create_file(directory_path: String, filename: &String, configuration: &FilesCreatorConfiguration, do_not_create_files: bool) -> String {
    let path_to_file = format!("{}\\{}", directory_path, filename);

    if do_not_create_files {
        return String::from(format!("skipped {} \'not create\'\n", path_to_file).as_str());
    }

    if !configuration.contain_exclusions.is_empty() &&
            string_contain_any_substring(&path_to_file, &configuration.contain_exclusions, configuration.contain_exclusions_ignore_case) {
        return String::from(format!("skipped {}", path_to_file).as_str());
    }

    let file_result = fs::File::create(&path_to_file);
    let mut out = format!("file created in\t{}", path_to_file);

    if file_result.is_ok() {
        let random_content = get_random_item(&configuration.content_variants);
        file_result.unwrap().write_all(random_content.as_bytes()).unwrap();
        out.push_str(format!("\nwrite in\t{}\n", path_to_file).as_str());
    }

    out
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

fn string_contain_any_substring(string: &String, substrings: &HashMap<i32, String>, ignore_case: bool) -> bool {
    for (_, substring) in substrings {
        if ignore_case && string.to_lowercase().contains(substring) {
            return true;
        }
        else if !ignore_case && string.contains(substring) {
            return false;
        }
    }

    false
}