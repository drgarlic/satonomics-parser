use std::process::Command;

pub struct Terminal;

impl Terminal {
    // https://github.com/RustScan/RustScan/wiki/Thread-main-paniced-at-too-many-open-files
    // https://apple.stackexchange.com/questions/32235/how-to-properly-increase-a-ulimit-n-limits
    pub fn increase_open_files_limit() {
        let _ = Command::new("ulimit").arg("-n").arg("10000").output();
    }
}
