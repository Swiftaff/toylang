use crate::{Compiler, ErrorStackJson};
use std::collections::HashMap;
use warp::{http::Response, Filter};

/// Only function for server
#[tokio::main]
pub async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let example1 = warp::get()
        .and(warp::path("parse"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("filepath") {
            Some(filepath) => {
                //let result = compiler_runner::main(input, debug, output, nosave, tokens, code);
                let mut compiler =
                    Compiler::new(filepath.clone(), false, None, true, true).unwrap();
                if let Err(e) = compiler.run(true, false) {
                    println!("Application error: {}", e);
                }
                let e = ErrorStackJson {
                    errors: compiler.error_stack,
                };
                let j = serde_json::to_string(&e).unwrap();
                Response::builder().body(j)
            }
            None => Response::builder().body(String::from("No \"filepath\" param in query.")),
        });
    let example2 = warp::get().and(warp::path("test")).map(|| "Server working");
    let routes = warp::get().and((example1).or(example2));
    warp::serve(routes).run(([127, 0, 0, 1], 12345)).await;
}
