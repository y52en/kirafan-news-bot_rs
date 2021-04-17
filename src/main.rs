use kirafan_newsbot_py_rust::Archive;
use kirafan_newsbot_py_rust::saveNews;
use kirafan_newsbot_py_rust::Mod::*;

// #[tokio::main]
// async fn main() {
//     let c = compiled_regex(r"(gyk)");
//     let a = re_find(&c, "dtyfugi");
//     println!("{}", a)
// }

#[tokio::main]
async fn main() {
    ///// init /////////////////////////
    let host = "https://kirara.star-api.com".to_string();
    let baseurl = format!("{}{}", &host, "/cat_news/");
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
    // r"/([^/]+)$"

    //for cron
    let rootpath = "/home/y52en/";
    ///// init end //////////////////////

    for (url_i, url) in category.iter().enumerate() {
        let url_clone = url.clone();
        let page1_html_feature = tokio::spawn(async move {
            let url_ = format!("{}{}", "https://kirara.star-api.com/cat_news/", url_clone);
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
                let html =
                    get_html_retry(&(format!("{}{}{}{}", baseurl, url, "/?page=", &page_id)), 3)
                        .await;
                doc = parse_html(&html);
            }

            // for debug
            if page_id > 3 {
                break;
            }

            let pagetitles = scraping(&re_inner, &sel_pagetitles, &doc);
            let pageurls = scraping(&re_urls, &sel_pageurls, &doc);

            //後でdebug
            let new_pagetitles = scraping(&re_inner, &sel_new_pagetitles, &doc);
            let new_pageurls = scraping(&re_urls, &sel_new_pageurls, &doc);

            let js_link = scraping(&re_links, &sel_js, &doc);
            let css_link = scraping(&re_csslinks, &sel_css, &doc);

            if new_pageurls.len() != 0 {
                let mut tmp_title = vec![String::from(""); 0];
                let mut tmp_url = vec![String::from(""); 0];

                let f = "tmp";
                let f_read = "tmp2";

                let tweeted_ls = ["tmp3"];

                for (i, title) in new_pagetitles.iter().enumerate() {}

                for title in &tmp_title {}

                if tmp_title.len() != 0 {
                    let mut tweet_str = "新しい".to_string();
                    if url_i == 1 {
                        tweet_str += "メンテナンスの";
                    } else if url_i == 2 {
                        tweet_str += "アップデートの";
                    }
                    tweet_str += "お知らせがあります\n";

                    while tmp_title.len() != 0 {
                        let mut tmp_str = tweet_str.to_string();

                        tmp_str += tmp_title.first().unwrap();
                        tmp_str += "\n";
                        tmp_str += tmp_url.first().unwrap();
                        tmp_str += "\n";

                        // 後でtwitter.textに変える
                        if tmp_str.len() > 280 {
                            println!("{}", tweet_str);
                        // tweet(tweet_str);
                        } else {
                            tweet_str = tmp_str;
                            tmp_title.pop();
                            tmp_url.pop();
                        }
                    }
                    println!("{}", tweet_str);
                    // tweet(tweet_str);
                }
            }

            //後で即awaitをやめる
            for js in js_link {
                Archive::archive_file(
                    &format!("{}{}", &host, &js),
                    "js",
                    &"".to_string(),
                    &host.clone().to_string(),
                )
                .await;
            }

            for css in css_link {
                Archive::archive_file(
                    &format!("{}{}", &host, &css),
                    "css",
                    &"".to_string(),
                    &host.clone(),
                )
                .await;
            }

            for (i, link) in new_pageurls.iter().enumerate() {
                println!("{}", link);
                let folder = re_find(&re_filename, link);
                mkdirs(format!(
                    "{}{}{}{}{}",
                    rootpath.to_string(),
                    "/kirafan-news_rs/news/",
                    url,
                    "/",
                    &folder
                ));
                Archive::archive_file(&link, "html", &format!("{}/{}", url, &folder), &host.clone())
                    .await;
                saveNews::savenews(link,&url.to_string(),&folder).await;
            }

            if pagetitles.len() != new_pagetitles.len() {
                break;
            }
        }
    }
    println!("fin");
}
