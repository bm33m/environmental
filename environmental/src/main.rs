use std::env;
use std::collections::HashMap;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
// use std::io::Write;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;

use atom;
use probability;
use articles;
use currency;

const PROJECTS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_x = dbg!(args);
    let num = PROJECTS + 1;
    let mut num_2 = probability::get_random(0, num);
    println!("Hello, world! Rand: {}, Num: {}", num_2, atom::add(num, num_2));
    //
    if args_x.len() > 1 {
        let num_x = &args_x[1];
        num_2 = match num_x.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    }

    match num_2 {
        0 => help(),
        1 => test_atoms(),
        2 => test_probability(),
        3 => test_currency(),
        4 => test_articles(),
        5 => test_all(),
        6 => info(),
        7 => todo!(),
        101 => server(),
        _ => test_all(),
    }

}

fn test_all() {
    //
    // DIY project no. 25007
    test_articles();
    //
    // DIY project no. 25001
    test_atoms();
    //
    // DIY project no. 25003
    test_currency();
    //
    // DIY project no. 25002
    test_probability();
    //
}

fn test_articles() {
    //
    let word  = "Cool".to_string();
    let word_check = articles::check_if_word_is_an_article(word);
    articles::print_test("Is Article: ", word_check.0);
    articles::print_test("Language: ", word_check.1);
    articles::print_test("Article: ", word_check.2);
    //
    let word  = "The".to_string();
    let word_check = articles::check_if_word_is_an_article(word);
    articles::print_test("Is Article: ", word_check.0);
    articles::print_test("Language: ", word_check.1);
    articles::print_test("Article: ", word_check.2);
    //
    let word  = "la".to_string();
    let word_check = articles::check_if_word_is_an_article(word);
    articles::print_test("Is Article: ", word_check.0);
    articles::print_test("Language: ", word_check.1);
    articles::print_test("Article: ", word_check.2);
    //
    let word  = "u".to_string();
    let word_check = articles::check_if_word_is_an_article(word);
    articles::print_test("Is Article: ", word_check.0);
    articles::print_test("Language: ", word_check.1);
    articles::print_test("Article: ", word_check.2);
    //
    let mut article_x = articles::Articles {
        sentence: "As an electronics product, the AI Agent model should be able to perform at its optimum for a couple of months.".to_string(), // String,
        number_of_articles: 0, // i32,
        language: "", // &'a str,
        frequency: Vec::new(), // Vec<ArticlesMap<'a>>,
        mode: String::new(), // String,
        mode_count: 0, // i32,
    };
    let count_x = article_x.count_articles();
    articles::print_test("count_x: ", count_x);
    articles::print_test("article_x: ", article_x);
    //
    let mut article_x = articles::Articles {
        sentence: "La durée en temps de parole, en veille, et le cycle de vic total d'une batterie rechargeable module pour AI Agent dépendra des conditions d'usage et des configurations résean.".to_string(), // String,
        number_of_articles: 0, // i32,
        language: "", // &'a str,
        frequency: Vec::new(), // Vec<ArticlesMap<'a>>,
        mode: String::new(), // String,
        mode_count: 0, // i32,
    };
    let count_x = article_x.count_articles();
    articles::print_test("count_x: ", count_x);
    articles::print_test("article_x: ", article_x);
    //
    let mut article_x = articles::Articles {
        sentence: "Kwakukhona ikhehla elalihama lithi, lithi.".to_string(), // String,
        number_of_articles: 0, // i32,
        language: "", // &'a str,
        frequency: Vec::new(), // Vec<ArticlesMap<'a>>,
        mode: String::new(), // String,
        mode_count: 0, // i32,
    };
    let count_x = article_x.count_articles();
    articles::print_test("count_x: ", count_x);
    articles::print_test("article_x: ", article_x);
}


fn test_atoms() {
    let mut atom1 = atom::new("Semiconductor", "Silicon".to_string(), "Si".to_string(),
        14, 28.00, 4,);
    let mut atom2 = atom::new("Substance", "Antinomy".to_string(), "Sb".to_string(),
        51, 122.00, 5,);
    //
    let mut atom3 = atom::new("Semiconductor", "Germanium".to_string(), "Ge".to_string(),
        32, 73.00, 4,);
    let mut atom4 = atom::new("Substance", "Indium".to_string(), "In".to_string(),
        49, 115.00, 3,);
    //
    let result = atom1.doping(&mut atom2);
    assert!(result == true);
    println!("{:?}", atom1);
    println!("{:#?}", atom2);
    println!("{:?}", atom3);
    println!("{:#?}", atom4);
    let result2 = atom3.doping(&mut atom4);
    assert!(result2 == true);
    println!("{:?}", atom3);
    println!("{:#?}", atom4);
}

fn test_currency() {
    let price_info = currency::price_increase_or_decrease_rate(234.00, 123.00);
    println!("{}", price_info);
    //
    // new(name: &str, code: &str, symbol: &str, id: i64)-> Coin
    //
    let coin_x = currency::new("Ubhozo", "Ubh", "Ubh", 9024390243902442);
    let mut currency_x = currency::CurrencyModel {
        coin: coin_x.clone(), // Coin,
        symbol: coin_x.get_currency_symbol(), // String,
        network: coin_x.get_network(), // String,
        sample: Vec::<currency::CurrencyData>::new(), // Vec<CurrencyData>,
    };
    // add_data(code: &str, day: i32, date: &str, month: &str,
    //    day_of_the_week: &str, time: &str, network: &str, price: f64,
    //    sample: &mut Vec<CurrencyData>,)
    currency::add_data("Ubh", 1, "2024/05/14", "May",
        "Tuesday", "13:00:00", "Ubhozo", 13.67,
        &mut currency_x.sample,);
    currency::add_data("Ubh", 2, "2024/05/15", "May",
        "Wednesday", "13:00:00", "Ubhozo", 12.00,
        &mut currency_x.sample,);
    currency::add_data("Ubh", 3, "2024/05/16", "May",
        "Thursday", "11:30:50", "Ubhozo", 14.90,
        &mut currency_x.sample,);
    let info = currency_x.get_currency_info_x();
    println!("{:?}", info);
    println!("{:#?}", info);
    println!("{:?}", info.0.len());
    println!("{:#?}", info.0[2]);
    println!("{:#?}", info.0[2].stats);
    println!("{:#?}", info.0[2].stats.lowest_price);
}


fn test_probability() {
    let t1: f32 = 19.67;
    let t2: f32 = 15.67;
    let t3: f32 = 23.67;
    let data_x1 = probability::EnvinmentalData {
        day: 100,
        date: "2024/05/15".to_string(),
        month: "May".to_string(),
        day_of_the_week: "Wednesday".to_string(),
        time: "14:00:00".to_string(),
        location: "Road".to_string(),
        temperature: t1,
        smoke_detected: true,
        smoke_duration: (2.0 * probability::HOURS),
        comments: String::new(),
        raining: false,
        rain_duration: 0.00,
        toxic_detected: true,
        toxic_smell_duration: (30.0 * probability::MINUTES),
        earthquake: false,
        earthquake_duration: 0.00,
    };
    let data_x2 = probability::EnvinmentalData {
        day: 99,
        date: "2024/05/14".to_string(),
        month: "May".to_string(),
        day_of_the_week: "Tuesday".to_string(),
        time: "15:00:00".to_string(),
        location: "Road".to_string(),
        temperature: t2,
        smoke_detected: true,
        smoke_duration: (3.0 * probability::HOURS),
        comments: String::new(),
        raining: true,
        rain_duration: (20.0 * probability::MINUTES),
        toxic_detected: true,
        toxic_smell_duration: (50.0 * probability::SECONDS),
        earthquake: false,
        earthquake_duration: 0.00,
    };
    let data_x3 = probability::EnvinmentalData {
        day: 98,
        date: "2024/05/13".to_string(),
        month: "May".to_string(),
        day_of_the_week: "Monday".to_string(),
        time: "09:00:00".to_string(),
        location: "Road".to_string(),
        temperature: t3,
        smoke_detected: true,
        smoke_duration: (3.0 * probability::HOURS),
        comments: String::new(),
        raining: false,
        rain_duration: 0.00,
        toxic_detected: true,
        toxic_smell_duration: (10.0 * probability::MINUTES),
        earthquake: false,
        earthquake_duration: 0.00,
    };
    let mut data_x: Vec<probability::EnvinmentalData> = Vec::new();
    data_x.push(data_x1);
    data_x.push(data_x2);
    data_x.push(data_x3);
    let mut model = probability::EnvinmentalModel {
        population: data_x,
        sample: Vec::new(),
    };
    model.sample = model.simple_sampling(&model.population, 2);
    let mean_x = model.get_mean("temperature");
    let m1 = (t1 + t2) / 2.0;
    let m2 = (t1 + t3) / 2.0;
    let m3 = (t2 + t3) / 2.0;
    println!("m1: {}, m2: {}, m3: {}", m1, m2, m3);
    println!("mean: {}", mean_x);
    assert!((mean_x == m1) | (mean_x == m2) | (mean_x == m3) );
    let likelihood = model.get_outcome_likelihood("rain");
    assert!((likelihood.0 == 0.00) | (likelihood.0 == 0.50));
    //
    println!("model: {:#?}", model);
    let likelihood = model.get_outcome_likelihood("rain");
    println!("likelihood: {:?}", likelihood);

}

fn help() {
    let mut usage = HashMap::new();
    usage.insert("help", "cargo run -- 0");
    usage.insert("atom model", "cargo run -- 1");
    usage.insert("probability model", "cargo run -- 2");
    usage.insert("virtual currency model", "cargo run -- 3");
    usage.insert("articles", "cargo run -- 4");
    usage.insert("all", "cargo run -- 5");
    usage.insert("info", "cargo run -- 6");
    usage.insert("server", "cargo run -- 101");
    usage.insert("random", "cargo run");
    usage.insert("test all", "cargo test");
    usage.insert("test atom", "cargo test -p atom");
    usage.insert("test probability", "cargo test -p probability");
    usage.insert("test currency", "cargo test -p currency");
    usage.insert("test articles", "cargo test -p articles");
    let mut i = 1;
    for (key, value) in usage {
        println!("usage #{}, {}: {}", i, key, value);
        i += 1;
    }
}

fn info_x(filename: &str)-> (usize, Vec<articles::Articles>) {
    let contents = fs::read_to_string(filename).unwrap();
    let len = contents.len();
    let sentences: Vec<&str> = contents.split('.').collect();
    let mut articles_list: Vec<articles::Articles> = Vec::new();
    for sentence in sentences {
        // let sentence_x = sentence.clone();
        let articles_x = articles::new(sentence.to_string());
        articles_list.push(articles_x);
    }
    (len, articles_list)
}

fn info() {
    let file_name = "knowledgebase\\README.md";
    let info_x = info_x(file_name);
    println!("info: {:#?}", info_x);
}

fn server()  {
    let address = "127.0.0.1:7878";
    println!("Server is now running at http://{}", address);
    println!("Ctrl C to Quit");
    let listener = TcpListener::bind(address).unwrap();
    let mut i = 1;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move ||{
            handle_connection(stream, i);
        });
        i += 1;
    }
}

fn handle_connection(mut stream: TcpStream, i: i64) {
    let buffer = BufReader::new(&mut stream);
    let request: Vec<_> = buffer.lines().map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()).collect();
    let file_name = "knowledgebase\\README.md";
    let info_x = info_x(file_name);
    let response_x = "HTTP/1.1 200 OK";
    let title = "Environmental";
    let page = format!("<html><title>{}</title><body><div id: info><h1>request: {}</h1> {:#?}</div><div id: data><h1>response: {}</h1> <pre>{:#?}</pre></div></body</html>",
        title, i, request, i, info_x.1);
    let len = info_x.1.len();
    let len_x = len + info_x.0;
    println!("connection: {} response: {}", i, len_x);
    let response = format!("{}\r\n Content-Length: {}\r\n\r\n
        {}", response_x, len_x, page);
    stream.write_all(response.as_bytes()).unwrap();
}
