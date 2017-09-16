extern crate rand;

use std::env;
use std::fs;

use std::path::Path;

use self::rand::Rng;

/// a temp path provider
///
/// path will be __**removed**__ when this struct dropped.
#[derive(Debug)]
pub struct Temp {
    pub path: String,
}

impl Temp {
    /// construct Temp object
    ///
    /// generate random path that will be deleted when leaving scope.
    ///
    /// # behavior
    ///
    /// generate path with random 10 letters basename under the temp directory.
    ///
    /// it has checked that path is not exists when path generated.
    ///
    /// this function **doesn't** create any files or directories.
    pub fn new() -> Result<Temp, String> {
        for _ in 1..10 {
            // try 10 random names

            let tmppath = make_tmp_file_name();

            if is_file_exists(tmppath.as_str()) {
                // regenerate filename if already exists
                continue;
            }

            return Ok(Temp { path: tmppath });
        }

        // failed 10 times

        Err("no names available.".to_string())
    }

    /// constract Temp with specified path
    ///
    /// specified path will be deleted when leaving scope of returned Temp object.
    pub fn as_temp(path: String) -> Temp {
        Temp { path: path }
    }
}

impl Drop for Temp {
    /// delete self.path if it exists.
    ///
    /// it will be deleted recursively if path is directory
    fn drop(&mut self) {
        if is_file_exists(self.path.as_str()) == false {
            return;
        }

        let path = Path::new(self.path.as_str());

        if path.is_file() {
            let _ = fs::remove_file(self.path.as_str());
            return;
        }

        if path.is_dir() {
            let _ = fs::remove_dir_all(self.path.as_str());
            return;
        }
    }
}

fn is_file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn make_tmp_file_name() -> String {
    let dir = env::temp_dir();
    let name = rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect::<String>();
    return dir.join(name).to_str().unwrap().to_string();
}

#[cfg(test)]
mod test {

use std::fs::File;
use std::path::Path;
use super::*;

    #[test]
    fn test_temp_file_deleted() {
        let name = cotest_create_temp_file();
        assert_eq!(Path::new(name.as_str()).exists(), false);
    }

    fn cotest_create_temp_file() -> Box<String> {
        let a = Temp::new().unwrap();
        File::create(a.path.as_str()).unwrap();
        assert_eq!(Path::new(a.path.as_str()).exists(), true);
        Box::new(a.path.clone())
    }

    #[test]
    fn test_temp_dir_deleted() {
        let name = cotest_create_temp_dir();
        assert_eq!(Path::new(name.as_str()).exists(), false);
    }

    fn cotest_create_temp_dir() -> String {
        let a = Temp::new().unwrap();
        fs::create_dir(a.path.as_str()).unwrap();
        assert_eq!(Path::new(a.path.as_str()).exists(), true);
        a.path.clone()
    }

    #[test]
    fn test_exsits_file_as_temp() {
        let name = cotest_exists_file_as_temp();
        assert_eq!(Path::new(name.as_str()).exists(), false);
    }

    fn cotest_exists_file_as_temp() -> String {
        let s = make_tmp_file_name();
        let _ = fs::create_dir(s.as_str());
        let _temp = Temp::as_temp(s.clone());
        assert_eq!(Path::new(s.as_str()).exists(), true);
        s
    }

}