/*
Nginx Proxy

- Provides controlled access to server
- Rate limiting
- Request caching
*/

mod server {
    // mod.rs
    pub use nginx::NginxServer;

    pub trait Server {
        fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
    }

    mod application {
        use super::Server;

        pub struct Application;

        impl Server for Application {
            fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
                if url == "/app/status" && method == "GET" {
                    return (200, "Ok".into());
                }

                if url == "/create/user" && method == "POST" {
                    return (201, "User Created".into());
                }

                (404, "Not Ok".into())
            }
        }
    }

    mod nginx {
        use std::collections::HashMap;

        use super::{application::Application, Server};

        /// NGINX server is a proxy to an application server.
        pub struct NginxServer {
            application: Application,
            max_allowed_requests: u32,
            rate_limiter: HashMap<String, u32>,
        }

        impl NginxServer {
            pub fn new() -> Self {
                Self {
                    application: Application,
                    max_allowed_requests: 2,
                    rate_limiter: HashMap::default(),
                }
            }

            pub fn check_rate_limiting(&mut self, url: &str) -> bool {
                let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);

                if *rate > self.max_allowed_requests {
                    return false;
                }

                *rate += 1;
                true
            }
        }

        impl Server for NginxServer {
            fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
                if !self.check_rate_limiting(url) {
                    return (403, "Not Allowed".into());
                }

                self.application.handle_request(url, method)
            }
        }
    }
}

fn main() {
    use crate::server::{NginxServer, Server};

    let app_status = &"/app/status".to_string();
    let create_user = &"/create/user".to_string();

    let mut nginx = NginxServer::new();

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(create_user, "POST");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

    let (code, body) = nginx.handle_request(create_user, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
}
