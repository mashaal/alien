#![deny(warnings)]
#![allow(unused_imports)]

use maud::{html, PreEscaped, DOCTYPE};
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

#[tokio::main]
async fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    let headers = warp::reply::with::headers(headers);

    let home = warp::path::end()
        .map(|| homepage())
        .map(|body| warp::reply::html(body))
        .with(&headers);

    let routes = warp::get().and(home.or(warp::fs::dir("public")));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn homepage() -> String {
    String::from(html! {
        (DOCTYPE)
        html {
            head {
                title { "_" }
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" type="text/css" href="global.css";
            }
            body {
                video-waste {
                    template shadowrootmode="open" {
                        video autoplay="true" loop="true" muted="true" playsinline="true" {
                            source src="/video.mp4" type="video/mp4";
                        }
                        div {
                            .left {
                                canvas class="one" {}
                            }
                            .right {
                                canvas class="two" {}
                            }
                        }
                        script src="/video-waste.js" {}
                        link rel="stylesheet" type="text/css" href="video-waste.css";
                    }
                }
            }
        }
    })
}
