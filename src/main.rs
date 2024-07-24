use std::collections::HashMap;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = "data";
    
    let mut store: HashMap<String, String> = match load_store(filename) {
        Ok(map) => map,
        Err(..) => {
            HashMap::new()
        }
    };

    if args.len() >= 2 {
        match args[1].as_str() {
            "get" => {
                if args.len() != 3 {
                    println!("Invalid command format. Run pat get <PAT_NAME>.");
                } else {
                    if store.contains_key(&args[2]) {
                        let _ = clipboard_anywhere::set_clipboard(store.get(&args[2]).unwrap());
                        if let Err(_e) = save_store(&store, filename) {
                            println!("Failed to setup PAT \"{}\". Please try again :P", &args[2]);
                        } else {
                            println!("Copied PAT to clipboard.");
                        }

                    } else {
                        println!("Key \"{}\" doesn't exist.", &args[2]);
                    }
                }
            },
            "set" => {
                if args.len() != 4 {
                    println!("Invalid command format. Run pat set <PAT_NAME> <PAT_VALUE>.");
                } else {
                    if store.contains_key(&args[2]) {
                        println!("PAT \"{}\" already exists. Run pat renew to change its value.", &args[2]);
                    } else {
                        store.insert(args[2].clone(), args[3].clone());
                        if let Err(_e) = save_store(&store, filename) {
                            println!("Failed to setup PAT \"{}\". Please try again :P", &args[2]);
                        } else {
                            println!("PAT \"{}\" setup with success.", &args[2]);
                        }
                    }
                }
            },
            "delete" => {
                if args.len() != 3 {
                    println!("Invalid command format. Run pat delete <PAT_NAME>.");
                } else {
                    if store.contains_key(&args[2]) {
                        store.remove(&args[2]);
                        if let Err(_e) = save_store(&store, filename) {
                            println!("Failed to delete PAT \"{}\". Please try again :P", &args[2]);
                        } else {
                            println!("PAT \"{}\" deleted with success.", &args[2]);
                        }
                    } else {
                        println!("Key \"{}\" doesn't exist.", &args[2]);
                    }
                }
            },
            "renew" => {
                if args.len() != 4 {
                    println!("Invalid command format. Run pat renew <PAT_NAME> <PAT_VALUE>.");
                } else {
                    if !store.contains_key(&args[2]) {
                        println!("Key \"{}\" doesn't exist.", &args[2]);
                    } else {
                        *store.get_mut(&args[2]).unwrap() = args[3].clone();
                        if let Err(_e) = save_store(&store, filename) {
                            println!("Failed to renew PAT \"{}\" Please try again :P", &args[2]);
                        } else {
                            println!("PAT \"{}\" renewed with success.", &args[2]);
                        }
                    }
                }
            },
            "help" => {
                println!("Run pat get <PAT_NAME> to get the value of the specified PAT.");
                println!("Run pat set <PAT_NAME> <PAT_VALUE> to setup a new PAT.");
                println!("Run pat delete <PAT_NAME> to delete the specified PAT.");
                println!("Run pat renew <PAT_NAME> <PAT_VALUE> to update the value of the specified PAT.");
            },
            _ => {
                println!("Run pat help to see a list of all commands.");
            }
        }
    } else {
        println!("Run pat help to see a list of all commands.");
    }
}

fn save_store(store: &HashMap<String, String> , filename: &str) -> Result<(), ()> {
    let file = File::create(filename).unwrap();
    let _ = serde_json::to_writer(file, store);
    Ok(())
}

fn load_store(filename: &str) -> Result<HashMap<String, String>, ()> {
    let file = match File::open(filename) {
        Ok(file) => Some(file),
        Err(..) => None
    };
    match file {
        Some(file) => {
            let store: HashMap<String, String> = serde_json::from_reader(file).unwrap();
            Ok(store)
        },
        None => {
            Err(())
        }
    }
}
