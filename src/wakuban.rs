use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

const JRA_SYUTSUBAHYO_CNAME: &str = "pw01dli00/F3";

#[derive(Debug, Deserialize, Serialize)]
struct Wakuban {
    race_name: String,
    condition: Vec<String>,
    price: Vec<String>,
    start_time: String,
    waku: Vec<Waku>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Waku {
    horse: Horse,
    weight: String,
    jokey: String,
    bracket_number: u8,
    horse_number: Option<u8>,
    status: super::common::WakubanStatus,
}

#[derive(Debug, Deserialize, Serialize)]
struct Horse {
    sex: String,
    age: u8,
    name: String,
    icon: Option<String>,
}

pub async fn get() {
    let mut wakulist = Vec::new();
    let race_list = super::common::get_race_list(JRA_SYUTSUBAHYO_CNAME).await;
    //println!("{:?}", race_list);
    for l in race_list {
        let contents = super::common::get_jra_html(&l).await;
        //println!("{}", contents);
        wakulist.push(get_waku(contents));
    }

    println!("{}", serde_json::to_string(&wakulist).unwrap());
}

fn get_waku(contents: String) -> Wakuban {
    let fragment = Html::parse_fragment(&contents);

    let race_name_selector = Selector::parse(r#"span.race_name"#).unwrap();
    let race_name = fragment
        .select(&race_name_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();

    let condition_type = ["category", "class", "rule", "weight"];
    let mut condition_list = Vec::new();
    for t in condition_type {
        let condition_selector = Selector::parse(&("div.type div.".to_owned() + t)).unwrap();
        let condition_category: &str = fragment
            .select(&condition_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        condition_list.push(condition_category.to_string());
    }

    let mut price_num_list = Vec::new();
    let price_num_selector = Selector::parse(r#"ul.prize li:first-of-type li"#).unwrap();
    for p in fragment.select(&price_num_selector) {
        price_num_list.push(p.text().nth(1).unwrap().to_string());
    }

    let start_time_selector = Selector::parse(r#"div.race_header div.time"#).unwrap();
    let start_time = fragment
        .select(&start_time_selector)
        .next()
        .unwrap()
        .text()
        .nth(1)
        .unwrap()
        .to_string()
        .replace("時", ":")
        .replace("分", "");

    // waku
    let mut wakulist = Vec::new();
    let waku_selector = Selector::parse(r#"table.mt20 tbody tr"#).unwrap();
    for wakudata in fragment.select(&waku_selector) {
        // wakuban
        let wakuban_selector = Selector::parse(r#"td.waku img"#).unwrap();
        let wakuban = wakudata
            .select(&wakuban_selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap();
        let re = regex::Regex::new(r#"/(\d)\.png"#).unwrap();
        let waku_caps = re.captures(wakuban).unwrap();

        // umaban
        let umaban_selector = Selector::parse(r#"td.num"#).unwrap();
        let umaban_str = wakudata
            .select(&umaban_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        let status = match umaban_str {
            "除外" => super::common::WakubanStatus::Exclude,
            _ => super::common::WakubanStatus::Normal,
        };

        // name and icon
        let name_and_icon_selector = Selector::parse(r#"div.name"#).unwrap();
        let name_and_icon = wakudata.select(&name_and_icon_selector).next().unwrap();
        let icon_selector = Selector::parse(r#"span.horse_icon img"#).unwrap();
        // 複数未対応。例待ち
        let icons = name_and_icon.select(&icon_selector).next();
        let horse_icon = match icons {
            Some(v) => Some(v.html()),
            _ => None,
        };
        let name = name_and_icon.text().next().unwrap();

        // sex and age
        let sex_and_age_selector = Selector::parse(r#"p.age"#).unwrap();
        let sex_and_age = wakudata
            .select(&sex_and_age_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let re = regex::Regex::new(r#"(.*?)(\d+)"#).unwrap();
        let sex_and_age_caps = re.captures(sex_and_age).unwrap();

        // weight
        let weight_selector = Selector::parse(r#"p.weight"#).unwrap();
        let weight = wakudata
            .select(&weight_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        // jockey
        let jockey_selector = Selector::parse(r#"p.jockey"#).unwrap();
        let jockey = wakudata
            .select(&jockey_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        let waku = Waku {
            horse: Horse {
                name: name.to_string(),
                sex: sex_and_age_caps[1].to_string(),
                age: sex_and_age_caps[2].parse().unwrap(),
                icon: horse_icon,
            },
            weight: weight.trim().to_string(),
            jokey: jockey.to_string(),
            bracket_number: waku_caps[1].parse().unwrap(),
            horse_number: umaban_str.parse::<u8>().ok(),
            status: status,
        };

        wakulist.push(waku);
        //println!("{}", caps[1].to_string());
    }

    let wakuban = Wakuban {
        race_name: race_name.to_string(),
        condition: condition_list,
        price: price_num_list,
        start_time: start_time.to_string(),
        waku: wakulist,
    };
    wakuban
}
