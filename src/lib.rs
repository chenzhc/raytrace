#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;

pub mod myserde;

pub mod myreqwest;

pub mod traits_test;

pub mod mydefault;

pub mod mythread;


// init log config
pub fn init() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("INFO");
    // let _ = env_logger::builder()
    //     .target(env_logger::Target::Stdout)
    //     .filter_level(log::LevelFilter::Trace)
    //     .is_test(true)
    //     .try_init();
}
