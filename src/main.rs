use kirafan_newsbot_py_rust::archive::*;
use kirafan_newsbot_py_rust::module::*;
use kirafan_newsbot_py_rust::savenews;

// use std::sync::{Arc, Mutex, RwLock};

// use regex::Regex;

static HOMEPATH: &str = "/home/y52en/kirafan-news_rs";

// #[macro_use]
// extern crate lazy_static;

// #[tokio::main]
// async fn main() {
//     let a = r"「まんがタイムきららMAX200号記念キャンペーン」
// https://kirara.comプレゼントキャンペーン
// https://kir";
// let config = twitter_text_config::default();
//     println!(
//         "{}",
//         parse(&a, config ,true).weighted_length
//     );
// }

#[tokio::main]
async fn main__() {
    let mut process_list = vec![];

    let process_a = tokio::spawn(async {
        //処理
    });
    process_list.push(process_a);

    //色々処理

    let process_b = tokio::spawn(async {
        //処理
    });
    process_list.push(process_b);

    //色々処理

    for process in process_list {
        process.await.unwrap();
    }
}

#[tokio::main]
async fn main_() {
    // let process_list:Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>  = Arc::new(Mutex::new(vec![]));
    // let process_list = Arc::new(RwLock::new(vec![]));

    // let mut process_a = tokio::spawn(async {
    //     //処理
    // });
    // // let mut process_list_locked = process_list_cloned.lock().unwrap();
    // {
    //     let process_list_ = Arc::clone(&process_list);
    //     let process_list_2 = process_list_.write();
    //     if let Ok(mut v) = process_list_2 {
    //         v.push(process_a);
    //     }
    // };
    // process_list_.push(Mutex::new(process_a));

    //色々処理
    // println!("1");

    // let process_b = tokio::spawn(async {
    //     //処理
    // });
    // let mut process_list_locked = process_list_cloned1.lock().unwrap();
    // process_list_locked.push(process_b);

    // println!("2");

    //色々処理

    // let process_list_locked = process_list_cloned.lock().unwrap();
    // for process in process_list_locked.into_iter() {
    //     process.await.unwrap();
    // }
    // let mut process_list_2 = q.write().unwrap();
    // // if v = process_list_2 {
    // for mut v_ in process_list_2.iter() {
    //     // (v_).await;
    // }mut q = Arc::clone(&process_list);
    // let 
    // }
    //   || unsafe { async {
    //      (&(*process_list_locked.get(0).unwrap())).await
    // } };
    // println!("{:#?}", );
}

#[tokio::main]
async fn main() {
    ///// init /////////////////////////
    let host = "https://kirara.star-api.com/".to_string();
    // let host = "http://127.0.0.1:5500/".to_string();
    let baseurl = format!("{}{}", &host, "/cat_news/");
    // let category = ["information", "maintenance", "update"];
    let category = ["information"];

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
    // r"/([^/]+)$"

    //for cron
    // let rootpath = "/home/y52en/";
    // let rootpath = "/home/pi/";

    // let mut process_list:Vec<Mutex<Vec<tokio::task::JoinHandle<()>>>> = vec![];
    let mut process_list = vec![];
    // let process_list = Arc::new(Mutex::new(vec![]));
    // let process_list_shared = process_list.clone();
    // tokio::task::JoinHandle

    ///// init end //////////////////////

    for (url_i, url) in category.iter().enumerate() {
        let url_clone = url.clone();
        let host_tmp = host.clone();
        let page1_html_feature = tokio::spawn(async move {
            let url_ = format!("{}{}{}", &host_tmp, "cat_news/", url_clone);
            return get_html_retry(&url_, 3).await;
        });
        let page1_html = page1_html_feature.await.unwrap();

        // println!("{:#?}", page1_html);

        let html_data = page1_html;
        let mut doc = parse_html(&html_data);

        let pagelist = scraping(&re_pagelist, &sel_pagelist, &doc);
        let last_page = pagelist.last().unwrap().parse::<i32>().unwrap();

        // for page_id in 1..(last_page + 1) {
        for page_id in 1..5 {
            println!("{}", page_id);
            if page_id != 1 {
                let html = get_html_retry(
                    &(format!("{}{}{}{}{}", &host, "cat_news/", url, "/?page=", &page_id)),
                    3,
                )
                .await;
                doc = parse_html(&html);
            }

            // for debug
            // if page_id > 3 {
            //     break;
            // }

            //lenを取得しているためunusedにならない
            let pagetitles = scraping(&re_inner, &sel_pagetitles, &doc);
            let pageurls = scraping(&re_urls, &sel_pageurls, &doc);

            //後でdebug
            let new_pagetitles = scraping(&re_inner, &sel_new_pagetitles, &doc);
            let new_pageurls = scraping(&re_urls, &sel_new_pageurls, &doc);

            // println!("{:?}", new_pagetitles);

            let js_link = scraping(&re_links, &sel_js, &doc);
            let css_link = scraping(&re_csslinks, &sel_css, &doc);

            if new_pageurls.len() != 0 {
                let mut tmp_title = vec![];
                let mut tmp_url = vec![];

                let tweeted_ls_path =
                    format!("{}{}{}{}{}", HOMEPATH, "/", "tweeted_", url.clone(), ".txt");
                let mut tweeted_ls = readfile_asline(&tweeted_ls_path).unwrap();

                for (i, title) in new_pagetitles.iter().enumerate() {
                    if !is_vec_contein(&tweeted_ls, &title) {
                        tmp_title.push(title);
                        tmp_url.push(new_pageurls.get(i).unwrap());
                    }
                }

                for title in &tmp_title {
                    tweeted_ls.push(title.to_string());
                }
                writefile_asline(&tweeted_ls_path, tweeted_ls).unwrap();

                if tmp_title.len() != 0 {
                    let mut tweet_str = "新しい".to_string();
                    if url_i == 1 {
                        tweet_str += "メンテナンスの";
                    } else if url_i == 2 {
                        tweet_str += "アップデートの";
                    }
                    tweet_str += "お知らせがあります\n";

                    while tmp_title.len() != 0 {
                        break;
                        let mut tmp_str = tweet_str.to_string();

                        tmp_str += tmp_title.first().unwrap();
                        tmp_str += "\n";
                        tmp_str += tmp_url.first().unwrap();
                        tmp_str += "\n";

                        // 後でtwitter.textに変える
                        // println!("{}",character_count(&tmp_str, 23,23));
                        if count_twitter_str(&tmp_str) > 280 {
                            println!("{}", tweet_str);
                            // tweet(tweet_str);
                            tweet_str = "続き\n".to_string();
                        } else {
                            tweet_str = tmp_str;
                            tmp_title.drain(..1);
                            tmp_url.drain(..1);
                        }
                    }
                    println!("{}", tweet_str);
                    // tweet(tweet_str);
                }
            }

            //後で即awaitをやめる
            for js in js_link {
                let host_tmp = host.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &format!("{}{}", &host_tmp, &js),
                        "js",
                        &"",
                        &host_tmp.clone(),
                    )
                    .await;
                });

                // let mut process_list_locked = process_list_shared.lock().unwrap();
                // process_list_locked.push(process);
                process_list.push(process);
            }

            for css in css_link {
                let host_tmp = host.clone();
                let process = tokio::spawn(async move {
                    archive_file(&format!("{}{}", &host_tmp, &css), "css", &"", &host_tmp).await;
                });
                process_list.push(process);

                // let mut process_list_locked = process_list_shared.lock().unwrap();
                // process_list_locked.push(process);
            }

            // let pageurls_clone = pageurls.clone();

            // for (i, link) in new_pageurls.iter().enumerate() {
            for (i, link) in pageurls.iter().enumerate() {
                println!("{}", link);
                // let folder = format!("{}{}",re_find(&re_filename, link)
                // , new_pagetitles.get(i).unwrap().replace("/", "／"));
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
                let url_tmp = url.to_string();
                let link_static = link.to_string();
                let folder_static = folder.to_string();
                process_list =
                    savenews::savenews(link_static, url_tmp, folder_static, process_list).await;

                let host_tmp = host.clone();
                let link_tmp = link.clone();
                let url_tmp = url.clone();
                let process = tokio::spawn(async move {
                    archive_file(
                        &link_tmp,
                        "html",
                        &format!("{}/{}", url_tmp, &folder),
                        &host_tmp,
                    )
                    .await;
                });
                // let mut process_list_locked = process_list_shared.lock().unwrap();
                // process_list_locked.push(process);
                process_list.push(process);
            }

            // if pagetitles.len() != new_pagetitles.len() {
            //     break;
            // }
        }
    }

    // let page1_html_feature = tokio::spawn(async move {
    // let process_list_locked = process_list_shared.lock().unwrap();

    for process in process_list.into_iter() {
        process.await.unwrap();
    }
    // page1_html_feature.await.unwrap();

    println!("fin");
}
