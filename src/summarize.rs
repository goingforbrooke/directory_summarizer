use std::collections::HashMap;
use walkdir::WalkDir;


pub fn make_summary() -> HashMap<String, i128> {
    let mut filetype_counts = HashMap::<String, i128>::new();

    // todo: Check if `test_dir` exists first.
    for entry in WalkDir::new("test_dir")
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let file_path = String::from(entry.path().to_string_lossy());
        let file_ext = entry.path().extension().unwrap().to_str().unwrap();
        // Add the file path to known file paths with a counter of 0.
        let counter = filetype_counts.entry(file_ext.to_string()).or_insert(0);
        // Increment the file path's counter by one.
        *counter += 1;
    }
    return filetype_counts
}