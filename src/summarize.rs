use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use walkdir::WalkDir;


pub fn catalog_directory(target_dir: &PathBuf) -> HashMap<String, i128> {
    let mut filetype_counts = HashMap::<String, i128>::new();
    let default_extension = OsString::from("No extension");

    for entry in WalkDir::new(target_dir)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let file_ext = entry.path().extension().unwrap_or(&default_extension);
        // Add the file path to known file paths with a counter of 0.
        let counter = filetype_counts.entry(String::from(file_ext.to_string_lossy())).or_insert(0);
        // Increment the file path's counter by one.
        *counter += 1;
    }
    filetype_counts
}