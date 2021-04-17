// use std::cell::RefCell;
// use std::rc::Rc;

// use html5ever::driver::ParseOpts;
// // use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
// use html5ever::serialize;
// use html5ever::serialize::SerializeOpts;
// use html5ever::tendril::{StrTendril, TendrilSink};
// use html5ever::{local_name, namespace_url, ns};
// use html5ever::{parse_document, parse_fragment};
// use html5ever::{Attribute, LocalName, QualName};

// use reqwest::Client;

// use std::future::Future;

// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};


// #[tokio::main]
use tokio::task;
#[tokio::main]
async fn main() {
    let host = String::from("https://kirara.star-api.com");
    let baseurl = String::from("https://kirara.star-api.com/cat_news/");

    let urls = ["information/", "maintenance/", "update/"];

    println!("{}", urls[1]);

    for (url_i, url) in urls.iter().enumerate() {
        let page1HTML = "";
    }

    // test();

    println!("fin");
}

// async fn test() -> Result<String,()> {
//     let res = reqwest::get("https://www.google.co.jp/")
//         // .unwrap()
//         .await?
//         .text();
//     println!("{:?}", res);
//     return res;
// }

fn getStr(regex: String, cssSelector: String, html: String) {}
