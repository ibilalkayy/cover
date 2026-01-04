// use cover_files::archive::archive::ArchiveData;
// use std::{
//     env,
//     fs::{create_dir_all, remove_dir_all},
//     path::PathBuf,
// };

// #[test]
// fn test_src_dir_present() {
//     let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
//     let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_present");

//     if src_parent_dir.exists() {
//         remove_dir_all(&src_parent_dir).ok();
//     }
//     create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

//     let archive = ArchiveData {
//         source: src_parent_dir.clone(),
//         zip: true,
//         tar: false,
//         encrypt: false,
//         timestamp: false,
//     };

//     assert!(
//         archive.src_dir_present(),
//         "[ERROR]: source directory not detected"
//     );

//     remove_dir_all(&src_parent_dir).ok();
// }

// #[test]
// fn test_single_flag_provided() {
//     let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
//     let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_zip_flag");

//     if src_parent_dir.exists() {
//         remove_dir_all(&src_parent_dir).ok();
//     }

//     create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

//     let archive = ArchiveData {
//         source: src_parent_dir.clone(),
//         zip: true,
//         tar: false,
//         encrypt: false,
//         timestamp: false,
//     };

//     assert!(
//         archive.single_flag_provided(),
//         "[ERROR]: expected one flag, but multiple are reported"
//     );

//     remove_dir_all(&src_parent_dir).ok();
// }

// #[test]
// fn test_src_file_zipped() {
//     let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
//     let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_zip");

//     if src_parent_dir.exists() {
//         remove_dir_all(&src_parent_dir).ok();
//     }

//     create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

//     let archive = ArchiveData {
//         source: src_parent_dir.clone(),
//         zip: true,
//         tar: false,
//         encrypt: false,
//         timestamp: false,
//     };

//     assert!(archive.src_file_zipped(), "[ERROR]: failed to zip the file");

//     remove_dir_all(src_parent_dir).ok();
// }
