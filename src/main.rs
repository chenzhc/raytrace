#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Arg, ArgGroup, Command};

fn main() {
    raytrace::init();

    let match_result = command!()
        .about("This application registers people with their doctor's office.")
        .subcommand(
    Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname","firstname"])
                        .required(true)
                        .help("The person's first name")
                        // .conflicts_with("lastname")
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("This argument takes the person's last name")
                )
        )
        .subcommand(
    Command::new("register-pet")
                .arg(
                    Arg::new("petname")
                        .long("pet-name")
                        .short('n')
                        .required(true)
                )
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy coat or not")
        )
        .get_matches();

        if let Some(matchs) = match_result.subcommand_matches("register-pet") {
            // println!("Fluffy: {}", match_result.get_one::<String>("fluffy").unwrap());
            let pet_name = matchs
                .get_one::<String>("petname") ;
            println!("Pet name: {}", pet_name.unwrap_or(&"test".to_string()));
        }
    

}
