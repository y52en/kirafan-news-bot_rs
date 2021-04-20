use kirafan_newsbot_rust::archive::*;
use kirafan_newsbot_rust::module::*;
use kirafan_newsbot_rust::savenews;
use kirafan_newsbot_rust::tweet::*;

use std::sync::mpsc::channel;

// use std::sync::{Arc, Mutex};

// static HOMEPATH: &str = "/home/y52en/kirafan-news_rs";
static HOMEPATH: &str = "/home/pi/kirafan-news_rs";

// #[tokio::main]
// async fn main(){
//     tweet("test").await;
// }

#[tokio::main]
async fn main() {
    ///// init /////////////////////////
    let host = "https://kirara.star-api.com".to_string();
    // let host = "http://127.0.0.1:5500/".to_string();
    // let baseurl = format!("{}{}", &host, "/cat_news/");
    let category = ["information", "maintenance", "update"];

    let sel_pageurls = compiled_selector(".newsPost > a");
    let sel_pagetitles = compiled_selector(".newsPost > a > dl > dd");
    let sel_dates = compiled_selector(".newsPost > a > dl > dt");

    let sel_js = compiled_selector("script[src]");
    let sel_css = compiled_selector("link");
    let sel_pagelist = compiled_selector("a.page-numbers");

    let re_urls = compiled_regex(r"Unity\.call\('([^?]+)\?auser_id=");
    let re_links = compiled_regex(r#""([^?"]+)"#);
    let re_csslinks = compiled_regex(r#"href="([^?"]+)"#);
    let re_pagelist = compiled_regex(r"=(\d+)");
    let re_inner = compiled_regex(r">([^<]+)<");
    let re_filename = compiled_regex(r"/(\d+)/$");

    let (process_list_sender, process_list_receiver) = channel();

    ///// init end //////////////////////

    for (url_i, url) in category.iter().enumerate() {
        let url_clone = url.clone();
        let host_tmp = host.clone();
        let page1_html_feature = tokio::spawn(async move {
            let url_ = format!("{}{}{}", &host_tmp, "/cat_news/", url_clone);
            return get_html_retry(&url_, 3).await;
        });
        let page1_html = page1_html_feature.await.unwrap();

        let html_data = page1_html;
        let mut doc = parse_html(&html_data);

        let pagelist = scraping(&re_pagelist, &sel_pagelist, &doc);
        let last_page = pagelist.last().unwrap().parse::<i32>().unwrap();

        for page_id in 1..(last_page + 1) {
            println!("{}", page_id);
            if page_id != 1 {
                let html = get_html_retry(
                    &(format!("{}{}{}{}{}", &host, "/cat_news/", url, "/?page=", &page_id)),
                    3,
                )
                .await;
                doc = parse_html(&html);
            }

            let pagetitles = scraping(&re_inner, &sel_pagetitles, &doc);
            let _pageurls = scraping(&re_urls, &sel_pageurls, &doc);

            let js_link = scraping(&re_links, &sel_js, &doc);
            let css_link = scraping(&re_csslinks, &sel_css, &doc);
            let dates = scraping(&re_inner, &sel_dates, &doc);

            for js in js_link {
                let host_tmp = host.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &format!("{}{}", &host_tmp, &js),
                        "js",
                        &"",
                        &host_tmp.clone(),
                        "",
                        "",
                    )
                    .await;
                });
                process_list_sender.send(process).unwrap();
            }

            for css in css_link {
                let host_tmp = host.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &format!("{}{}", &host_tmp, &css),
                        "css",
                        &"",
                        &host_tmp,
                        "",
                        "",
                    )
                    .await;
                });
                process_list_sender.send(process).unwrap();
            }

            for (i, link) in _pageurls.iter().enumerate() {
                let pagetitles = pagetitles.clone();
                println!("{}", link);
                // let folder = format!(
                //     "{}_{}",
                //     re_find(&re_filename, link),
                //     pagetitles.get(i).unwrap().replace("/", "／")
                // );
                // let url_tmp = url.to_string();
                let link_static = link.to_string();
                // let folder_static = folder.to_string();

                savenews::savenews(link_static, "".to_string(), "".to_string(), &process_list_sender).await;

                let host_tmp = host.clone();
                let link_tmp = link.clone();
                // let url_tmp = url.clone();
                // let new_date = new_date.clone();
                let dates = dates.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &link_tmp,
                        "html",
                        "",
                        &host_tmp,
                        dates.get(i).unwrap(),
                        pagetitles.get(i).unwrap(),
                    )
                    .await;
                });
                process_list_sender.send(process).unwrap();
            }
        }
    }

    drop(process_list_sender);
    futures::future::join_all(process_list_receiver).await;

    println!("fin");
}
