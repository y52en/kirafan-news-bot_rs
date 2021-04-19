use crate::archive::*;
use crate::module::*;
use once_cell::sync::Lazy;
use regex::Regex;

#[allow(non_upper_case_globals, dead_code)]
static re_js_css_link: Lazy<Regex> = Lazy::new(|| compiled_regex(r#""([^?"]+)"#));
#[allow(non_upper_case_globals, dead_code)]
static re_css_link: Lazy<Regex> = Lazy::new(|| compiled_regex(r#"href="([^?"]+)"#));
#[allow(non_upper_case_globals, dead_code)]
static re_img_link: Lazy<Regex> = Lazy::new(|| compiled_regex(r#"src="([^?"]+)"#));
#[allow(non_upper_case_globals, dead_code)]
static re_assets_link: Lazy<Regex> = Lazy::new(|| compiled_regex(r#"url\(([^?)]+)"#));
#[allow(non_upper_case_globals, dead_code)]
static sel_js_link: Lazy<scraper::Selector> = Lazy::new(|| compiled_selector(r#"script[src]"#));
#[allow(non_upper_case_globals, dead_code)]
static sel_css_link: Lazy<scraper::Selector> =
    Lazy::new(|| compiled_selector(r#"link[rel="stylesheet"]"#));
#[allow(non_upper_case_globals, dead_code)]
static sel_img_link: Lazy<scraper::Selector> = Lazy::new(|| compiled_selector(r#"img[src]"#));
#[allow(non_upper_case_globals, dead_code)]
static sel_assets_link: Lazy<scraper::Selector> =
    Lazy::new(|| compiled_selector(r#"div[style*='background-image']"#));

pub async fn savenews(
    path: String,
    url: String,
    folder: String,
    mut process_list: Vec<tokio::task::JoinHandle<()>>,
) -> Vec<tokio::task::JoinHandle<()>> {
    let html = get_html_retry(&path, 5).await;
    let doc = parse_html(&html);

    let js_link = scraping(&re_img_link, &sel_js_link, &doc);
    let css_link = scraping(&re_css_link, &sel_css_link, &doc);

    let img_link = scraping(&re_img_link, &sel_img_link, &doc);
    let assets_link = scraping(&re_assets_link, &sel_assets_link, &doc);

    for js in js_link {
        let path_clone = path.clone();
        let process = tokio::spawn(async move {
            archive_file(&js, "js", "", &path_clone).await;
        });
        process_list.push(process);
    }

    for css in css_link {
        let path_clone = path.clone();
        let process = tokio::spawn(async move {
            archive_file(&css, "css", "", &path_clone).await;
        });
        process_list.push(process);
    }

    for asset in assets_link {
        let path_clone = path.clone();
        let process = tokio::spawn(async move {
            archive_file(&asset, "asset", "", &path_clone).await;
        });
        process_list.push(process);
    }

    for img in img_link {
        let path_clone = path.clone();
        let url_clone = url.clone();
        let folder_clone = folder.clone();
        let process = tokio::spawn(async move {
            archive_file(
                &img,
                "img",
                &format!("{}{}{}{}", url_clone.clone(), "/", folder_clone, "/"),
                &path_clone.clone(),
            )
            .await;
        });
        process_list.push(process);
    }

    return process_list;
}
