#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]


#[cfg(test)]
mod tests {
    use log::info;
    use reqwest::{blocking, redirect::Policy};

    use super::*;

    #[test]
    fn it_redirects_test() {
        crate::init();

        let redir_policy = Policy::limited(5);
        let http_client = blocking::ClientBuilder::new()
            .redirect(redir_policy)
            .build()
            .ok().unwrap();

        let http_result = http_client.get("http://localhost:3000/weather")
            .send();
        if http_result.is_err() {
            info!("{:?}", http_result.err());
        }
        // info!("Weather app result: {:?}", http_result);

    }

    #[test]
    fn it_post_send_test() {
        crate::init();
        let http_client = blocking::Client::new();

        let json_body = r#"
        {
            first_name: "Trevor"
        }
        "#;
        let post_result = http_client
            .post("http://localhost:3000/send_data")
            .header("User-Agent", "Trevor's Rust Application on Linux")
            .body(json_body)
            .send();
        info!("{:?}", post_result.ok().unwrap().text().unwrap());

    }

    #[test]
    fn it_client_req_test01() {
        crate::init();
        let http_clinet = reqwest::blocking::Client::new();
        let http_result = http_clinet.get("https://trevorsullivan.net").send();

        if http_result.is_ok() {
            info!("Body: {:#?}", http_result.ok()
                .unwrap()
                .text()
                .unwrap());
        } 
        else if http_result.is_err() {
            info!("Error occured: {:?}", http_result.err());
        }

        

    }

    #[test]
    fn it_req_test01() {
        crate::init();

        let body = reqwest::blocking::get("https://www.rust-lang.org")
            .unwrap()
            .text().unwrap();

        info!("body = {body:?}");
    }
}