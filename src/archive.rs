use crate::module::*;
use once_cell::sync::Lazy;
use regex::Regex;

#[allow(non_upper_case_globals, dead_code)]
static re_is_root: Lazy<Regex> = Lazy::new(|| compiled_regex(r"^/"));
#[allow(non_upper_case_globals, dead_code)]
static re_host: Lazy<Regex> = Lazy::new(|| compiled_regex(r"(https?://[^/]+)/"));
#[allow(non_upper_case_globals, dead_code)]
static re_filename: Lazy<Regex> = Lazy::new(|| compiled_regex(r"/(([^/]+$))"));

#[allow(non_upper_case_globals, dead_code)]
static re_path: Lazy<Regex> = Lazy::new(|| compiled_regex(r"//(.+)$"));
#[allow(non_upper_case_globals, dead_code)]
static re_folder: Lazy<Regex> = Lazy::new(|| compiled_regex(r"//((?:[^/]+/)+)[^/]*$"));
#[allow(non_upper_case_globals, dead_code)]
static re_is_html: Lazy<Regex> = Lazy::new(|| compiled_regex(r"/$"));

#[allow(non_upper_case_globals, dead_code)]
static re_slash: Lazy<Regex> = Lazy::new(|| compiled_regex(r"/"));

static HOMEPATH: &str = "/home/pi/kirafan-news_rs";
// static HOMEPATH: &str = "/home/y52en/kirafan-news_rs";

pub async fn savefile(url: &str, date: &str, title: &str) {
    let folder = re_find(&re_folder, &url);
    let url_path = re_find(&re_path, &url);
    let url_path_clone = url_path.clone();
    let mut path = format!("{}/{}", HOMEPATH, url_path_clone);
    if is_re_match(&re_is_html, &url) {
        path += &format!(
            "{}{}{}{}",
            date.replace("/", "／"),
            re_slash.replace_all(title,"／"),
            "/",
            &"index.html"
        );
    };
    mkdirs(&format!(
        "{}{}{}{}{}{}{}",
        HOMEPATH,
        "/",
        folder,
        "/",
        date.replace("/", "／"),
        re_slash.replace_all(title,"／"),
        "/"
    ))
    .await;

    if !is_path_exist(&path) {
        let mut rem_retry: i32 = 3;
        while rem_retry > 0 {
            let html_feature = urlretrieve(url, &path).await;
            if html_feature.is_ok() {
                return ();
            }
            rem_retry -= 1;
            println!("retry {}", (3 - rem_retry));
            wait(5000).await;
        }
        panic!("failed to save file:{}", url);
    }
}

pub async fn archive_file(
    in_url: &str,
    _filetype: &str,
    _savepath: &str,
    baseurl: &str,
    date: &str,
    title: &str,
) {
    let mut url = in_url.to_string();
    let no_downloadlist = [
        "https://krr-dev-web.star-api.com/wp-content/uploads/2019/09/専用武器追加_201910-1.png",
        "https://krr-dev-web.star-api.com/wp-content/uploads/2019/05/NEW-GAME_-期間限定特別_クロモン.png",
        "http://krr-dev-web.star-api.com/wp-content/uploads/2018/05/Profile_naru_hanayamata1_1_Ud7fKyWG.png",
        "https://krr-dev-web.star-api.com/wp-content/uploads/2018/08/29002000用帯.png",
    ];

    if is_array_contein(no_downloadlist, &url) {
        return;
    };

    if is_re_match(&re_is_root, &url) {
        let host = re_find(&re_host, &baseurl);
        url = host + &url;
    };

    // let filename = if filetype != "html" {
    //     re_find(&re_filename, &url)
    // } else {
    //     "index.html".to_string()
    // };

    savefile(&url, date, title).await;

    // match filetype {
    //     "js" => savefile(&url, &format!("{}{}{}", HOMEPATH, "/assets/js/", &filename)).await,
    //     "css" => {
    //         savefile(
    //             &url,
    //             &format!("{}{}{}", HOMEPATH, "/assets/css/", &filename),
    //         )
    //         .await
    //     }
    //     "asset" => {
    //         savefile(
    //             &url,
    //             &format!("{}{}{}", HOMEPATH, "/assets/img/", &filename),
    //         )
    //         .await
    //     }
    //     "img" => {
    //         savefile(
    //             &url,
    //             &format!("{}{}{}{}{}", HOMEPATH, "/news/", &savepath, "/", &filename),
    //         )
    //         .await
    //     }
    //     "html" => {
    //         savefile(
    //             &url,
    //             &format!("{}{}{}{}{}", HOMEPATH, "/news/", &savepath, "/", &filename),
    //         )
    //         .await;
    //     }
    //     other => panic!("filetype is wrong:{:#?}", other),
    // };
}
