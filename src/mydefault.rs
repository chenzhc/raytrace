#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::default;


#[derive(Debug)]
struct FirstName(String);

impl Default for FirstName {
    fn default() -> Self {
        return FirstName("test".to_string());
    }
}


#[derive(Debug)]
struct Person {
    first_name: FirstName,
    last_name: String,
    age: u8,
    location: String,
}

impl Default for Person {
    fn default() -> Self {
        return Person {
            first_name: FirstName::default(), 
            last_name: "Sullivan".to_string(), 
            age: 26, 
            location: "United States".to_string(), 
        };
    }
}


fn new_person() -> Person {
    return Person { 
        first_name: FirstName::default(), 
        last_name: "Sullivan".to_string(), 
        age: 26, 
        location: "United States".to_string(), 
    };
}



#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_first_name_test() {
        crate::init();

        let f1 = FirstName::default();
        info!("{:?}", f1);

    }

    #[test]
    fn it_redirects_test() {
        crate::init();
        info!("Test");

        let p1 = new_person();
        info!("{:?}", p1);

        let p2 = Person {
            first_name: FirstName("Shannon".to_string()),
            ..Default::default()
        };
        info!("Default person is: {:?}", p2);

    }
}