use std::env;
use std::path::Path;
use std::fs::DirBuilder;
use std::io;

pub fn ensure_dir<T: AsRef<Path>>(path: T) -> io::Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(&path)
        //.expect(
            //&format!("Ensure dir {} failed", path.as_ref().to_string_lossy())
        //);
}

pub fn get_home_var() -> String {
    env::var("HOME").expect("env var $HOME not set.")
}
