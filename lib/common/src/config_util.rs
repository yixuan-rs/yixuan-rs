use std::path::Path;

use serde::de::DeserializeOwned;

pub fn load_or_create<T: DeserializeOwned>(path: &str, defaults: &str) -> T {
    let path = Path::new(path);

    std::fs::read_to_string(path).map_or_else(
        |_| {
            path.parent()
                .inspect(|parent| std::fs::create_dir_all(parent).unwrap());

            std::fs::write(path, defaults).unwrap();
            toml::from_str(defaults).unwrap_or_else(|err| {
                panic!(
                    "failed to parse defaults for configuration file {}: {}",
                    path.display(),
                    err
                )
            })
        },
        |data| {
            toml::from_str(&data).unwrap_or_else(|err| {
                panic!(
                    "failed to parse configuration file {}: {}",
                    path.display(),
                    err
                )
            })
        },
    )
}
