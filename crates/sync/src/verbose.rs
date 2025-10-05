use crate::utils::take_strip;
use std::{path::PathBuf, process::exit};

pub fn verbose_flag(
    source_files: &Vec<(PathBuf, i64)>,
    destination_files: &Vec<(PathBuf, i64)>,
    source: &PathBuf,
    destination: &PathBuf,
    src_files: &Vec<(PathBuf, i64, String, (f64, &'static str))>,
    dest_files: &Vec<(PathBuf, i64, String, (f64, &'static str))>,
) {
    let mut files_to_be_copied = Vec::new();
    let mut files_to_be_updated = Vec::new();
    let mut files_to_be_removed = Vec::new();

    for (src_path, src_modified) in source_files {
        let src_real_path = take_strip(src_path, source);

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
            files_to_be_copied.push(src_real_path.to_path_buf().into_os_string());
        } else if needs_update {
            files_to_be_updated.push(src_real_path.to_path_buf().into_os_string());
        }
    }

    for (dest_path, _) in destination_files {
        // compute relative path from destination root
        let dest_real_path = take_strip(dest_path, destination);

        let mut found = false;
        for (src_path, _) in source_files {
            let src_real_path = take_strip(src_path, source);
            if src_real_path == dest_real_path {
                found = true;
                break;
            }
        }

        if !found {
            files_to_be_removed.push(dest_real_path.to_owned());
        }
    }

    println!(
        "[INFO] Starting sync: /{} → /{}",
        source.display(),
        destination.display()
    );
    println!("[INFO] Scanning source folder...");
    println!(
        "[INFO] Found {} files in source, {} files in destination.",
        source.into_iter().count(),
        destination.into_iter().count()
    );

    if !files_to_be_copied.is_empty() {
        println!(
            "\n[VERBOSE] Checking file(s): {:?} in the source",
            source_files
                .iter()
                .map(|(path, _)| path)
                .collect::<Vec<_>>()
        );

        println!("[VERBOSE] Destination missing → copying file");
        println!(
            "[COPY] {:?} → {:?} destination\n",
            files_to_be_copied, destination
        );
    }
    if !files_to_be_updated.is_empty() {
        println!(
            "[VERBOSE] Checking file(s): {:?} in the source",
            source_files
                .iter()
                .map(|(path, _)| path)
                .collect::<Vec<_>>()
        );

        for (_, _, src_pretty_time, (_, _)) in src_files {
            for (_, _, dest_pretty_time, (_, _)) in dest_files {
                println!(
                    "[VERBOSE] Source is newer ({:?} > {:?})",
                    src_pretty_time, dest_pretty_time
                );
            }
        }

        println!(
            "[UPDATE] {:?} → {:?} destination",
            source_files
                .iter()
                .map(|(path, _)| path)
                .collect::<Vec<_>>(),
            destination,
        );
    }
    if !files_to_be_removed.is_empty() {
        println!("[VERBOSE] Checking destination extra files...");
        println!("[DELETE] {:?} not found in the source", files_to_be_removed);
    }

    if source_files.is_empty() {
        exit(0);
    }

    if files_to_be_copied.is_empty()
        && files_to_be_updated.is_empty()
        && files_to_be_removed.is_empty()
    {
        println!("\n[VERBOSE] Checking file...");
        println!("[VERBOSE] Files to be copied: {:?}", files_to_be_copied);
        println!("[VERBOSE] Files to be updated: {:?}", files_to_be_updated);
        println!("[VERBOSE] Files to be deleted: {:?}", files_to_be_removed);
        println!("[VERBOSE] File already up-to-date → skipping.");
        println!(
            "[SKIP] {:?} files",
            destination_files
                .iter()
                .map(|(path, _)| path)
                .collect::<Vec<_>>()
        );
    }
}
