use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use regex::Regex;
use reqwest::Error;

pub async fn urlretrieve(url: &String, path_s: &String) -> Result<(), Error> {
    let path = Path::new(&path_s);
    let resp = reqwest::get(url)
        .await
        .expect("request failed")
        .bytes()
        .await
        .unwrap();
    println!("{:#?}{:#?}", url, Path::new(path_s));
    let mut out = File::create(path).expect("failed to create file");
    io::copy(&mut resp.as_ref(), &mut out).expect("failed to copy content");
    return Ok(());
}

pub async fn get_html(url: &String) -> Result<String, Error> {
    let req = reqwest::get(url).await;
    match req {
        Ok(html) => Ok(html.text().await.unwrap()),
        Err(e) => Err(e),
    }
}

pub async fn get_html_retry(url: &String, mut rem_retry: i32) -> String {
                    // println!("{}",url);

    while rem_retry > 0 {
        let html_feature = get_html(&url).await;
        if html_feature.is_ok() {
            return html_feature.unwrap();
        }
        rem_retry -= 1;
    }
    panic!("err")
}

pub fn re_find(regex: &Regex, string: &str) -> String {
    regex
        .captures(string)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}

pub fn is_re_match(regex: &Regex, string: &str) -> Result<(), ()> {
    let result = regex.captures(string);
    match result {
        Some(_) => {
            let result2 = result.unwrap().get(1);
            match result2 {
                Some(_) => Ok(()),
                None => Err(()),
            }
        }
        None => Err(()),
    }
}

// https://doc.rust-jp.rs/rust-by-example-ja/std_misc/fs.html
pub fn mkdirs(path: String) {
    fs::create_dir_all(&path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn compiled_selector(selector: &str) -> scraper::Selector {
    scraper::Selector::parse(&selector).unwrap()
}

pub fn compiled_regex(reg: &str) -> Regex {
    Regex::new(reg).unwrap()
}

pub fn readfile(path: &String) -> Result<String, Box<std::error::Error>> {
    if !is_pathExist(&path) {
        fs::File::create(path).unwrap();
    }
    let content = fs::read_to_string(path)?;
    // println!("{}", content);
    Ok(content)
}

pub fn is_pathExist(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn split_by_line(string: &String) -> Vec<&str> {
    string.split("\n").collect()
}

pub fn is_array_contein(in_vec: [&str; 4], string: &String) -> bool {
    in_vec.iter().any(|&i| i == string)
}

pub fn parse_html(html: &String) -> scraper::Html {
    scraper::Html::parse_document(html)
}

pub fn scraping(regex: &Regex, selector: &scraper::Selector, html: &scraper::Html) -> Vec<String> {
    let mut x = vec![String::from(""); 0];

    for elm in html.select(&selector) {
        x.push(re_find(regex, &elm.html()))
    }
    return x;
}
