#![deny(warnings)]
#![allow(unused_imports)]

use maud::{html, PreEscaped, DOCTYPE};
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello_world = warp::path::end()
        .map(|| hello_markup())
        .map(|body| warp::reply::html(body));
    let routes = warp::get().and(hello_world.or(warp::fs::dir("app")));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn hello_markup() -> String {
    String::from(html! {
        (DOCTYPE)
        html {
            head {
                title { "Webcrust" }
                link rel="stylesheet" type="text/css" href="global.css";
            }
            body {
                main-heading {
                    template shadowrootmode="open" {
                        h1 {
                            slot {}
                        }
                        link rel="stylesheet" type="text/css" href="main-heading.css";
                    }
                    "Webcrust"
                }
                duster-counter {
                    template shadowrootmode="open" {
                        div {
                            slot {}
                        }
                        button disabled="true" {
                            "MAKE COUNT GO UP"
                        }
                        link rel="stylesheet" type="text/css" href="counter.css";
                    }
                    "0"
                }
                script src="/polyfill.js" {}
                script src="/counter.js" {}
                p {
                    "A bunch of copy that that is just regular old HTML:"
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
                
            }
        }
    })
}
