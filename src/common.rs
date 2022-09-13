use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum WakubanStatus {
    Normal,
    Exclude,
    Stop,
    Cancel,
}

pub async fn get_jra_html_using_form(cname: &str) -> String {
    let response = reqwest::Client::new()
        .post(super::JRA_ACCESS_URL)
        .header(reqwest::header::REFERER, super::JRA_URL)
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
    let fragment = Html::parse_fragment(&contents);
    let ul_selector = Selector::parse(r#"ul.grade_race_unit"#).unwrap();
    let a_selector = Selector::parse("div.main a").unwrap();
    for ul in fragment.select(&ul_selector) {
        for a in ul.select(&a_selector) {
            r.push(a.value().attr("href").unwrap().to_string());
        }
    }
    r
}
