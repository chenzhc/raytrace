#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;


pub async fn say_hello() {
    info!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[tokio::test]
    async fn it_say_hello_test() {
        crate::init();

        say_hello().await;
        
    }

}