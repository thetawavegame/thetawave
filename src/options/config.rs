use std::{
    env::current_dir,
    fs::{DirBuilder, File},
    io::prelude::*,
};

/// Creates config file in config directory from config file in this directory
macro_rules! confgen {
    ( $($filename:expr),* ) => {
        {
            let conf_dir = current_dir().unwrap().join("config");
            if !conf_dir.is_dir() {
                DirBuilder::new()
                    .create(conf_dir.clone())
                    .expect("Confgen failed: could not create config dir.");
            }

            $({
                let default = include_bytes!($filename);
                let file_path = conf_dir.join($filename);
                if !file_path.is_file() {
                    let mut file = File::create(file_path)
                        .expect(concat!("Confgen failed: could not create config file ", $filename, "."));
                    file.write_all(default)
                        .expect(concat!("Confgen failed: could not write config file ", $filename, "."));
                }
            })*
        }
    }
}

/// Generates the display config file
pub fn generate_config_files() {
    confgen!("display.ron");
    confgen!("input.ron");
}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "cli"))]
mod cli_tests {
    use argh::FromArgs;
    #[test]
    fn test_cli_parse_asset_path_dir() {
        assert_eq!(
            super::GameInitCLIOptions::from_args(&["thetawave"], &["--assets-dir", "myassets/"])
                .unwrap()
                .assets_dir,
            Some(std::path::PathBuf::from("myassets/"))
        );
    }
}
