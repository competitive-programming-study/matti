use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

const LIB_FILE: &str = "lib";
const SRC_DIR: &str = "src";
const TEST_DIR: &str = "tests";
const TEST_EXTENSION: &str = "rs";
const MOD_FILE: &str = "mod";

pub fn get_file_name(p: &Path) -> String {
    p.file_stem()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_string()
}

pub fn get_project_dir() -> PathBuf {
    Result::expect(env::current_dir(), "Cannot get project directory")
        .parent()
        .expect("automation cannot be located in root")
        .to_path_buf()
}

pub fn get_project_dir_name() -> String {
    get_project_dir()
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_string()
}

pub fn get_source_dir() -> PathBuf {
    get_project_dir().join(SRC_DIR)
}

pub fn get_lib_path() -> PathBuf {
    get_source_dir().join(LIB_FILE).with_extension("rs")
}

pub fn get_set_path(set_name: &str) -> PathBuf {
    get_source_dir().join(set_name)
}

pub fn get_exercise_path(set_name: &str, exercise_name: &str) -> PathBuf {
    get_set_path(set_name)
        .join(exercise_name)
        .with_extension("rs")
}

pub fn get_mod_path(set_name: &str) -> PathBuf {
    get_set_path(set_name).join(MOD_FILE).with_extension("rs")
}

pub fn tmp_file(p: &Path) -> PathBuf {
    p.with_extension("tmp")
}

pub fn get_test_path(exercise_name: &str) -> PathBuf {
    get_project_dir()
        .join(TEST_DIR)
        .join(format!("test_{exercise_name}"))
        .with_extension(TEST_EXTENSION)
}
