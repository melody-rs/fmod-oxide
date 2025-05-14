use std::path::PathBuf;

use fmod::Utf8CString;

pub fn media_path_for(item: &str) -> Utf8CString {
    let api_dir = PathBuf::from(fmod::sys::API_DIR);
    let media_path = api_dir.join("core/examples/media").join(item);
    let media_path = media_path.to_str().unwrap();
    Utf8CString::new(media_path).unwrap()
}
