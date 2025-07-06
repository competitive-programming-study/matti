
use automation::crud;
use std::env;
use std::process::exit;

fn print_usage() {
    eprintln!(
        "Usage:\n\
         \tnew <set_name> <exercise_name>\n\
         \tremove <set_name> <exercise_name>\n\
         \tupdate"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    match args[1].as_str() {
        "new" => {
            if args.len() != 4 {
                print_usage();
                exit(1);
            }
            let set = &args[2];
            let exercise = &args[3];
            match crud::new_exercise(set, exercise) {
                Ok(_) => println!("Exercise {exercise} created in set {set}."),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        "remove" => {
            if args.len() != 4 {
                print_usage();
                exit(1);
            }
            let set = &args[2];
            let exercise = &args[3];
            match crud::remove_exercise(set, exercise) {
                Ok(true) => println!("Exercise {exercise} removed. Set {set} was also removed."),
                Ok(false) => println!("Exercise {exercise} removed."),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        "update" => {
            match crud::update_conf() {
                Ok(_) => println!("Configuration updated."),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        _ => {
            print_usage();
            exit(1);
        }
    }
}
