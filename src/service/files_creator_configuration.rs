use std::collections::HashMap;
use bincode::{Encode, Decode};

#[derive(Encode, Decode)]
pub struct FilesCreatorConfiguration {
    pub filename: String,
    pub file_creation_delay_in_milliseconds_low_limit: u64,
    pub file_creation_delay_in_milliseconds_high_limit: u64,
    pub path_to_directory: String,
    pub depth: i32,
    pub content_variants: HashMap<i32, String>,

    pub contain_exclusions: HashMap<i32, String>,
    pub contain_exclusions_ignore_case: bool,
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
            content_variants: HashMap::new(),
            contain_exclusions: HashMap::new(),
            contain_exclusions_ignore_case: true,
        }
    }
}

pub struct FilesCreatorConfigurationSaver {

}

impl FilesCreatorConfigurationSaver {
    pub fn save_to_file(path: &String, configuration: &FilesCreatorConfiguration) -> Result<(), Box<dyn std::error::Error>> {
        let encoded =
            bincode::encode_to_vec(configuration, bincode::config::standard())?;

        std::fs::write(&path, encoded)?;

        Ok(())
    }

    pub fn load_from_file(path: &String) -> Result<FilesCreatorConfiguration, Box<dyn std::error::Error>> {
        let encoded = std::fs::read(path)?;

        let (config, _) = bincode::decode_from_slice(&encoded, bincode::config::standard())?;

        Ok(config)
    }
}