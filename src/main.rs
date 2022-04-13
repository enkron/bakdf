use std::{
    env, error, fs, os,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let rps = Rps::new()?;
    // 1st we're creating a directories' structure in user's home
    fs::create_dir_all(&rps.path)?;

    // 2nd we have to detect OS type, because symlink creation is platform dependent
    if cfg!(unix) {
        os::unix::fs::symlink(
            rps.path.join("env").join("dotfiles").join("Linux"),
            Path::new(env::var("HOME")?.as_str()).join("df_linux"),
        )?;
    }

    Ok(())
}

// the structure represents repositories' entity in user's environment
struct Rps {
    path: PathBuf,
}

impl Rps {
    fn new() -> Result<Self, env::VarError> {
        // build a `path` field
        let path = Path::new(env::var("HOME")?.as_str())
            .join("rps")
            .join("github.com")
            .join("enkron");

        Ok(Self { path })
    }
}
