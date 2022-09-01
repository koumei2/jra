use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

const JRA_SYUTSUBAHYO_CNAME: &str = "pw01dli00/F3";

#[derive(Debug, Deserialize, Serialize)]
struct Wakuban {
    race_name: String,
    condition: String,
    price: String,
    start_time: String,
    waku: Vec<Waku>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Waku {
    sex: u8,
    horse_age: u8,
    weight: u16,
    name: String,
    jokey: String,
    wnum: u8,
    unum: u8,
    born: String,
}

pub async fn get() {
    let mut wakulist = Vec::new();
    let race_list = get_race_list().await;
    //println!("{:?}", race_list);
    for l in race_list {
        //let contents = get_jra_html(&l).await;
        let contents = get_waku_html(&l).await;
        //println!("{}", contents);
        wakulist.push(get_waku(contents));
    }

    println!("{}", serde_json::to_string(&wakulist).unwrap());
}

async fn get_race_list() -> Vec<String> {
    let mut r = Vec::new();
    let contents = get_jra_html(JRA_SYUTSUBAHYO_CNAME).await;
    //println!("{}", contents);
    let fragment = Html::parse_fragment(&contents);
    let ul_selector = Selector::parse(r#"ul.grade_race_unit"#).unwrap();
    let a_selector = Selector::parse("div.main a").unwrap();
    //let re = regex::Regex::new(r".*'(.*?)'").unwrap();
    for ul in fragment.select(&ul_selector) {
        //println!("{}", ul.html());
        for a in ul.select(&a_selector) {
            //println!("{:?}", a.value());
            //let click_str = a.value().attr("onclick").unwrap();
            //println!("{}", click_str);
            //let caps = re.captures(click_str).unwrap();
            //r.push(caps[1].to_string());
            r.push(a.value().attr("href").unwrap().to_string());
        }
    }
    r
}

async fn get_jra_html(cname: &str) -> String {
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

async fn get_waku_html(cname: &str) -> String {
    let response = reqwest::get(super::JRA_URL.to_owned() + cname)
        .await
        .unwrap();
    let body = response.bytes().await.unwrap();
    let (body_utf8, _, _) = encoding_rs::SHIFT_JIS.decode(&body);
    body_utf8.to_string()
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
        condition_list.push(condition_category);
    }

    let price_selector = Selector::parse(r#"div.prize_unit .cap"#).unwrap();
    let mut price_text = fragment.select(&price_selector).next().unwrap().text();
    let price = price_text.next().unwrap();
    let price2 = price_text.next().unwrap();
    let price_num_selector = Selector::parse(r#"ul.prize li:first-of-type li"#).unwrap();
    let mut price_num_list = Vec::new();
    for p in fragment.select(&price_num_selector) {
        price_num_list.push(p.text().nth(1).unwrap());
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

    let mut wakulist = Vec::new();
    let waku_selector = Selector::parse(r#"table tbody tr td.waku img"#).unwrap();
    for line in fragment.select(&waku_selector) {
        let re = regex::Regex::new(r#"/(\d)\.png"#).unwrap();
        let caps = re.captures(line.value().attr("src").unwrap()).unwrap();

        let waku = Waku {
            sex: 0,
            horse_age: 0,
            weight: 0,
            name: "".to_string(),
            jokey: "".to_string(),
            wnum: caps[1].parse().unwrap(),
            unum: 1,
            born: "".to_string(),
        };
        wakulist.push(waku);
        //println!("{}", caps[1].to_string());
    }

    let wakuban = Wakuban {
        race_name: race_name.to_string(),
        condition: condition_list.join(" "),
        price: price.to_owned() + price2 + " " + &(price_num_list.join("、")),
        start_time: start_time.to_string(),
        waku: wakulist,
    };
    wakuban
}

/*
my ($tbody) = $content =~ m|<table.*?<tbody>(.*?)</tbody>|s;
#print $tbody;

my @waku = ();
my %sid = ('牡' => 0, '牝' => 1, 'せん' => 2);

#       while ( $tbody =~ m|<tr>.*?waku/(\d+).png.*?<td class="num">\s+(.*?)\s+</td>.*?<div class="name">\s+(.*?)\s+</div>.*?<p class="age">\s+(.*?)(\d+)/.*?<p cl
ass="weight">\s+(.*?)\s+</p>.*?<p class="jockey">(.*?)</p>.*?</tr>|sg ) {
while ( $tbody =~ m|<tr>.*?waku/(\d+).png.*?<td class="num">(.*?)\s*</td>.*?<div class="name">(.*?)</div>.*?<p class="age">(.*?)(\d+)/.*?<p class="weight"
>\s*(.*?)\s*</p>.*?<p class="jockey">(.*?)</p>.*?</tr>|sg ) {
    my $wnum = $1;
    my $unum = $2;
    my $name = $3;
    my $sex = $sid{$4};
    my $yold = $5;
    my $weight = $6;
    my $jokey = $7;

    my $born = '';
    if ($name =~ m|(<img.*?>)|) {
        $born = $1;
    }
    $name =~ s|<.*?>||g;
    $unum =~ s|<.*?>||g;
    $unum = 0 if $unum eq '取消';
    $weight =~ s|<.*?>||g;
    $weight =~ s|\.||;
    $weight =~ s|kg||;
    $jokey =~ s|<.*?>||g;
    $jokey =~ s|^\s+||g;
    $jokey =~ s|\s+$||g;

    my $w = { wnum => $wnum,
              unum => $unum,
              name => $name,
              born => $born,
              sex  => $sex,
              yold => $yold,
              weight => $weight,
              jokey => $jokey,
    };
    push @waku, $w;
}

my $d = { name => $race_name,
          cond => $cond,
          price => $price,
          time  => $time,
          waku => \@waku,
};

push @result, $d;
}
*/
