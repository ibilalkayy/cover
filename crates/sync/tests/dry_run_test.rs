use std::path::PathBuf;

use sync::dry_run::dry_run_flag;

fn make_path(s: &str) -> PathBuf {
    PathBuf::from(s)
}

#[test]
fn test_dry_run_flag_behavior() {
    // Fake data setup
    let source = make_path("/src");
    let destination = make_path("/dest");

    // Simulate files in source and destination
    let source_files = vec![
        (make_path("/src/file1.txt"), 200), // newer than destination
        (make_path("/src/file2.txt"), 100), // same timestamp
        (make_path("/src/file3.txt"), 150), // new file not in dest
    ];

    let destination_files = vec![
        (make_path("/dest/file1.txt"), 100), // outdated
        (make_path("/dest/file2.txt"), 100), // up to date
        (make_path("/dest/extra.txt"), 50),  // extra file not in source
    ];

    // Capture printed output
    let output = std::panic::catch_unwind(|| {
        dry_run_flag(&source_files, &destination_files, &source, &destination);
    });

    assert!(output.is_ok(), "Function should not panic");
}
