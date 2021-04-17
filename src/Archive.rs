use crate::Mod::*;
use once_cell::sync::Lazy;
use regex::Regex;

#[allow(non_upper_case_globals, dead_code)]
static re_is_root: Lazy<Regex> = Lazy::new(|| compiled_regex(r"^(/)"));
#[allow(non_upper_case_globals, dead_code)]
static re_host: Lazy<Regex> = Lazy::new(|| compiled_regex(r"(https?://[^/]+)/"));
#[allow(non_upper_case_globals, dead_code)]
static re_filename: Lazy<Regex> = Lazy::new(|| compiled_regex(r"/(([^/]+$))"));

static HOMEPATH: &str = "/home/y52en";

pub async fn savefile(url: &String, path: String) {
    if !is_pathExist(&path) {
        let mut rem_retry: i32 = 3;
        while rem_retry > 0 {
            let html_feature = urlretrieve(url, &path).await;
            if html_feature.is_ok() {
                return ();
            }
            rem_retry -= 1;
            println!("retry {}", (3 - rem_retry));
        }
        panic!("failed to save file");
    }
}

pub async fn archive_file(in_url: &String, filetype: &str, savepath: &String, baseurl: &String) {
    let mut url: String = in_url.to_string();
    let no_downloadlist = [
        "https://krr-dev-web.star-api.com/wp-content/uploads/2019/09/専用武器追加_201910-1.png",
        "https://krr-dev-web.star-api.com/wp-content/uploads/2019/05/NEW-GAME_-期間限定特別_クロモン.png",
        "http://krr-dev-web.star-api.com/wp-content/uploads/2018/05/Profile_naru_hanayamata1_1_Ud7fKyWG.png",
        "https://krr-dev-web.star-api.com/wp-content/uploads/2018/08/29002000用帯.png",
    ];

    if is_array_contein(no_downloadlist, &url) {
        return;
    };

    if is_re_match(&re_is_root, &url).is_ok() {
        let host = re_find(&re_host, &baseurl);
        url = host + &url;
    };

    let filename = if filetype != "html" {
        re_find(&re_filename, &url)
    } else {
        "index.html".to_string()
    };

    match filetype {
        "js" => {
            savefile(
                &url,
                format!("{}{}{}", HOMEPATH, "/kirafan-news_rs/assets/js/", &filename),
            )
            .await
        }
        "css" => {
            savefile(
                &url,
                format!(
                    "{}{}{}",
                    HOMEPATH, "/kirafan-news_rs/assets/css/", &filename
                ),
            )
            .await
        }
        "asset" => {
            savefile(
                &url,
                format!(
                    "{}{}{}",
                    HOMEPATH, "/kirafan-news_rs/assets/img/", &filename
                ),
            )
            .await
        }
        "img" => {
            savefile(
                &url,
                format!(
                    "{}{}{}{}{}",
                    HOMEPATH, "/kirafan-news_rs/news/", &savepath,"/", &filename
                ),
            )
            .await
        }
        "html" => {
            savefile(
                &url,
                format!(
                    "{}{}{}{}{}",
                    HOMEPATH, "/kirafan-news_rs/news/", &savepath,"/" , &filename
                ),
            )
            .await;
        }
        other => panic!("filetype is wrong:{:#?}", other),
    };
}
