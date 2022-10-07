use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub const JRA_URL: &str = "https://www.jra.go.jp/";
pub const JRA_ACCESS_URL: &str = "https://www.jra.go.jp/JRADB/accessD.html";

#[derive(Debug, Deserialize, Serialize)]
pub enum WakubanStatus {
    Normal,
    Exclude,
    Stop,
    Cancel,
}

pub async fn get_jra_html(cname: &str) -> String {
    let response = reqwest::get(JRA_URL.to_owned() + cname).await.unwrap();
    let body = response.bytes().await.unwrap();
    let (body_utf8, _, _) = encoding_rs::SHIFT_JIS.decode(&body);
    body_utf8.to_string()
}

pub async fn get_jra_html_using_form(cname: &str) -> String {
    let response = reqwest::Client::new()
        .post(JRA_ACCESS_URL)
        .header(reqwest::header::REFERER, JRA_URL)
        .form(&[("cname", cname)])
        .send()
        .await
        .unwrap();
    let body = response.bytes().await.unwrap();
    let (body_utf8, _, _) = encoding_rs::SHIFT_JIS.decode(&body);
    body_utf8.to_string()
}

pub async fn get_race_list(cname: &str) -> Vec<String> {
    let mut r = Vec::new();
    let contents = get_jra_html_using_form(cname).await;
    //println!("{}", contents);
    let fragment = Html::parse_fragment(&contents);
    let ul_selector = Selector::parse(r#"ul.grade_race_unit"#).unwrap();
    let a_selector = Selector::parse("div.main a").unwrap();
    for ul in fragment.select(&ul_selector) {
        for a in ul.select(&a_selector) {
            let umaban_selector = Selector::parse("span.umaban").unwrap();
            if a.select(&umaban_selector).next().is_none() {
                break;
            }
            //println!("{}", a.html());
            r.push(a.value().attr("href").unwrap().to_string());
        }
    }
    r
}
