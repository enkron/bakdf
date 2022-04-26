use std::{
    env, error, fs, os,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    // 0 - build an instance of `Rps`'s type
    let rps = Rps::new("rps3").unwrap_or_default();
    // 1st we're creating a directories' structure in user's home
    fs::create_dir_all(&rps.path)?;

    let df_linux_path = Path::new(env::var("HOME")?.as_str()).join("df_linux");
    if df_linux_path.exists() {
        println!("warn: {:#?} already exists. skipping", df_linux_path);
    } else {
        // 2nd we have to detect OS type, because symlink creation is platform dependent
        if cfg!(unix) {
            os::unix::fs::symlink(
                rps.path.join("env").join("dotfiles").join("Linux"),
                &df_linux_path,
            )?;
        }
    }
    Ok(())
}

// the structure represents repositories' entity in user's environment
struct Rps {
    path: PathBuf,
}

impl Default for Rps {
    fn default() -> Self {
        Self {
            path: Path::new(env::var("HOME").unwrap().as_str())
                .join("rps")
                .join("github.com")
                .join("enkron"),
        }
    }
}

impl Rps {
    fn new(dir_name: &str) -> Result<Self, env::VarError> {
        // build a `path` field
        let path = Path::new(env::var("HOME")?.as_str())
            .join(dir_name)
            .join("github.com")
            .join("enkron");

        Ok(Self { path })
    }
}
