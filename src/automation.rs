use std::{env, io};
#[allow(unused)]

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path};

const LIB_FILE: &str = "lib";
const SRC_DIR: &str = "src";
const TEST_DIR: &str = "tests";
const TEST_DEFAULT_CONTENT: &str = "\n#[test]\nfn test_1() -> (){\n\ttodo!()\n}";
const MOD_FILE: &str = "mod";


/**
* in the src folder, every folder refers to a set* folder
*
* The file lib.rs imports each set as a module
*
* Every set* folder contains one mod.rs file that imports each exercise.rs file
*
* The test folder contains a file _test_<exercise>.rs associated to one exercise
*
* We need a method:
*
* add_exercise(exe: &str, set: &s)
* If the set doesn't exist, it creates it: updating the lib.rs file in the src dir
* and adding the mod.rs file
* when creating an exercise, we add the file to the set and update the mod.rs and add the _test file
* to the tests directory
*
* remove_exercise(exe: &str, set: &s)
* remove the exercise from the set, updating the mod.rs file, also remove the test file
*
* if the set is empty remove it, updating the lib.rs file
*

*/

/**
 * Creates a new exercise given a set name
 * 
 * if the set is not present it will be created along with a mod.rs file
 * and it will be linked with the lib.rs file
 * 
 * A test file for the exercise will be created in the default test directory
 * 
 */

fn test_file_path(exercise_name: &str) -> Result<std::path::PathBuf,std::io::Error> {
    let test_name = format!("test_{}.rs", exercise_name);
    let base_dir = env::current_dir()?;
    Ok(base_dir.join(TEST_DIR).join(test_name))
}

pub fn new_exercise(set_name: &str, exercise_name: &str) -> io::Result<()> {
    let cwd = env::current_dir()?;

    // Directory for the set: cwd/set_name
    let set_dir = cwd.join(SRC_DIR).join(set_name);

    // Create the set directory if it doesn't exist
    if !set_dir.exists() {
        fs::create_dir(&set_dir)?;

        // Create mod.rs inside the set directory
        let mod_path = set_dir.join(format!("{}.rs", MOD_FILE));
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&mod_path)?;

        // Update lib.rs in src folder to add this new set
        let lib_path = cwd.join(SRC_DIR).join(format!("{}.rs", LIB_FILE));
        let mut lib_file = OpenOptions::new()
            .append(true)
            .open(&lib_path)?;
        writeln!(lib_file, "pub mod {};", set_name)?;
    }

    // Create new exercise file: set_dir/exercise_name.rs
    let exercise_path = set_dir.join(format!("{}.rs", exercise_name));
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&exercise_path)?;

    // Append `pub mod exercise_name;` to mod.rs
    let mod_path = set_dir.join(format!("{}.rs", MOD_FILE));
    let mut mod_file = OpenOptions::new()
        .append(true)
        .open(&mod_path)?;
    writeln!(mod_file, "pub mod {};", exercise_name)?;

    // Create associated test file in tests directory
    let test_path = test_file_path(exercise_name)?;
    let mut test_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&test_path)?;
    writeln!(test_file, "{}", TEST_DEFAULT_CONTENT)?;

    Ok(())
}

fn rem_line_from_file(file: &Path, line: &str, allow_duplicates: bool) -> io::Result<bool> {
    let reader = std::io::BufReader::new(fs::File::open(file)?);
    let tmp_path = file.with_extension("tmp");
    let tmp_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp_path)?;
    let mut writer = std::io::BufWriter::new(tmp_file);
    let mut found_count = 0;

    for line_res in reader.lines() {
        let current_line = line_res?;
        if current_line == line {
            found_count += 1;
            if !allow_duplicates && found_count > 1 {
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Duplicate lines found"));
            }
            // skip writing this line (removing it)
        } else {
            writeln!(writer, "{}", current_line)?;
        }
    }
    writer.flush()?;

    if found_count == 0 {
        fs::remove_file(&tmp_path)?; // no changes, remove tmp
        Ok(false)
    } else {
        fs::rename(&tmp_path, file)?; // replace original with tmp
        Ok(true)
    }
}

fn remove_set(set_path: &Path) -> io::Result<bool> {
    let entries = fs::read_dir(set_path)?;
    let mut file_count = 0;
    let mut mod_file_found = false;

    for entry_res in entries {
        let entry = entry_res?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            file_count += 1;
            let file_name = entry.file_name();
            let line = file_name.to_str().expect("Can't get filename");
            if  line ==  format!("{}.rs", MOD_FILE) {
                mod_file_found = true;
            }
            if file_count > 1 {
                return Ok(false); // More than mod.rs, set not empty
            }
        }
    }

    if !(file_count == 1 && mod_file_found) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Set directory structure invalid"));
    }

    // Remove mod.rs file
    let mod_path = set_path.join(format!("{}.rs", MOD_FILE));
    fs::remove_file(&mod_path)?;

    // Remove the set directory itself
    fs::remove_dir(set_path)?;

    // Remove set entry from lib.rs
    let src_dir = env::current_dir()?.join(SRC_DIR);
    let lib_path = src_dir.join(LIB_FILE).with_extension("rs");
    let set_name = set_path.file_name().unwrap().to_str().unwrap();
    rem_line_from_file(&lib_path, &format!("pub mod {};", set_name), false)
}

pub fn remove_exercise(set_name: &str, exercise_name: &str) -> io::Result<bool> {
    let cwd = env::current_dir()?;
    let set_path = cwd.join(SRC_DIR).join(set_name);

    // Remove exercise file
    let exercise_path = set_path.join(format!("{}.rs", exercise_name));
    fs::remove_file(&exercise_path)?;

    // Remove test file
    let test_path = test_file_path(exercise_name)?;
    fs::remove_file(&test_path)?;

    // Remove exercise line from mod.rs
    let mod_path = set_path.join(format!("{}.rs", MOD_FILE));
    rem_line_from_file(&mod_path, &format!("pub mod {};", exercise_name), false)?;

    // Check if set is empty and remove it if so
    remove_set(&set_path)
}

/**
 * Indexes all exercises inside each set, updates the MOD_FILE for each
 * set and creates tests files that dont exist
 */
fn build_index(path: &Path) ->io::Result<()> {
    let mod_path =  &path.join(MOD_FILE).with_extension("rs");
    let mod_tmp =   &path.join(MOD_FILE).with_extension("tmp");
    let mod_tmp_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(mod_tmp)?;
    let mut writer = BufWriter::new(&mod_tmp_file);

    for entry_res in fs::read_dir(path)? {
        let entry = entry_res?;
        if entry.metadata()?.is_file() {
            let e_path = entry.path();
            if e_path == *mod_path || e_path == *mod_tmp {
                continue;
            }

            let exercise_file  = entry.file_name();
            let exercise_name = Path::new(&exercise_file)
                .file_stem()
                .unwrap()
                .to_string_lossy();
            let test_path = test_file_path(&exercise_name)?;
            writeln!(&mut writer,"pub mod {};",exercise_name)?;
            //check if test file exists or create it
            if !Path::exists(&test_path) {
                let test_file = OpenOptions::new().create(true).write(true).open(&test_path)?;
                writeln!(&test_file,"{}",TEST_DEFAULT_CONTENT)?;
            }
            
        } 
    };

    writer.flush()?;
    fs::rename(mod_tmp,mod_path)?;
    Ok(())

}

/**
 * Updates the configuration for each set, also adds each set to the lib directory
 */
pub fn update_conf() -> io::Result<()> {
    let lib_path = env::current_dir()?.as_path().join(SRC_DIR).join(LIB_FILE).with_extension("rs");
    let mut mods: Vec<String> = BufReader::new(File::open(&lib_path)?)
        .lines()
        .map(|x: Result<String, io::Error>|
            String::from(
                x.unwrap()
                .split_whitespace()
                .last()
                .unwrap()))
        .collect();
    //read the content of lib and parse the set imports
    for set_res in fs::read_dir(lib_path.parent().unwrap())? {
        let set = set_res?;
        //we're in a set
        if set.metadata()?.is_dir() {
            //index the set
            if let Err(e) =  build_index(set.path().as_ref()) {
                return Err(e);
            }
            let set_to_search = String::from(format!("{};",set.path().iter().last().unwrap().to_string_lossy()));
            if !mods.contains(&set_to_search){
                mods.push(set_to_search);
            }
        }
    };

    //write all mods to lib.tmp
    let lib_tmp = &lib_path.with_extension(".tmp");
    let mut writer = BufWriter::new(OpenOptions::new().create_new(true).write(true).open(lib_tmp)?); 

    for m in mods {
        writeln!(&mut writer,"pub mod {}",m)?;
    };

    fs::rename(&lib_tmp,&lib_path)?;
    Ok(())
}


