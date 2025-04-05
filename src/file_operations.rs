use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

pub async fn open_file() -> Result<PathBuf, Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open an image file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(PathBuf::from(picked_file.path()))
}

pub async fn save_file(path: Option<PathBuf>) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    Ok(path)
}
