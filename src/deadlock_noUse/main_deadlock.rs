use kirafan_newsbot_rust::archive::*;
use kirafan_newsbot_rust::module::*;
use kirafan_newsbot_rust::savenews;

use std::sync::mpsc::channel;

use std::sync::{Arc, Mutex};

static HOMEPATH: &str = "/home/y52en/kirafan-news_rs";

#[tokio::main]
async fn main_() {
    let process_list = Arc::new(Mutex::new(vec![]));
    let process_list_cloned = process_list.clone();

    for i in 0..2 {
        let process = tokio::spawn(async {
            let process_list = Arc::new(Mutex::new(vec![]));
            let process_list_cloned = process_list.clone();
            let process = tokio::spawn(async {});
            let mut process_list_locked2 = process_list_cloned.try_lock().unwrap();
            process_list_locked2.push(process);
        });
        let mut process_list_locked = process_list_cloned.try_lock().unwrap();
        process_list_locked.push(process);

        let mut process_list_locked = process_list_cloned.try_lock().unwrap();
        futures::future::join_all(process_list_locked.drain(..)).await;
    }
    let mut process_list_locked = process_list_cloned.try_lock().unwrap();
    futures::future::join_all(process_list_locked.drain(..)).await;
}

#[tokio::main]
async fn main() {
    ///// init /////////////////////////
    let host = "https://kirara.star-api.com";
    // let host = "http://127.0.0.1:5500/".to_string();
    // let baseurl = format!("{}{}", &host, "/cat_news/");
    let category = ["information", "maintenance", "update"];

    let sel_pageurls = compiled_selector(".newsPost > a");
    let sel_pagetitles = compiled_selector(".newsPost > a > dl > dd");
    let sel_new_pageurls = compiled_selector(".new > a");
    let sel_new_pagetitles = compiled_selector(".new > a > dl > dd");

    let sel_js = compiled_selector("script[src]");
    let sel_css = compiled_selector("link");
    let sel_pagelist = compiled_selector("a.page-numbers");

    let re_urls = compiled_regex(r"Unity\.call\('([^?]+)\?auser_id=");
    let re_links = compiled_regex(r#""([^?"]+)"#);
    let re_csslinks = compiled_regex(r#"href="([^?"]+)"#);
    let re_pagelist = compiled_regex(r"=(\d+)");
    let re_inner = compiled_regex(r">([^<]+)<");
    let re_filename = compiled_regex(r"/(\d+)/$");

    let process_list = Arc::new(Mutex::new(vec![]));
    let process_list_cloned = process_list.clone();

    let process_list0 = Arc::new(Mutex::new(vec![]));
    let process_list0_cloned = process_list0.clone();
    ///// init end //////////////////////

    for (url_i, url) in category.iter().enumerate() {
        // let process_list2 = Arc::new(Mutex::new(vec![]));
        // let process_list2_cloned = process_list2.clone();
        let (process_list_sender, process_list_receiver) = channel();

        
        let url_clone = url.clone();
        let url = url.clone();
        let sel_pageurls = sel_pageurls.clone();
        let sel_pagetitles = sel_pagetitles.clone();
        let sel_new_pageurls = sel_new_pageurls.clone();
        let sel_new_pagetitles = sel_new_pagetitles.clone();
        let sel_js = sel_js.clone();
        let sel_css = sel_css.clone();
        let sel_pagelist = sel_pagelist.clone();
        let re_urls = re_urls.clone();
        let re_links = re_links.clone();
        let re_csslinks = re_csslinks.clone();
        let re_pagelist = re_pagelist.clone();
        let re_inner = re_inner.clone();
        let re_filename = re_filename.clone();
        let process_list_cloned = process_list.clone();

        let process = tokio::spawn(async move {
            let page1_html_feature = tokio::spawn(async move {
                let url_ = format!("{}{}{}", &host, "/cat_news/", url_clone);
                return get_html_retry(&url_, 3).await;
            });
            let page1_html = page1_html_feature.await.unwrap();

            let html_data = page1_html;

            let pagelist = scraping(&re_pagelist, &sel_pagelist, &parse_html(&html_data));
            let last_page = pagelist.last().unwrap().parse::<i32>().unwrap();

            for page_id in 1..(last_page + 1) {
                let pagetitles;
                let pageurls;

                let new_pagetitles;
                let new_pageurls;

                let js_link;
                let css_link;
                println!("{}", page_id);                    
                {
                    let html = get_html_retry(
                        &(format!("{}{}{}{}{}", &host, "/cat_news/", url, "/?page=", &page_id)),
                        3,
                    )
                    .await;
                    let doc = parse_html(&html);

                    //lenを取得しているためunusedにならない
                    pagetitles = scraping(&re_inner, &sel_pagetitles, &doc);
                    pageurls = scraping(&re_urls, &sel_pageurls, &doc);

                    new_pagetitles = scraping(&re_inner, &sel_new_pagetitles, &doc);
                    new_pageurls = scraping(&re_urls, &sel_new_pageurls, &doc);

                    js_link = scraping(&re_links, &sel_js, &doc);
                    css_link = scraping(&re_csslinks, &sel_css, &doc);
                }

                for js in js_link {
                    let process = tokio::spawn(async move {
                        archive_file(
                            &format!("{}{}", &host, &js),
                            "js",
                            &"",
                            &host.clone(),
                        )
                        .await;
                    });
                    let mut process_list2_locked = process_list2_cloned.try_lock().unwrap();
                    process_list2_locked.push(process);
                }

                for css in css_link {
                    let process = tokio::spawn(async move {
                        archive_file(&format!("{}{}", &host, &css), "css", &"", &host)
                            .await;
                    });
                    let mut process_list2_locked = process_list2_cloned.try_lock().unwrap();
                    process_list2_locked.push(process);
                }

                for (i, link) in pageurls.iter().enumerate() {
                    println!("{}", link);
                    let folder = format!(
                        "{}_{}",
                        re_find(&re_filename, link),
                        pagetitles.get(i).unwrap().replace("/", "／")
                    );
                    mkdirs(&format!(
                        "{}{}{}{}{}{}",
                        HOMEPATH, "/news/", &url, "/", &folder, "/"
                    ))
                    .await;
                    let folder_clone = folder.clone();
                    savenews_copy::savenews(link.to_string(), url.to_string(), folder_clone, &process_list2_cloned).await;

                    let link_tmp = link.clone();
                    let process = tokio::spawn(async move {
                        archive_file(
                            &link_tmp,
                            "html",
                            &format!("{}/{}", url, &folder),
                            &host,
                        )
                        .await;
                    });
                    let mut process_list2_locked = process_list2_cloned.try_lock().unwrap();
                    process_list2_locked.push(process);
                }

                if pagetitles.len() != new_pagetitles.len() {
                    break;
                }
            }

            let mut process_list2_locked = process_list2_cloned.try_lock().unwrap();
            let mut process_list_locked = process_list_cloned.try_lock().unwrap();
            process_list_locked.push(futures::future::join_all(process_list2_locked.drain(..)));
        });
        let mut process_list0_locked = process_list0_cloned.try_lock().unwrap();
        process_list0_locked.push(process);
    }

    let mut process_list_locked = process_list_cloned.try_lock().unwrap();
    futures::future::join_all(process_list_locked.drain(..)).await;

    let mut process_list0_locked = process_list0_cloned.try_lock().unwrap();
    futures::future::join_all(process_list0_locked.drain(..)).await;

    println!("fin");
}
