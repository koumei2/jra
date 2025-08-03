use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

const JRA_RESULT_CNAME: &str = "pw01sli00/AF";

#[derive(Debug, Deserialize, Serialize)]
struct RaceResult {
    race_name: String,
    baba: Vec<(String, String)>,
    result_time: Vec<(String, String)>,
    refund: Refund,
    horses: Vec<ResultHorse>,
    record: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Refund {
    win: Vec<u32>,     // 単勝
    place: Vec<u32>,   // 複勝
    wakuren: Vec<u32>, // 枠連
    wide: Vec<u32>,    // ワイド
    umaren: Vec<u32>,  // 馬連
    umatan: Vec<u32>,  // 馬単
    trio: Vec<u32>,    // 三連複
    tierce: Vec<u32>,  // 三連単
}

#[derive(Debug, Deserialize, Serialize)]
struct ResultHorse {
    horse_number: u8,
    waku_number: u8,
    place: Option<u8>,
    time: String,
    margin: String,
    horse_weight: Option<u16>,
    horse_weight_changes: String,
    popularity: Option<u8>,
    status: super::common::WakubanStatus,
}

pub async fn get() {
    let mut result_list = Vec::new();
    let race_list = super::common::get_race_list(JRA_RESULT_CNAME, false).await;
    for l in race_list {
        let contents = super::common::get_jra_html(&l).await;
        //println!("{}", contents);
        result_list.push(get_result(contents));
    }

    println!("{}", serde_json::to_string(&result_list).unwrap());
}

fn get_result(contents: String) -> RaceResult {
    let fragment = Html::parse_fragment(&contents);

    let race_name_selector = Selector::parse(r#"span.race_name"#).unwrap();
    let race_name = fragment
        .select(&race_name_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string();

    let baba_selector = Selector::parse(r#"div.baba li"#).unwrap();
    let mut baba = Vec::new();
    for li in fragment.select(&baba_selector) {
        let mut text = li.text();
        let label = text.next().unwrap().to_string();
        let value = text.next().unwrap().to_string();
        baba.push((label, value));
    }

    let result_time_selector = Selector::parse(r#"div.result_time_data tr"#).unwrap();
    let mut result_time = Vec::new();
    for tr in fragment.select(&result_time_selector) {
        let mut t = tr.children().filter(|x| x.value().is_element());
        let result_time_label = ElementRef::wrap(t.next().unwrap())
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();
        let result_time_value = ElementRef::wrap(t.next().unwrap())
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();
        result_time.push((result_time_label, result_time_value));
    }

    let result = RaceResult {
        race_name: race_name,
        baba: baba,
        result_time: result_time,
        refund: Refund {
            win: get_result_refund(&fragment, "win"),
            place: get_result_refund(&fragment, "place"),
            wakuren: get_result_refund(&fragment, "wakuren"),
            wide: get_result_refund(&fragment, "wide"),
            umaren: get_result_refund(&fragment, "umaren"),
            umatan: get_result_refund(&fragment, "umatan"),
            trio: get_result_refund(&fragment, "trio"),
            tierce: get_result_refund(&fragment, "tierce"),
        },
        horses: get_result_horses(&fragment),
        record: get_record(&fragment),
    };
    result
}

fn get_result_horses(fragment: &Html) -> Vec<ResultHorse> {
    let mut horses = Vec::new();
    let horse_table_selector = Selector::parse(r"div.race_result_unit table tbody").unwrap();
    let horse_table = fragment.select(&horse_table_selector).next().unwrap();
    let horse_selector = Selector::parse("tr").unwrap();
    for h in horse_table.select(&horse_selector) {
        //println!("{}", h.html());

        // 着順
        let selector = Selector::parse("td.place").unwrap();
        let place_str = h.select(&selector).next().unwrap().text().next().unwrap();
        let (place, status) = match place_str {
            "除外" => (None, super::common::WakubanStatus::Exclude),
            "中止" => (None, super::common::WakubanStatus::Stop),
            "取消" => (None, super::common::WakubanStatus::Cancel),
            _ => (place_str.parse().ok(), super::common::WakubanStatus::Normal),
        };

        // 枠番
        let selector = Selector::parse("td.waku img").unwrap();
        let wakuban = h
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap();
        let re = regex::Regex::new(r#"/(\d)\.png"#).unwrap();
        let waku_caps = re.captures(wakuban).unwrap();

        // 馬番
        let selector = Selector::parse("td.num").unwrap();
        let horse_number = h
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        // タイム
        let selector = Selector::parse("td.time").unwrap();
        let time = h
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap_or("")
            .to_string();

        // 着差
        let selector = Selector::parse("td.margin").unwrap();
        let margin = h
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .trim()
            .to_string();

        // 馬体重
        let selector = Selector::parse("td.h_weight").unwrap();
        let mut horse_weight_text = h.select(&selector).next().unwrap().text();
        let horse_weight = horse_weight_text.next().unwrap().parse().ok();
        let horse_weight_change = horse_weight_text.next().unwrap_or("").to_string();

        // 人気
        let selector = Selector::parse("td.pop").unwrap();
        let popularity_text = h.select(&selector).next().unwrap().text().next();
        let popularity = match popularity_text {
            Some(v) => v.parse().ok(),
            None => None,
        };

        horses.push(ResultHorse {
            horse_number: horse_number,
            waku_number: waku_caps[1].parse().unwrap(),
            place: place,
            time: time,
            margin: margin,
            horse_weight: horse_weight,
            horse_weight_changes: horse_weight_change,
            popularity: popularity,
            status: status,
        })
    }
    horses
}

fn get_result_refund(fragment: &Html, kind: &str) -> Vec<u32> {
    let selector = Selector::parse(&format!(r"div.refund_area li.{} div.yen", kind)).unwrap();

    fragment
        .select(&selector)
        .filter_map(|x| x.text().next())
        .map(|x| x.replace(",", "").parse().unwrap())
        .collect()
}

fn get_record(fragment: &Html) -> bool {
    let selector = Selector::parse("strong.record").unwrap();
    fragment.select(&selector).next().is_some()
}
