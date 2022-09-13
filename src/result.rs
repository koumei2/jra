use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

const JRA_RESULT_CNAME: &str = "pw01sli00/AF";

#[derive(Debug, Deserialize, Serialize)]
struct Result {
    race_name: String,
}

pub async fn get() {
    let mut result_list = Vec::new();
    let race_list = super::common::get_race_list(JRA_RESULT_CNAME).await;
    for l in race_list {
        //let contents = get_jra_html(&l).await;
        let contents = get_result_html(&l).await;
        //println!("{}", contents);
        result_list.push(get_result(contents));
    }

    println!("{}", serde_json::to_string(&result_list).unwrap());
}

async fn get_result_html(cname: &str) -> String {
    let response = reqwest::get(super::JRA_URL.to_owned() + cname)
        .await
        .unwrap();
    let body = response.bytes().await.unwrap();
    let (body_utf8, _, _) = encoding_rs::SHIFT_JIS.decode(&body);
    body_utf8.to_string()
}

fn get_result(contents: String) -> Result {
    let fragment = Html::parse_fragment(&contents);

    let race_name_selector = Selector::parse(r#"span.race_name"#).unwrap();
    let race_name = fragment
        .select(&race_name_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();

    let result = Result {
        race_name: race_name.to_string(),
    };
    result
}
