use std::io;
use std::io::Write;
use std::{fs, io::Read, path::PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    sounds: Vec<SoundKey>,
}

impl Config {
    /// Loads & Parses the existing confile from, or makes one and returns the default config.
    pub fn try_get() -> crate::Result<Self> {
        let config_path = dirs::config_dir()
            .expect("Failed to find config dir")
            .join("notify_sound.toml");

        let config_file = fs::File::open(&config_path);

        // User hasn't made a config file yet, so we make a default one there to serve as
        // an example.
        if let Err(ref err) = config_file
            && err.kind() == io::ErrorKind::NotFound
        {
            let mut default_config = Config::default();
            default_config.sounds.push(SoundKey {
                app_name: ".*".to_string(),
                sound_path: String::new(),
            });

            let mut config_file = fs::File::create(&config_path)?;
            let string = toml::to_string(&default_config)?;
            write!(config_file, "{string}")?;

            return Ok(default_config);
        }

        let mut config_file = config_file?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        let config: Config = toml::from_str(&buffer)?;
        Ok(config)
    }

    /// Compile the regex and load the sound files into memory
    pub fn load_sounds(self) -> crate::Result<Vec<Sound>> {
        self.sounds
            .into_iter()
            .map(|s| {
                let s: crate::Result<Sound> = s.try_into();
                s
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SoundKey {
    app_name: String,
    sound_path: String,
}

pub struct Sound {
    app_regex: Regex,
    sound_bytes: &'static [u8],
}

impl TryFrom<SoundKey> for Sound {
    type Error = crate::Error;
    fn try_from(value: SoundKey) -> crate::Result<Self> {
        let app_regex = Regex::new(&value.app_name)?;
        let path: PathBuf = value.sound_path.into();
        let sound_bytes: Vec<u8> = fs::read(path)?;
        let sound_bytes: &'static [u8] = Box::new(sound_bytes).leak();

        Ok(Self {
            app_regex,
            sound_bytes,
        })
    }
}
