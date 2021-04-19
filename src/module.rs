use std::fs;
use std::fs::File;
// use std::io;
use std::io::Write;

// use std::io::Write;
use std::path::Path;

use regex::Regex;
use reqwest::Error;

use tokio::time::*;
use once_cell::sync::Lazy;

// use std::fs::File;

    use twitter_text::parse;



#[allow(non_upper_case_globals, dead_code)]
static re_rem_lastline: Lazy<Regex> = Lazy::new(|| compiled_regex(r"\n$"));
#[allow(non_upper_case_globals, dead_code)]
static re_rem_firstline: Lazy<Regex> = Lazy::new(|| compiled_regex(r"^\n"));


pub async fn urlretrieve(url: &str, path_s: &str) -> Result<(), Error> {
    let path = Path::new(&path_s);
    let mut out = tokio::fs::File::create(path).await.expect("failed to create file");
    let resp = reqwest::get(url).await;
    if !resp.is_ok() {
        tokio::fs::remove_file(path).await.unwrap();
        return Err(resp.unwrap_err());
    };
    let bytes = resp.unwrap().bytes().await.unwrap();
    println!("{:#?}", url);
    tokio::io::copy(&mut bytes.as_ref(), &mut out).await.expect("failed to copy content");
    return Ok(());
}

pub async fn get_html(url: &str) -> Result<String, Error> {
    let req = reqwest::get(url).await;
    match req {
        Ok(html) => Ok(html.text().await.unwrap()),
        Err(e) => Err(e),
    }
}

pub async fn get_html_retry(url: &str, mut rem_retry: i32) -> String {
    // println!("{}",url);

    while rem_retry > 0 {
        let html_feature = get_html(&url).await;
        if html_feature.is_ok() {
            return html_feature.unwrap();
        }
        rem_retry -= 1;
        wait(5000).await;
    }
    panic!("err")
}

pub async fn wait(millsec: u64) {
    sleep(Duration::from_millis(millsec)).await;
}

pub fn re_find(regex: &Regex, text: &str) -> String {
    regex
        .captures(text)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}

pub fn is_re_match(regex: &Regex, text: &str) -> bool {
    let result = regex.captures(text);
    match result {
        Some(_) => {
            let result2 = result.unwrap().get(1);
            match result2 {
                Some(_) => true,
                None => false,
            }
        }
        None => false,
    }
}

// https://doc.rust-jp.rs/rust-by-example-ja/std_misc/fs.html
pub async fn mkdirs(path: &str) {
    tokio::fs::create_dir_all(&path).await.unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn compiled_selector(selector: &str) -> scraper::Selector {
    scraper::Selector::parse(&selector).unwrap()
}

pub fn compiled_regex(reg: &str) -> Regex {
    Regex::new(reg).unwrap()
}

pub fn readfile_asline(path: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !is_path_exist(&path) {
        fs::File::create(path).unwrap();
    }
    let content = fs::read_to_string(path)?;
    let rm_lastline = re_rem_lastline.replace(&content,"");
    let splited = split_by_line(&rm_lastline);
    // println!("{}", content);
    Ok(splited)
    // let c = rm_lastline.split("\n").collect();
    // Ok(c)
}

pub fn writefile_asline(path: &String,vec:Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_to = vec.join("\n");
    write_to += "\n";
    write_to = re_rem_firstline.replace(&write_to,"").to_string();

    let mut file = File::create(path)?;
    write!(file, "{}", write_to)?;
    file.flush()?;
    Ok(())
}

pub fn is_path_exist(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn split_by_line(text: &str) -> Vec<String> {
    text.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn is_array_contein(in_vec: [&str; 4], text: &str) -> bool {
    in_vec.iter().any(|&i| i == text)
}

pub fn is_vec_contein(in_vec: &Vec<String>, text: &str) -> bool {
    in_vec.iter().any(|i| i == text)
}

pub fn parse_html(html: &str) -> scraper::Html {
    scraper::Html::parse_document(html)
}

pub fn scraping(regex: &Regex, selector: &scraper::Selector, html: &scraper::Html) -> Vec<String> {
    let mut x = vec![];

    for elm in html.select(&selector) {
        x.push(re_find(regex, &elm.html()))
    }
    return x;
}

pub fn count_twitter_str(tweet:&str) -> i32 {
    let config = twitter_text_config::default();
    return parse(&tweet, config ,true).weighted_length;
}
