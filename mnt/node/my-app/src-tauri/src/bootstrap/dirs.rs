use directories::UserDirs;
use std::io;
use std::path;

// ユーザのホームディレクトリ直下にデータベースのディレクトリを作成する
pub fn make_home_dir() -> Result<path::PathBuf, io::Error> {
    let user_dirs = UserDirs::new();

    let path_buf = match user_dirs {
        Some(dirs) => Ok(dirs.home_dir().to_path_buf()),
        None => create_dir_under_current(),
    };

    path_buf
}

fn create_dir_under_current() -> Result<path::PathBuf, io::Error> {
    let path_buf = std::env::current_dir();
    let path_buf = match path_buf {
        Ok(path) => Ok(path),
        Err(e) => Err(e),
    };
    path_buf
}
