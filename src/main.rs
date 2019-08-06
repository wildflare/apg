// Air Pressure Graph

extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

const BP_RANGE: usize = 21;
const TIME_RANGE: usize = 24;
const GRAPH_CENTER: usize = TIME_RANGE / 2;
const MAGNIFICATION: usize = 4;

fn round(f: f32) -> usize {
    (f + 0.5) as usize
}

fn get_web_body(url: &str) -> String {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    resp.text().unwrap()
}

fn scraype_pressure_data(body: &str, data: &mut Vec<f32>) {
    const OFFSET: usize = 3;
    const DATA_POSITION: usize = 9;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("#tbl_list > tbody > tr").unwrap();

    let mut cnt = 0;
    for value in fragment.select(&selector) {
        cnt += 1;
        let value_txt = value.text().collect::<Vec<_>>();
        if cnt < OFFSET {
            continue;
        }
        match value_txt[DATA_POSITION].parse::<f32>() {
            Ok(n) => data.push(n),
            Err(_err) => break,
        }
    }
}

fn scraype_date_time(body: &str) -> String {
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("#tbl_title > tbody > tr").unwrap();

    let mut res: String = "".to_string();
    let mut cnt = 0;
    for value in fragment.select(&selector) {
        cnt += 1;
        // #tbl_title > tbody > tr:nth-child(2)
        if cnt == 2 {
            let value_txt = value.text().collect::<Vec<_>>();
            res = value_txt[1].to_string();
            break;
        }
    }
    return res;
}

fn get_range_offset(data: &Vec<f32>) -> usize {
    let n = data.len();
    round(data[n - GRAPH_CENTER]) + (BP_RANGE / 2 / MAGNIFICATION)
}

fn get_time_offset(data: &Vec<f32>) -> usize {
    data.len() - TIME_RANGE + 1
}

fn set_field(fld: &mut [Vec<i32>], data: &Vec<f32>) {
    let r_offset = get_range_offset(data);
    let t_offset = get_time_offset(data) - 1;

    let mut cnt: usize = 0;
    for dt in &data[t_offset..] {
        let i: isize = (r_offset as isize) * (MAGNIFICATION as isize)
            - (round(*dt * (MAGNIFICATION as f32)) as isize);
        if i >= 0 && i < BP_RANGE as isize {
            let row: usize = i as usize;
            fld[row][cnt] = 1;
        }
        cnt += 1;
    }
}

fn print_field(fld: &mut [Vec<i32>], data: &Vec<f32>) {
    let r_offset = get_range_offset(data);
    let t_offset = get_time_offset(data);

    println!("");
    println!("      時刻→");
    print!("↓気圧");
    for i in t_offset..t_offset + 24 {
        print!("{0:^3}", i % 24);
    }
    println!("");

    let mut dot: String;
    let mut atmark: String;

    let mut cnt: usize = 0;
    for row in fld {
        let step: f32 = r_offset as f32 - (cnt as f32) / (MAGNIFICATION as f32);
        if ((step * 10.0) as isize) % 10 == 0 {
            print!("{0:>6}", step);
            dot = "...".to_string();
            atmark = ".@.".to_string();
        } else {
            print!("      ");
            dot = "   ".to_string();
            atmark = " @ ".to_string();
        }
        cnt += 1;
        for col in row {
            let cell = if *col == 0 {
                dot.to_string()
            } else {
                atmark.to_string()
            };
            print!("{}", cell);
        }
        println!("");
    }
}

fn main() {
    const YESTERDAY_URL: &str = "http://www.jma.go.jp/jp/amedas_h/yesterday-44132.html";
    const TODAY_URL: &str = "http://www.jma.go.jp/jp/amedas_h/today-44132.html";

    let mut field: Vec<Vec<i32>> = vec![vec![0; TIME_RANGE]; BP_RANGE];
    let mut data: Vec<f32> = Vec::new();

    let body = get_web_body(YESTERDAY_URL);
    scraype_pressure_data(&body, &mut data);

    let body = get_web_body(TODAY_URL);
    scraype_pressure_data(&body, &mut data);
    let title = scraype_date_time(&body);

    set_field(&mut field, &data);
    print_field(&mut field, &data);
    println!("\t\t\t {}", title);
}
