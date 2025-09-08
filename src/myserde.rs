#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="camelCase")]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="camelCase")]
struct DogOwner {
    #[serde(alias="firstName")]
    first_name: String, 
    last_name: String,
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_deserialize_test() {
        crate::init();
        let json_string = r#"
        {
            "name": "test",
            "year_born": 2021,
            "owner": {
                "first_name": "test01",
                "last_name": "test02"
            },
            "breed": "Great Pyrenees"
        }"#;

        let dog_deser = 
            serde_json::from_str::<Dog>(json_string);

        if dog_deser.is_ok() {
            let dog = dog_deser.ok().unwrap();
            info!("{:?}", dog);
        } else {
            info!("{}", dog_deser.err().unwrap());
        }


    }

    #[test]
    fn it_test01() {
        crate::init();

        info!("test");

        let owner01 = DogOwner{ 
            first_name: "test01".to_string(), 
            last_name:"test02".to_string()
        };
        let dog01 = Dog {
            name: "test".to_string(),
            year_born: 2021,
            owner: owner01,
        };
        let dog_ser = serde_json::to_string_pretty(&dog01);
        if dog_ser.is_ok() {
            info!("{}", dog_ser.ok().unwrap());
        } else {
            info!("{}", dog_ser.err().unwrap());
        }

    }
}