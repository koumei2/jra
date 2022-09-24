use chrono::{Local, NaiveDate};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

const JRA_JYUSYO_LIST_URL: &str = "datafile/seiseki/replay/{year}/jyusyo.html";

#[derive(Debug, Deserialize, Serialize)]
struct Race {
    date: NaiveDate,
    grade: String,
    name: String,
    place: String,
    age: String,
    course_type: String,
    course_distance: u16,
}

pub async fn get(year: u16) {
    let year_str = match year {
        0 => Local::today().format("%Y").to_string(),
        _ => year.to_string(),
    };
    let url = JRA_JYUSYO_LIST_URL.replace("{year}", &year_str);
    let contents = super::common::get_jra_html(&url).await;
    //println!("{}", contents);
    println!(
        "{}",
        serde_json::to_string(&get_races(year_str, contents)).unwrap()
    );
}

fn get_races(year: String, contents: String) -> Vec<Race> {
    let mut grade_race_list = Vec::new();
    let fragment = Html::parse_fragment(&contents);
    let selector = Selector::parse(r"table tbody tr").unwrap();
    for tr in fragment.select(&selector) {
        let date = NaiveDate::parse_from_str(
            &format!("{},{}", year, get_race_td_item(tr, r"td.date", 0)),
            "%Y,%m月%d日",
        )
        .unwrap();

        let selector = Selector::parse(r"td.race span").unwrap();
        let grade = tr
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("class")
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .to_string();

        let course_distance = get_race_td_item(tr, r"td.course", 1)
            .replace(",", "")
            .parse()
            .unwrap();

        grade_race_list.push(Race {
            date: date,
            grade: grade,
            name: get_race_name(tr),
            place: get_race_td_item(tr, r"td.place", 0),
            age: get_race_td_item(tr, r"td.age", 0),
            course_type: get_race_td_item(tr, r"td.course", 0),
            course_distance: course_distance,
        })
    }
    grade_race_list
}

fn get_race_td_item(element: ElementRef, query: &str, n: usize) -> String {
    let selector = Selector::parse(query).unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .nth(n)
        .unwrap()
        .to_string()
}

fn get_race_name(element: ElementRef) -> String {
    let selector = Selector::parse(r"td.race").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .last()
        .unwrap()
        .to_string()
}
