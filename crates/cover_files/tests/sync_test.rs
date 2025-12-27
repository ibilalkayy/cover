use cover_files::sync::{
    log::{filter_dest_dir, filter_dest_file, filter_src_dir, filter_src_file},
    sync::SyncData,
};
use std::{
    env,
    fs::{
        File, create_dir_all, metadata, read_dir, read_to_string, remove_dir_all, remove_file,
        write,
    },
    path::PathBuf,
    thread,
    time::{self, UNIX_EPOCH},
};
use walkdir::WalkDir;

#[test]
fn test_src_dest_dir_present() {
    let home_dir = env::var("HOME").expect("HOME env variable not set");

    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_present");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_present");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        sync.src_dest_dir_present(),
        "[ERROR]: source or destination directory not detected"
    );

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_single_command_selected() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");

    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_command");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_command");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let mut sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        sync.single_command_selected(),
        "[ERROR]: expected one command, but multiple are reported"
    );

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_list_src_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmpsrc_listing");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let src_sub_file = src_sub_dir.join("test_src_file.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut searched_file: Vec<PathBuf> = Vec::new();
    let searched = sync.list_src_files();
    for file in searched {
        let filename = file
            .file_name()
            .and_then(|f| f.to_str())
            .expect("[ERROR]: failed to get the filename");

        searched_file.push(PathBuf::from(filename));
    }

    assert!(searched_file.len() != 0);

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_list_src_dirs() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmpsrc_dir_listing");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }
    let src_sub_dir = src_parent_dir.join("test_dir");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut searched_dir: Vec<PathBuf> = Vec::new();
    let searched = sync.list_src_dirs();
    for dir in &searched {
        let data = dir
            .iter()
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string_lossy()
            .to_string();

        searched_dir.push(PathBuf::from(data));
    }

    assert!(searched_dir.len() != 0);

    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_list_dest_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_listing");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let dest_sub_file = dest_sub_dir.join("dest_test_file.txt");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut searched_file: Vec<PathBuf> = Vec::new();
    let searched = sync.list_dest_files();
    for file in searched {
        let filename = file
            .file_name()
            .and_then(|f| f.to_str())
            .expect("[ERROR]: failed to get the filename");

        searched_file.push(PathBuf::from(filename));
    }

    assert!(searched_file.len() != 0);

    remove_file(&dest_sub_file).ok();
    remove_file(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_list_dest_dirs() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_dir_listing");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut searched_dir: Vec<PathBuf> = Vec::new();
    let searched = sync.list_dest_dirs();
    for dir in &searched {
        let data = dir
            .iter()
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string_lossy()
            .to_string();

        searched_dir.push(PathBuf::from(data));
    }

    assert!(searched_dir.len() != 0);

    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_file_timestamp() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_timestamp");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_timestamp");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("src_duration_since.txt");
    let dest_sub_file = dest_sub_dir.join("dest_duration_since.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files_list = sync.list_src_files();
    let dest_files_list = sync.list_dest_files();

    let src_timestamp = sync.file_timestamp(src_files_list.clone(), &sync.source);
    let dest_timestamp = sync.file_timestamp(dest_files_list.clone(), &sync.destination);

    for (path, dest_time) in &dest_timestamp {
        let src = src_timestamp.get(path);
        match src {
            Some(src_time) => {
                assert!(*src_time != 0.0 && *dest_time != 0.0);
            }
            None => {}
        }
    }

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_file_created() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmpsrc_creation");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let src_sub_file = src_sub_dir.join("src_file_creation.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let mut file_found = false;
    for entry in WalkDir::new(&src_parent_dir) {
        let entry_path = entry
            .expect("[ERROR]: failed to get the entry")
            .path()
            .to_path_buf();
        if entry_path.is_file() {
            file_found = true;
        }
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let created = sync.src_file_created();
    assert_eq!(created, file_found);

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_src_file_modified() {
    let mut modified_file = PathBuf::new();
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_modified");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_modified");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    write(&dest_sub_file, "original content").expect("[ERROR]: failed to write the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "modification content").expect("[ERROR]: failed to write the file");

    let src_num = metadata(&src_sub_file)
        .ok()
        .and_then(|f| f.modified().ok())
        .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
        .map(|f| f.as_secs() as f64)
        .unwrap_or(0.0);

    let dest_num = metadata(&dest_sub_file)
        .ok()
        .and_then(|f| f.modified().ok())
        .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
        .map(|f| f.as_secs() as f64)
        .unwrap_or(0.0);

    let src_content =
        read_to_string(&src_sub_file).expect("[ERROR]: failed to read the source file");
    let dest_content =
        read_to_string(&dest_sub_file).expect("[ERROR]: failed to read the destination file");

    let src_file = src_sub_file
        .strip_prefix(&src_parent_dir)
        .expect("[ERROR]: failed to get the file");

    if src_num > dest_num {
        if src_content != dest_content {
            modified_file.push(&src_file);
        }
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let (file_modified, is_modified) = sync.src_file_modified();
    for file in file_modified {
        assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    }
    assert!(is_modified);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_file_created() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_creation");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }
    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let dest_sub_file = dest_sub_dir.join("dest_file_creation.txt");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let mut file_found = false;
    for entry in WalkDir::new(&dest_parent_dir) {
        let entry_path = entry
            .expect("[ERROR]: failed to get the entry")
            .path()
            .to_path_buf();
        if entry_path.is_file() {
            file_found = true;
        }
    }

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let created = sync.dest_file_created();
    assert_eq!(created, file_found);

    remove_file(&dest_sub_file).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_file_modified() {
    let mut modified_file = PathBuf::new();
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_modification");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_modification");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    write(&src_sub_file, "original content").expect("[ERROR]: failed to write the source file");
    thread::sleep(time::Duration::from_secs(1));
    write(&dest_sub_file, "modification content")
        .expect("[ERROR]: failed to write the destination file");

    let src_num = metadata(&src_sub_file)
        .ok()
        .and_then(|f| f.modified().ok())
        .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
        .map(|f| f.as_secs() as f64)
        .unwrap_or(0.0);

    let dest_num = metadata(&dest_sub_file)
        .ok()
        .and_then(|f| f.modified().ok())
        .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
        .map(|f| f.as_secs() as f64)
        .unwrap_or(0.0);

    let src_content =
        read_to_string(&src_sub_file).expect("[ERROR]: failed to read the source file");
    let dest_content =
        read_to_string(&dest_sub_file).expect("[ERROR]: failed to read the destination file");

    let dest_file = dest_sub_file
        .strip_prefix(&dest_parent_dir)
        .expect("[ERROR]: failed to get the file");

    if dest_num > src_num {
        if dest_content != src_content {
            modified_file.push(&dest_file);
        }
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let (file_modified, is_modified) = sync.dest_file_modified();
    for file in file_modified {
        assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    }
    assert!(is_modified);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_copy_src_to_destination() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_src");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_dest");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let src_sub_file = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    write(&src_sub_file, "original content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    sync.copy_src_to_dest();

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let src: Vec<PathBuf> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    let dest: Vec<PathBuf> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    assert_eq!(src, dest);

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_remove_dest_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_remove");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_remove");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    write(&dest_sub_file, "original content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    sync.remove_dest_file();

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let src: Vec<PathBuf> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    let dest: Vec<PathBuf> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    assert_eq!(src, dest);

    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_update_dest_file() {
    let mut src_content = String::new();
    let mut dest_content = String::new();

    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_update");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_update");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let (modified_src_file, src_modified) = sync.src_file_modified();
    if src_modified {
        sync.update_dest_file(modified_src_file.clone());
    }

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    for src_file in src_files {
        for dest_file in &dest_files {
            src_content =
                read_to_string(&src_file).expect("[ERROR]: failed to read the source file");
            dest_content =
                read_to_string(&dest_file).expect("[ERROR]: failed to read the destination file");
        }
    }
    assert_eq!(src_content, dest_content);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_remove_all_dest_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_rmv_all");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: false,
        delete: true,
        verbose: false,
        dry_run: false,
    };
    sync.remove_all_dest_files();

    let dest_files = sync.list_dest_files();
    let dest: Vec<PathBuf> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    assert_eq!(dest.len(), 0);

    remove_file(&dest_sub_file).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn src_files_present_or_not() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_file_present");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let src_sub_file = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let (dir_list, file_list) = sync.get_file_names();
    assert!(dir_list.len() != 0 && file_list.len() != 0);

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_has_duplicates() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_has_duplicates");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let src_sub_file1 = src_parent_dir.join("common.txt");
    let src_sub_file2 = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file1).expect("[ERROR]: failed to create a source file");
    File::create(&src_sub_file2).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let result = sync.has_duplicates();
    assert!(result == true);

    remove_file(&src_sub_file1).ok();
    remove_file(&src_sub_file2).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_filter_src_dirs() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_filter_dir");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let list_dirs = sync.list_src_dirs();
    let filtered = filter_src_dir(&list_dirs, &sync.source);
    let dir = src_sub_dir
        .iter()
        .last()
        .expect("[ERROR]: failed to get the lastname")
        .to_string_lossy()
        .to_string();

    assert!(filtered.len() != 0 && !dir.is_empty());
    assert_eq!(filtered[0], PathBuf::from(dir));

    remove_dir_all(src_sub_dir).ok();
    remove_dir_all(src_parent_dir).ok();
}

#[test]
fn test_filter_dest_dirs() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_filter_dir");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_sub_dir = dest_parent_dir.join("test_dir");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let list_dirs = sync.list_dest_dirs();
    let filtered = filter_dest_dir(&list_dirs, &sync.destination);
    let dir = dest_sub_dir
        .iter()
        .last()
        .expect("[ERROR]: failed to get the lastname")
        .to_string_lossy()
        .to_string();

    assert!(filtered.len() != 0 && !dir.is_empty());
    assert_eq!(filtered[0], PathBuf::from(dir));

    remove_dir_all(dest_sub_dir).ok();
    remove_dir_all(dest_parent_dir).ok();
}

#[test]
fn test_filter_src_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_filter_file");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let src_sub_file = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let list_files = sync.list_src_files();
    let filtered = filter_src_file(&list_files);
    let file = src_sub_file
        .file_name()
        .expect("[ERROR]: failed to get the filename");

    assert!(filtered.len() != 0 && !file.is_empty());
    assert_eq!(filtered[0], PathBuf::from(file));

    remove_file(src_sub_file).ok();
    remove_dir_all(src_sub_dir).ok();
    remove_dir_all(src_parent_dir).ok();
}

#[test]
fn test_filter_dest_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_filter_file");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let list_files = sync.list_dest_files();
    let filtered = filter_dest_file(&list_files);
    let file = dest_sub_file
        .file_name()
        .expect("[ERROR]: failed to get the filename");

    assert!(filtered.len() != 0 && !file.is_empty());
    assert_eq!(filtered[0], PathBuf::from(file));

    remove_file(dest_sub_file).ok();
    remove_dir_all(dest_sub_dir).ok();
    remove_dir_all(dest_parent_dir).ok();
}

// ----------------------------------------------- Edge Case ------------------------------------------------

#[test]
fn test_src_dir_present() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_presence");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        !sync.src_dest_dir_present(),
        "[ERROR]: source directory not detected"
    );

    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_dest_dir_present() {
    let home_dir = env::var("HOME").expect("HOME env variable not set");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_presence");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        !sync.src_dest_dir_present(),
        "[ERROR]: destination directory not detected"
    );

    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dir_not_present() {
    let sync = SyncData {
        source: PathBuf::new(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(!sync.src_dest_dir_present(), "No directory is detected");
}

#[test]
fn test_multiple_commands_selected() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");

    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_command");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_command");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let mut sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: true,
        verbose: false,
        dry_run: false,
    };

    assert!(
        !sync.single_command_selected(),
        "[ERROR]: expected only one command, but multiple commands reported"
    );

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_no_command_selected() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");

    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_command");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_command");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let mut sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: false,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert!(
        !sync.single_command_selected(),
        "[ERROR]: expected only one command, but no commands reported"
    );

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_source_empty() {
    let mut list_file: Vec<PathBuf> = Vec::new();
    let mut list_dir: Vec<PathBuf> = Vec::new();

    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmpsrc_listing");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let entries = read_dir(&src_parent_dir).expect("[ERROR]: failed to read the source directory");
    let searched_files: Vec<PathBuf> = entries.filter_map(|f| f.ok()).map(|f| f.path()).collect();

    let src_files = sync.list_src_files();
    let src_dirs = sync.list_src_dirs();

    for file in src_files {
        list_file.push(file);
    }

    for entry in src_dirs {
        if entry == src_parent_dir {
            continue;
        }

        let entry_path = entry.iter().last();
        let dir = entry_path.expect("[ERROR]: failed to get the file");
        list_dir.push(PathBuf::from(dir.display().to_string()));
    }

    assert!(list_file.len() == 0 && list_dir.len() == 0 && searched_files.len() == 0);

    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_dest_empty() {
    let mut list_file: Vec<PathBuf> = Vec::new();
    let mut list_dir: Vec<PathBuf> = Vec::new();

    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpsrc_listing");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let entries = read_dir(&dest_parent_dir).expect("[ERROR]: failed to read the directory");
    let searched_files: Vec<PathBuf> = entries.filter_map(|f| f.ok()).map(|f| f.path()).collect();

    let dest_files = sync.list_dest_files();
    let dest_dirs = sync.list_dest_dirs();

    for file in dest_files {
        list_file.push(file);
    }

    for entry in dest_dirs {
        if entry == dest_parent_dir {
            continue;
        }

        let entry_path = entry.iter().last();
        let dir = entry_path.expect("[ERROR]: failed to get the file");
        list_dir.push(PathBuf::from(dir.display().to_string()));
    }

    assert!(list_file.len() == 0 && list_dir.len() == 0 && searched_files.len() == 0);

    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_timestamp_exists_dest_not() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_one_timestamp");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_one_timestamp");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files_list = sync.list_src_files();
    let dest_files_list = sync.list_dest_files();

    let src_timestamp = sync.file_timestamp(src_files_list.clone(), &sync.source);
    let dest_timestamp = sync.file_timestamp(dest_files_list.clone(), &sync.destination);

    for (path, src_time) in &src_timestamp {
        if !dest_timestamp.contains_key(path) {
            assert!(*src_time != 0.0)
        }
    }

    assert!(
        !src_timestamp.is_empty(),
        "source timestamps should not be empty"
    );

    assert!(
        dest_timestamp.is_empty(),
        "destination timestamps should be empty"
    );

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_timestamp_greater_than_dest() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_two_timestamp");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_two_timestamp");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files_list = sync.list_src_files();
    let dest_files_list = sync.list_dest_files();

    let src_timestamp = sync.file_timestamp(src_files_list.clone(), &sync.source);
    let dest_timestamp = sync.file_timestamp(dest_files_list.clone(), &sync.destination);

    for (path, src_time) in &src_timestamp {
        let dest = dest_timestamp.get(path);
        match dest {
            Some(dest_time) => {
                assert!(src_time > dest_time)
            }
            None => {}
        }
    }

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_timestamp_greater_than_src() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_three_duration");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_three_duration");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files_list = sync.list_src_files();
    let dest_files_list = sync.list_dest_files();

    let src_timestamp = sync.file_timestamp(src_files_list.clone(), &sync.source);
    let dest_timestamp = sync.file_timestamp(dest_files_list.clone(), &sync.destination);

    for (path, dest_time) in &dest_timestamp {
        let src = src_timestamp.get(path);
        match src {
            Some(src_time) => {
                assert!(dest_time > src_time)
            }
            None => {}
        }
    }

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_timestamp_equal_to_dest() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_four_duration");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_four_duration");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files_list = sync.list_src_files();
    let dest_files_list = sync.list_dest_files();

    let src_timestamp = sync.file_timestamp(src_files_list.clone(), &sync.source);
    let dest_timestamp = sync.file_timestamp(dest_files_list.clone(), &sync.destination);

    for (path, src_time) in &src_timestamp {
        let dest = dest_timestamp.get(path);
        match dest {
            Some(dest_time) => {
                assert!(src_time == dest_time)
            }
            None => {}
        }
    }

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_file_not_created() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmpsrc_not_created");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }
    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let created = sync.src_file_created();
    assert_eq!(
        created, false,
        "Source file incorrectly detected as created"
    );

    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_src_file_not_modified() {
    let mut modified_file = PathBuf::new();
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_not_modified");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_not_modified");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    thread::sleep(std::time::Duration::from_secs(1));
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");

    let src_num = metadata(&src_sub_file)
        .ok()
        .and_then(|f| f.modified().ok())
        .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
        .map(|f| f.as_secs() as f64)
        .unwrap_or(0.0);

    let dest_num = metadata(&dest_sub_file)
        .ok()
        .and_then(|f| f.modified().ok())
        .and_then(|f| f.duration_since(UNIX_EPOCH).ok())
        .map(|f| f.as_secs() as f64)
        .unwrap_or(0.0);

    let src_content =
        read_to_string(&src_sub_file).expect("[ERROR]: failed to read the source file");
    let dest_content =
        read_to_string(&dest_sub_file).expect("[ERROR]: failed to read the destination file");

    let dest_file = dest_sub_file
        .strip_prefix(&dest_parent_dir)
        .expect("[ERROR]: failed to get the file");

    if dest_num > src_num {
        if dest_content != src_content {
            modified_file.push(&dest_file);
        }
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let (file_modified, is_modified) = sync.src_file_modified();
    for file in &file_modified {
        assert_eq!(file.to_string_lossy(), modified_file.to_string_lossy());
    }
    assert!(!is_modified);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_file_not_created() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_not_created");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let created = sync.dest_file_created();
    assert_eq!(
        created, false,
        "Destination file incorrectly detected as created"
    );

    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_dest_equal() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_src_copy");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_dest_copy");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    sync.copy_src_to_dest();

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let srcfile: Vec<PathBuf> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    let destfile: Vec<PathBuf> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    assert_eq!(srcfile, destfile);
    assert!(srcfile.len() == 0 && destfile.len() == 0);

    let src_dirs = sync.list_src_dirs();
    let dest_dirs = sync.list_dest_dirs();

    let mut srcdirs: Vec<PathBuf> = Vec::new();
    let mut destdirs: Vec<PathBuf> = Vec::new();

    for src in src_dirs {
        if src == sync.source {
            continue;
        }

        srcdirs.push(src);
    }

    for dest in dest_dirs {
        if dest == sync.destination {
            continue;
        }

        destdirs.push(dest);
    }

    assert_eq!(srcdirs, destdirs);
    assert!(srcdirs.len() == 0 && destdirs.len() == 0);

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_files_not_removed() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_not_remove");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_not_remove");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let srcfile: Vec<PathBuf> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    let destfile: Vec<PathBuf> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    assert_eq!(srcfile, destfile);
    assert!(srcfile.len() != 0 && destfile.len() != 0);

    let mut srcdirs: Vec<PathBuf> = Vec::new();
    let mut destdirs: Vec<PathBuf> = Vec::new();

    let src_dirs = sync.list_src_dirs();
    let dest_dirs = sync.list_dest_dirs();

    for src in src_dirs {
        if src == sync.source {
            continue;
        }
        let source = src
            .to_string_lossy()
            .to_string()
            .split("/")
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string();
        srcdirs.push(PathBuf::from(source));
    }

    for dest in dest_dirs {
        if dest == sync.destination {
            continue;
        }
        let directory = dest
            .to_string_lossy()
            .to_string()
            .split("/")
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string();
        destdirs.push(PathBuf::from(directory));
    }

    assert_eq!(srcdirs, destdirs);
    assert!(srcdirs.len() != 0 && destdirs.len() != 0);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_file_not_updated() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_not_update");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_not_update");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let mut src_content = String::new();
    let mut dest_content = String::new();

    for src_file in src_files {
        for dest_file in &dest_files {
            src_content =
                read_to_string(&src_file).expect("[ERROR]: failed to read the source file");
            dest_content =
                read_to_string(&dest_file).expect("[ERROR]: failed to read the destination file");
        }
    }

    assert!(src_content != dest_content);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_no_dest_file() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_no_dest_file");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_no_dest_file");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let mut src_content = String::new();
    for src_file in src_files {
        src_content = read_to_string(&src_file).expect("[ERROR]: failed to read the source file");
    }

    assert!(!src_content.is_empty() && dest_files.is_empty());

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_no_src_file() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_no_src_file");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_no_src_file");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    thread::sleep(time::Duration::from_secs(1));
    write(&dest_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let mut dest_content = String::new();
    for dest_file in dest_files {
        dest_content =
            read_to_string(&dest_file).expect("[ERROR]: failed to read the destination file");
    }

    assert!(!dest_content.is_empty() && src_files.is_empty());

    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}
