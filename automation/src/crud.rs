use crate::path_util::*;

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead as _, BufReader, Error, ErrorKind, Result, Write as _},
    path::Path,
};

const IGNORE: &str = ".ignore";

///
/// Check if a set names is valid:
/// 
/// A set name is valid if the corresponding path doesn't exist or 
/// the path exists and inside the set directory doesn't exist an
/// ignore file
/// 
fn is_valid_set(set_name: &str) -> Result<bool> {
    let path = get_set_path(set_name).join(IGNORE);
    Ok(!fs::exists(path)?)

}


fn create_test_file(set_name: &str, exercise_name: &str) -> Result<bool> {
    let test_path = get_test_path(exercise_name);
    let project_name = get_project_dir_name();
    if project_name.is_empty() {
        return Err(Error::other("Cannot extrapolate project name"));
    }
    if test_path.exists() {
        return Ok(false);
    }

    let test_content = format!(
        "#![allow(unused_imports)]\n\
        use {project_name}::{set_name}::{exercise_name};\n\
        use {project_name}::test_util::{{TestCase}};\n\
        \n\
        #[test]\n\
        fn test_1() -> (){{\n\
        \ttodo!()\n\
        }}"
    );

    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(test_path)?
        .write_all(test_content.as_bytes())?;
    Ok(true)
}

/// Creates a new exercise given a set, and a name
///
/// Every exercise should be located in a `set` directory that contains
/// a `mod.rs` file for module import. Each set is linked to the `main.rs` file
/// through the `lib.rs` file.
///
/// When creating an exercise, if the set exists we create the file there
/// updating the set's `mod.rs` file, or we create the set directory updating
/// the `lib.rs` file.
///
/// Each exercise file maps in a test file, presumably containing tests cases
/// that can be asserted. When creating a new exercise we also create the corresponding
/// test file.
///
pub fn new_exercise(set_name: &str, exercise_name: &str) -> Result<()> {

    if !is_valid_set(set_name)?  {
        return Err(Error::new(ErrorKind::PermissionDenied,"Shoudn't modify this directory [IGNORE FILE SETTED]"))
    }

    let set_dir = get_set_path(set_name);
    // Create the set directory if it doesn't exist
    if !set_dir.exists() {
        fs::create_dir(&set_dir)?;

        // Create mod.rs inside the set directory
        File::create_new(get_mod_path(set_name))?;

        // Update lib.rs in src folder to add this new set
        let lib_path = get_lib_path();
        OpenOptions::new()
            .append(true)
            .open(lib_path)?
            .write_fmt(format_args!("\npub mod {set_name};"))?;
    }

    // Create new exercise file: set_dir/exercise_name.rs
    File::create_new(get_exercise_path(set_name, exercise_name))?;

    // Append `pub mod exercise_name;` to mod.rs
    OpenOptions::new()
        .append(true)
        .open(get_mod_path(set_name))?
        .write_fmt(format_args!("\npub mod {exercise_name};"))?;

    // Create associated test file in tests directory
    create_test_file(set_name, exercise_name)?;
    Ok(())
}

/// Removes a line from a file specified by `Path file`
///
/// Returns a `io::Result<bool>` which would be `true` if the corresponding
/// file was empty or was left empty after the line removal
///
fn remove_line(file: &Path, line: &str) -> Result<bool> {
    let lines = BufReader::new(File::open(file)?).lines();

    let lines_to_write: Vec<String> = lines.filter_map(Result::ok).filter(|s| s != line).collect();

    if lines_to_write.is_empty() {
        OpenOptions::new().write(true).truncate(true).open(file)?;
        Ok(false)
    } else {
        let tmp_path = &tmp_file(file);
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(tmp_path)?
            .write(lines_to_write.join("\n").as_bytes())?;
        fs::rename(tmp_path, file)?;
        Ok(true)
    }
}

/// Attempts to remove a `set` directory, if no exercises are present
///
/// By convention a `set` directory doesn't contain any subdirectories
/// and contains a special file `mod.rs` used for imports. If the removal
/// of an exercise would leave the directory empty (not accounting for the import file)
/// the set would be removed
///
/// Returns a `io::Result<bool>` which indicates whether the set was actually removed
fn remove_set(set_name: &str) -> Result<bool> {
    let set_path = get_set_path(set_name);

    let mod_file = get_file_name(&get_mod_path(set_name));

    for entry_res in set_path.read_dir()? {
        let entry = entry_res?;
        if entry.metadata()?.is_file() {
            if mod_file != get_file_name(&entry.path()) {
                return Ok(false);
            }
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Directory provided doesn't comply with set directory specification",
            ));
        }
    }

    // Remove the set directory itself
    fs::remove_dir_all(set_path)?;

    // Remove set entry from lib.rs
    remove_line(&get_lib_path(), &format!("pub mod {set_name};"))?;
    Ok(true)
}

/// Removes an exercise from a set directory, deleting the corresponding test
/// file and updating the `mod.rs` file
///
pub fn remove_exercise(set_name: &str, exercise_name: &str) -> Result<bool> {
    if !is_valid_set(set_name)?  {
        return Err(Error::new(ErrorKind::PermissionDenied,"Shoudn't modify this directory [IGNORE FILE SETTED]"))
    }


    // Remove exercise file
    fs::remove_file(get_exercise_path(set_name, exercise_name))?;
    // Remove test file
    fs::remove_file(get_test_path(exercise_name))?;

    // Remove exercise line from mod.rs
    remove_line(
        &get_mod_path(set_name),
        &format!("pub mod {exercise_name};"),
    )?;

    // Check if set is empty and remove it if so
    remove_set(set_name)
}

/// Indexes each exercise inside a set directory, potentially updating the
/// corresponding `mod.rs` file and creating tests files if not present
///
fn build_index(set: &str) -> Result<()> {
    let mod_path = get_mod_path(set);
    let set_path = get_set_path(set);
    //Converts the exercises paths into readable strings
    let exercises: Vec<String> = set_path
        .read_dir()?
        .map_while(Result::ok)
        .filter(
            |entry| {
                entry.metadata().map(|m| m.is_file()).unwrap_or(false) && (entry.path() != mod_path)
            }, //we exclude the mod file
        )
        .map(|entry| get_file_name(&entry.path()))
        .collect();
    for e in exercises.iter() {
        create_test_file(set, e)?;
    }

    let mods_to_write = exercises
        .iter()
        .map(|s| format!("pub mod {s};"))
        .collect::<Vec<String>>()
        .join("\n");

    let mod_tmp = tmp_file(&mod_path);
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&mod_tmp)?
        .write_all(mods_to_write.as_bytes())?;

    fs::rename(mod_tmp, mod_path)?;
    Ok(())
}

/// Walks every set indexing each exercise, rebuilding the `lib.rs` file
/// and potentially updating the `mod.rs` files and creating test files if not
/// present
///
pub fn update_conf() -> Result<()> {
    let lib_path = get_lib_path();

    let mut mods: Vec<String> = BufReader::new(File::open(&lib_path)?)
        .lines()
        .map(|x: Result<String>| String::from(x.unwrap().split_whitespace().last().unwrap()))
        .collect();
    //read the content of lib and parse the set imports

    let sets = fs::read_dir(get_source_dir())?
        .map_while(Result::ok)
        .filter(|d| d.metadata().as_ref().map(|m| m.is_dir()).unwrap_or(false))
        .map(|s| s.path())
        .map(|p| get_file_name(&p))
        .filter(|s|is_valid_set(s).is_ok_and(|x|x))
        .collect::<Vec<String>>();

    for s in sets {
        build_index(&s)?;
        let set_to_search = s + ";";
        if !mods.contains(&set_to_search) {
            mods.push(set_to_search);
        }
    }

    //write all mods to lib.tmp
    let lib_tmp = tmp_file(&lib_path);

    let mods_to_write = mods
        .into_iter()
        .map(|s| format!("pub mod {s}"))
        .collect::<Vec<String>>()
        .join("\n");

    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&lib_tmp)?
        .write_all(mods_to_write.as_bytes())?;

    fs::rename(&lib_tmp, &lib_path)?;
    Ok(())
}
