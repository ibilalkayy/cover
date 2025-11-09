use backup::sync::sync::SyncData;
use std::{
    env,
    fs::{File, create_dir_all, read_dir, read_to_string, remove_dir_all, remove_file, write},
    path::PathBuf,
    thread, time,
};

#[test]
fn test_src_dest_dir_present() {
    let home_dir = env::var("HOME").expect("HOME env variable not set");

    let src_path = PathBuf::from(&home_dir).join("tmpsrc_present");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_present");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        sync.src_dest_dir_present(),
        "Source or destination directory not detected"
    );

    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_single_command_selected() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");

    let src_path = PathBuf::from(&home_dir).join("tmpsrc_command");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_command");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let mut sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        sync.single_command_selected(),
        "Expected only one command active, but function reported multiple"
    );

    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_list_source_files() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(home_dir).join("tmpsrc_listing");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = format!(
        "{}/src_test_file.txt",
        src_path.to_string_lossy().to_string()
    );
    File::create(&src_file).expect("Err: failed to create a source file");

    let mut searched_file: Vec<String> = Vec::new();
    let searched = sync.list_source_files();
    for file in searched {
        let filename = file
            .file_name()
            .and_then(|f| f.to_str())
            .expect("Err: failed to get the filename");

        searched_file.push(filename.to_string());
    }

    assert!(searched_file.len() != 0);

    remove_file(&src_file).ok();
    remove_dir_all(&src_path).ok();
}

#[test]
fn test_list_destination_files() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let dest_path = PathBuf::from(home_dir).join("tmpdest_listing");

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let dest_file = format!(
        "{}/dest_test_file.txt",
        dest_path.to_string_lossy().to_string()
    );
    File::create(&dest_file).expect("Err: failed to create a destination file");

    let mut searched_file: Vec<String> = Vec::new();
    let searched = sync.list_destination_files();
    for file in searched {
        let filename = file
            .file_name()
            .and_then(|f| f.to_str())
            .expect("Err: failed to get the filename");

        searched_file.push(filename.to_string());
    }

    assert!(searched_file.len() != 0);

    remove_file(&dest_file).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_file_duration_since() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");

    let src_path = PathBuf::from(&home_dir).join("tmpsrc_duration");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_duration");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = format!(
        "{}/src_duration_since.txt",
        src_path.to_string_lossy().to_string()
    );
    File::create(&src_file).expect("Err: failed to create a source file");

    let dest_file = format!(
        "{}/dest_duration_since.txt",
        dest_path.to_string_lossy().to_string()
    );
    File::create(&dest_file).expect("Err: failed to create a destination file");

    let (src_numeric, dest_numeric) = sync
        .file_duration_since(&PathBuf::from(&src_file))
        .expect("Err: failed to get the file changes");

    assert!(src_numeric != 0.0 && dest_numeric != 0.0);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_src_file_created() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(home_dir).join("tmpsrc_creation");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }
    create_dir_all(&src_path).expect("Err: failed to create a source directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = format!(
        "{}/src_file_creation.txt",
        src_path.to_string_lossy().to_string()
    );
    File::create(&src_file).expect("Err: failed to create a source file");

    let mut file_found = false;
    let src_directory = read_dir(&src_path).expect("Err: failed to read the source directory");
    for file in src_directory {
        if file
            .unwrap()
            .file_name()
            .to_string_lossy()
            .to_string()
            .len()
            != 0
        {
            file_found = true;
        }
    }

    let created = sync.src_file_created();
    assert_eq!(created, file_found);

    remove_file(&src_file).ok();
    remove_dir_all(&src_path).ok();
}

#[test]
fn test_src_file_modified() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(&home_dir).join("tmpsrc_modified");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_modified");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_path.join("common.txt");
    let dest_file = dest_path.join("common.txt");

    File::create(&src_file).expect("Err: failed to create a source file");
    File::create(&dest_file).expect("Err: failed to create a destination file");

    write(&dest_file, "original content").expect("Err: failed to modify the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_file, "modification content").expect("Err: failed to modify the file");

    let src_entries = read_dir(&sync.source).expect("Err: failed to read the source dir");
    let mut modified_file = PathBuf::new();

    for entry in src_entries.flatten() {
        let filename = entry.file_name().to_string_lossy().to_string();
        if let Some((src_time, dest_time)) = sync.file_duration_since(&PathBuf::from(&filename)) {
            if src_time > dest_time && dest_time != 0.0 {
                modified_file = PathBuf::from(&filename);
            } else {
                break;
            }
        }
    }

    let (modified, is_modified) = sync.src_file_modified();
    assert_eq!(modified.to_string_lossy(), modified_file.to_string_lossy());
    assert!(is_modified);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_dest_file_created() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let dest_path = PathBuf::from(home_dir).join("tmpdest_creation");

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let dest_file = format!(
        "{}/dest_file_creation.txt",
        dest_path.to_string_lossy().to_string()
    );
    File::create(&dest_file).expect("Err: failed to create a source file");

    let mut file_found = false;
    let dest_directory =
        read_dir(&dest_path).expect("Err: failed to read the destination directory");
    for file in dest_directory {
        if file
            .unwrap()
            .file_name()
            .to_string_lossy()
            .to_string()
            .len()
            != 0
        {
            file_found = true;
        }
    }

    let created = sync.dest_file_created();
    assert_eq!(created, file_found);

    remove_file(&dest_file).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_dest_file_modified() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(&home_dir).join("tmpsrc_modification");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_modification");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_path.join("common.txt");
    let dest_file = dest_path.join("common.txt");

    File::create(&src_file).expect("Err: failed to create a source file");
    File::create(&dest_file).expect("Err: failed to create a destination file");

    write(&src_file, "original content").expect("Err: failed to modify the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&dest_file, "modification content").expect("Err: failed to modify the file");

    let dest_entries =
        read_dir(&sync.destination).expect("Err: failed to read the destination dir");
    let mut modified_file = PathBuf::new();

    for entry in dest_entries.flatten() {
        let filename = entry.file_name().to_string_lossy().to_string();
        if let Some((src_time, dest_time)) = sync.file_duration_since(&PathBuf::from(&filename)) {
            if dest_time > src_time && src_time != 0.0 {
                modified_file = PathBuf::from(&filename);
            } else {
                break;
            }
        }
    }

    let (modified, is_modified) = sync.dest_file_modified();
    assert_eq!(modified.to_string_lossy(), modified_file.to_string_lossy());
    assert!(is_modified);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_copy_src_to_destination() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(&home_dir).join("tmpsrc_dest");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_src");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_path.join("common.txt");
    File::create(&src_file).expect("Err: failed to create a source file");
    write(&src_file, "original content").expect("Err: failed to modify the file");

    sync.copy_src_to_destination();

    let src_files = sync.list_source_files();
    let dest_files = sync.list_destination_files();

    let src: Vec<String> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
        .collect();

    let dest: Vec<String> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
        .collect();

    assert_eq!(src, dest);

    remove_file(&src_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_remove_dest_files() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(&home_dir).join("tmpsrc_remove");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_remove");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let dest_file = dest_path.join("common.txt");
    File::create(&dest_file).expect("Err: failed to create a destination file");
    write(&dest_file, "original content").expect("Err: failed to modify the file");

    sync.remove_dest_file();

    let src_files = sync.list_source_files();
    let dest_files = sync.list_destination_files();

    let src: Vec<String> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
        .collect();

    let dest: Vec<String> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
        .collect();

    assert_eq!(src, dest);

    remove_file(&dest_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_update_dest_file() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(&home_dir).join("tmpsrc_update");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_update");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_path.join("common.txt");
    let dest_file = dest_path.join("common.txt");

    File::create(&src_file).expect("Err: failed to create a source file");
    File::create(&dest_file).expect("Err: failed to create a destination file");

    write(&dest_file, "destination content").expect("Err: failed to modify the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_file, "source content").expect("Err: failed to modify the file");

    let (modified_src_file, src_modified) = sync.src_file_modified();
    if src_modified {
        sync.update_dest_file(modified_src_file.clone());
    }

    let src_files = sync.list_source_files();
    let dest_files = sync.list_destination_files();

    let mut src_content = String::new();
    let mut dest_content = String::new();

    for src_file in src_files {
        for dest_file in &dest_files {
            src_content = read_to_string(&src_file).expect("Err: failed to read the file");
            dest_content = read_to_string(&dest_file).expect("Err: failed to read the file");
        }
    }
    assert_eq!(src_content, dest_content);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_overwrite_with_src() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let src_path = PathBuf::from(&home_dir).join("tmpsrc_overwrite");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_overwrite");

    if src_path.exists() {
        remove_dir_all(&src_path).ok();
    }

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }

    create_dir_all(&src_path).expect("Err: failed to create a source directory");
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: src_path.clone(),
        destination: dest_path.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_path.join("common.txt");
    let dest_file = dest_path.join("common.txt");

    File::create(&src_file).expect("Err: failed to create a source file");
    File::create(&dest_file).expect("Err: failed to create a destination file");

    write(&src_file, "source content").expect("Err: failed to modify the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&dest_file, "destination content").expect("Err: failed to modify the file");

    let (modified_dest_file, dest_modified) = sync.dest_file_modified();
    if dest_modified {
        sync.overwrite_with_src(modified_dest_file);
    }

    let src_files = sync.list_source_files();
    let dest_files = sync.list_destination_files();

    let mut src_content = String::new();
    let mut dest_content = String::new();

    for src_file in src_files {
        for dest_file in &dest_files {
            src_content = read_to_string(&src_file).expect("Err: failed to read the file");
            dest_content = read_to_string(&dest_file).expect("Err: failed to read the file");
        }
    }
    assert_eq!(src_content, dest_content);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_dir_all(&src_path).ok();
    remove_dir_all(&dest_path).ok();
}

#[test]
fn test_remove_all_dest_files() {
    let home_dir = env::var("HOME").expect("Err: failed to get the home directory");
    let dest_path = PathBuf::from(&home_dir).join("tmpdest_rmv_all");

    if dest_path.exists() {
        remove_dir_all(&dest_path).ok();
    }
    create_dir_all(&dest_path).expect("Err: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_path.clone(),
        changed_only: false,
        delete: true,
        verbose: false,
        dry_run: false,
    };

    let dest_file = dest_path.join("common.txt");
    File::create(&dest_file).expect("Err: failed to create a destination file");
    write(&dest_file, "written content").expect("Err: failed to write the file");

    sync.remove_all_dest_files();

    let dest_files = sync.list_destination_files();

    let dest: Vec<String> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
        .collect();

    assert_eq!(dest.len(), 0);

    remove_file(&dest_file).ok();
    remove_dir_all(&dest_path).ok();
}
