use crate::Archive;
use crate::Mod::*;
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

pub async fn savenews(path: &String, url: &String, folder: &String) {
    let html = get_html_retry(path, 3).await;
    let doc = parse_html(&html);

    let js_link = scraping(&re_js_css_link, &sel_js_link, &doc);
    let css_link = scraping(&re_css_link, &sel_css_link, &doc);

    let img_link = scraping(&re_img_link, &sel_img_link, &doc);
    let assets_link = scraping(&re_assets_link, &sel_assets_link, &doc);

    for js in js_link {
        Archive::archive_file(&js, "js", &"".to_string(), &path.to_string()).await;
    }

    for css in css_link {
        Archive::archive_file(&css, "css", &"".to_string(), &path.to_string()).await;
    }

    for asset in assets_link {
        Archive::archive_file(&asset, "asset", &"".to_string(), &path.to_string()).await;
    }

    for img in img_link {
        Archive::archive_file(
            &img,
            "img",
            &format!("{}{}{}", url,"/", folder),
            &path.to_string(),
        )
        .await;
    }
}
