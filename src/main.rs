use kirafan_newsbot_rust::archive::*;
use kirafan_newsbot_rust::module::*;
use kirafan_newsbot_rust::savenews;
use kirafan_newsbot_rust::tweet::*;

use std::sync::mpsc::channel;

// use std::sync::{Arc, Mutex};

static HOMEPATH: &str = "/home/pi/kirafan-news_rs";
// static HOMEPATH: &str = "/home/y52en/kirafan-news_rs";

// #[tokio::main]
// async fn main(){
//     tweet("test").await;
// }

#[tokio::main]
async fn main() {
    ///// init /////////////////////////
    // let host = "https://kirara.star-api.com".to_string();
    let host = "http://127.0.0.1:5500".to_string();
    // let baseurl = format!("{}{}", &host, "/cat_news/");
    let category = ["information", "maintenance", "update"];

    let sel_pageurls = compiled_selector(".newsPost > a");
    let sel_pagetitles = compiled_selector(".newsPost > a > dl > dd");
    let sel_new_pageurls = compiled_selector(".new > a");
    let sel_new_pagetitles = compiled_selector(".new > a > dl > dd");
    let sel_dates = compiled_selector(".new > a > dl > dt");
    let sel_new_dates = compiled_selector(".newsPost > a > dl > dt");

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

            //len???????????????????????????unused???????????????
            let pagetitles = scraping(&re_inner, &sel_pagetitles, &doc);
            let _pageurls = scraping(&re_urls, &sel_pageurls, &doc);

            let new_pagetitles = scraping(&re_inner, &sel_new_pagetitles, &doc);
            let new_pageurls = scraping(&re_urls, &sel_new_pageurls, &doc);

            let js_link = scraping(&re_links, &sel_js, &doc);
            let css_link = scraping(&re_csslinks, &sel_css, &doc);

            let new_date = scraping(&re_inner, &sel_new_dates, &doc);
            let dates = scraping(&re_inner, &sel_dates, &doc);

            if new_pageurls.len() != 0 {
                let mut tmp_title = vec![];
                let mut tmp_url = vec![];
                let mut tmp_date = vec![];

                let tweeted_ls_path =
                    format!("{}{}{}{}{}", HOMEPATH, "/", "tweeted_", url.clone(), ".txt");
                let mut tweeted_ls = readfile_asline(&tweeted_ls_path).unwrap();

               

                for (i, title) in new_pagetitles.iter().enumerate() {
                     println!("{:#?}{}",&tweeted_ls,
                        &format!("{}{}", new_date.get(i).unwrap(), &title));

                    if !is_vec_contein(
                        &tweeted_ls,
                        &format!("{}{}", new_date.get(i).unwrap(), &title),
                    ) {
                        tmp_title.push(title);
                        tmp_date.push(new_date.get(i).unwrap());
                        tmp_url.push(new_pageurls.get(i).unwrap());
                    }
                }

                for (i, title) in tmp_title.iter().enumerate() {
                    tweeted_ls.push(format!("{}{}", tmp_date.get(i).unwrap(), title));
                }
                writefile_asline(&tweeted_ls_path, tweeted_ls).unwrap();

                if tmp_title.len() != 0 {
                    let mut tweet_str = "?????????".to_string();
                    if url_i == 1 {
                        tweet_str += "?????????????????????";
                    } else if url_i == 2 {
                        tweet_str += "?????????????????????";
                    }
                    tweet_str += "???????????????????????????\n";

                    while tmp_title.len() != 0 {
                        let mut tmp_str = tweet_str.to_string();

                        tmp_str += tmp_title.first().unwrap();
                        tmp_str += "\n";
                        tmp_str += tmp_url.first().unwrap();
                        tmp_str += "\n";

                        if count_twitter_str(&tmp_str) > 280 {
                            println!("{}", tweet_str);
                            tweet(&tweet_str).await;
                            tweet_str = "??????\n".to_string();
                        } else {
                            tweet_str = tmp_str;
                            tmp_title.drain(..1);
                            tmp_url.drain(..1);
                        }
                    }
                    println!("{}", tweet_str);
                    tweet(&tweet_str).await;
                }
            }

            for js in js_link {
                let host_tmp = host.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &format!("{}{}", &host_tmp, &js),
                        "js",
                        &"",
                        &host_tmp.clone(),
                        "",
                        ""
                    )
                    .await;
                });
                process_list_sender.send(process).unwrap();
            }

            for css in css_link {
                let host_tmp = host.clone();
                let process = tokio::spawn(async move {
                    archive_file(&format!("{}{}", &host_tmp, &css), "css", &"", &host_tmp,"","")
                        .await;
                });
                process_list_sender.send(process).unwrap();
            }

            for (i, link) in new_pageurls.iter().enumerate() {
                let new_pagetitles = new_pagetitles.clone();
                // for (i, link) in pageurls.iter().enumerate() {
                println!("{}", link);
                // let folder = format!(
                //     "{}_{}",
                //     re_find(&re_filename, link),
                //     new_pagetitles.get(i).unwrap().replace("/", "???")
                // );
                // let folder = format!(
                //     "{}_{}",
                //     re_find(&re_filename, link),
                //     pagetitles.get(i).unwrap().replace("/", "???")
                // );
                // mkdirs(&format!(
                //     "{}{}{}{}{}{}",
                //     HOMEPATH, "/news/", &url, "/", &folder, "/"
                // ))
                // .await;
                // let url_tmp = url.to_string();
                let link_static = link.to_string();
                // let folder_static = folder.to_string();

                savenews::savenews(link_static, "".to_string(), "".to_string(), &process_list_sender).await;

                let host_tmp = host.clone();
                let link_tmp = link.clone();
                // let url_tmp = url.clone();
                let new_date = new_date.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &link_tmp,
                        "html",
                        "",
                        &host_tmp,
                        new_date.get(i).unwrap(),
                        new_pagetitles.get(i).unwrap()
                    )
                    .await;
                });
                process_list_sender.send(process).unwrap();
            }

            if pagetitles.len() != new_pagetitles.len() {
                break;
            }
        }
    }

    drop(process_list_sender);
    futures::future::join_all(process_list_receiver).await;

    println!("fin");
}
