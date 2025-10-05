use crate::utils::take_strip;
use std::fs::{copy, remove_file};
use std::path::{Path, PathBuf};

pub fn file_changed_only(
    src_files: &Vec<(PathBuf, i64, String, (f64, &'static str))>,
    source_files: &Vec<(PathBuf, i64)>,
    destination_files: &Vec<(PathBuf, i64)>,
    source: &PathBuf,
    destination: &PathBuf,
) {
    let mut files_to_be_copied = Vec::new();
    let mut files_to_be_updated = Vec::new();

    for (path, _, pretty_time, (size, unit)) in src_files {
        println!("File: {:?}", path);
        println!("Size: {} {}", size, unit);
        println!("Last Modified: {}", pretty_time);
    }

    // Remove extra files from destination
    for (dest_path, _) in destination_files {
        let dest_real_path = take_strip(dest_path, destination);
        let mut found = false;

        println!("source files: {:?}", source_files);

        for (src_path, _) in source_files {
            let src_real_path = take_strip(src_path, source);
            if src_real_path == dest_real_path {
                found = true;
                break;
            }
        }

        if !found {
            remove_file(dest_path).expect("Err: failed to delete the file(s)");
        }
    }

    // Compare source and destination files
    for (src_path, src_modified) in source_files {
        let src_real_path = take_strip(src_path, source);
        let mut dest_path = destination.clone();
        dest_path.push(src_real_path);

        let mut found = false;
        let mut needs_update = false;

        for (dest_path, dest_modified) in destination_files {
            let dest_real_path = take_strip(dest_path, destination);
            if src_real_path == dest_real_path {
                found = true;
                if src_modified > dest_modified {
                    needs_update = true;
                }
                break;
            }
        }

        if !found {
            copy(src_path, &dest_path).expect("Err: failed to copy file(s)");
            files_to_be_copied.push(src_real_path.to_path_buf().into_os_string());
        } else if needs_update {
            copy(src_path, &dest_path).expect("Err: failed to update file(s)");
            files_to_be_updated.push(src_real_path.to_path_buf().into_os_string());
        }
    }

    if !files_to_be_copied.is_empty() {
        println!(
            "Missing files {:?} are copied to {:?} destination",
            files_to_be_copied, destination
        );
    }
    if !files_to_be_updated.is_empty() {
        println!(
            "Files {:?} are updated in the {:?} destination",
            files_to_be_updated, destination
        );
    }
    if files_to_be_copied.is_empty() && files_to_be_updated.is_empty() {
        eprintln!("No missing or outdated files. Destination is up-to-date");
    }
}

pub fn else_file_changed_only(
    source_files: &Vec<(PathBuf, i64)>,
    destination_files: &Vec<(PathBuf, i64)>,
    source: &PathBuf,
    destination: &PathBuf,
) {
    if !destination_files.is_empty() {
        let mut all_src_real_path = Vec::new();
        let mut all_dest_real_path = Vec::new();

        for (dest_path, _) in destination_files {
            let dest_real_path = take_strip(dest_path, destination);
            all_dest_real_path.push(dest_real_path.to_path_buf());
            remove_file(dest_path).expect("Err: failed to delete the file(s)");
        }

        let dest_dir = Path::new(destination);
        for (src_path, _) in source_files {
            let src_real_path = take_strip(src_path, source);
            let new_dest_path = dest_dir.join(src_real_path.to_path_buf());
            all_src_real_path.push(src_real_path.to_path_buf());
            copy(src_path, new_dest_path).expect("Err: failed to copy the file(s)");
        }

        if all_src_real_path == all_dest_real_path {
            println!("No missing files. No changes needed");
        } else {
            println!("All the files are copied to destination successfully");
        }
    } else {
        eprintln!("Err: destination path is empty");
    }
}
