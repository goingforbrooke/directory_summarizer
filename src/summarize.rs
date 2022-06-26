use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use walkdir::WalkDir;


pub fn catalog_directory<'a>(target_dir: &PathBuf, extension_counts: &'a mut HashMap<String, i128>) -> &'a mut HashMap<String, i128> {
    // Reset file extension counts to zero.
    *extension_counts = HashMap::new();
    // Categorize all extensionless files as "No extension."
    let default_extension = OsString::from("No extension");

    for entry in WalkDir::new(target_dir)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let file_ext = entry.path().extension().unwrap_or(&default_extension);
        // Add newly encountered file extensions to known file extensions with a counter of 0.
        let counter = extension_counts.entry(String::from(file_ext.to_string_lossy())).or_insert(0);
        // Increment the counter for known file extensions by one.
        *counter += 1;
    }
    extension_counts 
}