use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_temp_test_dir(test_name: &str) -> PathBuf {
    let mut test_dir = env::temp_dir();
    test_dir.push("zamm/tests");
    test_dir.push(test_name);
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).unwrap_or_else(|_| {
            panic!("Can't remove temp test dir at {}", test_dir.display())
        });
    }
    fs::create_dir_all(&test_dir).unwrap_or_else(|_| {
        panic!("Can't create temp test dir at {}", test_dir.display())
    });
    test_dir
}
