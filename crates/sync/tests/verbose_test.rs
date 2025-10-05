use std::path::PathBuf;

use sync::verbose::verbose_flag;

// Simple helper to create path easily
fn make_path(s: &str) -> PathBuf {
    PathBuf::from(s)
}

#[test]
fn test_verbose_flag_logs() {
    // --- Setup mock paths ---
    let source = make_path("/src");
    let destination = make_path("/dest");

    // Source contains 3 files
    let source_files = vec![
        (make_path("/src/file1.txt"), 200), // newer → update
        (make_path("/src/file2.txt"), 100), // same → skip
        (make_path("/src/file3.txt"), 150), // new → copy
    ];

    // Destination contains 3 files
    let destination_files = vec![
        (make_path("/dest/file1.txt"), 100), // outdated
        (make_path("/dest/file2.txt"), 100), // up-to-date
        (make_path("/dest/old.txt"), 80),    // extra → delete
    ];

    // Verbose data (pretty times + sizes)
    let src_files = vec![
        (
            make_path("/src/file1.txt"),
            200,
            "12-Sep-2025 10:00:00 AM".to_string(),
            (1.2, "KB"),
        ),
        (
            make_path("/src/file2.txt"),
            100,
            "12-Sep-2025 09:00:00 AM".to_string(),
            (2.0, "KB"),
        ),
        (
            make_path("/src/file3.txt"),
            150,
            "12-Sep-2025 09:30:00 AM".to_string(),
            (3.0, "KB"),
        ),
    ];

    let dest_files = vec![
        (
            make_path("/dest/file1.txt"),
            100,
            "12-Sep-2025 08:00:00 AM".to_string(),
            (1.1, "KB"),
        ),
        (
            make_path("/dest/file2.txt"),
            100,
            "12-Sep-2025 09:00:00 AM".to_string(),
            (2.0, "KB"),
        ),
        (
            make_path("/dest/old.txt"),
            80,
            "12-Sep-2025 07:00:00 AM".to_string(),
            (0.5, "KB"),
        ),
    ];

    // --- Capture printed output ---
    let output = std::panic::catch_unwind(|| {
        verbose_flag(
            &source_files,
            &destination_files,
            &source,
            &destination,
            &src_files,
            &dest_files,
        );
    });

    assert!(output.is_ok(), "verbose_flag should not panic");
}
