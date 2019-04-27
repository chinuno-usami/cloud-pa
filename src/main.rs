extern crate reqwest;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

use threadpool::ThreadPool;

use reqwest::header::USER_AGENT;
use reqwest::header::ORIGIN;
use reqwest::header::REFERER;

use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("云啪啪")
        .version("0.1")
        .author("Usami Chinuno. <usami@chinuno.com>")
        .about("调用https://api.yuncaioo.com/t/pa/接口")
        .arg(Arg::with_name("threads")
             .help("并发线程数")
             .required(true)
             .index(1))
        .arg(Arg::with_name("NAME")
             .help("要啪谁?")
             .possible_values(&["xl", "diy", "wan", "dog", "all"])
             .required(true)
             .index(2))
        .get_matches();
    let n_workers = value_t!(matches, "threads", usize).unwrap_or(10);
    if n_workers < 1 {
        println!("0线程请求？");
        return Ok(());
    }
    println!("Use threads: {}", n_workers);
    let target = matches.value_of("NAME").unwrap();

    let tgt;
    if target == "xl" {
        println!("啪咸狼");
        tgt = "3";
    } else if target == "diy" {
        println!("啪diy");
        tgt = "2";
    } else if target == "wan" {
        println!("啪烷");
        tgt = "1";
    } else if target == "dog" {
        println!("啪狗狗");
        tgt = "4";
    } else if target == "all"{
        println!("全啪啪");
        tgt = "0";
    } else {
        println!("你到底要啪谁！");
        return Ok(());
    }

    let pool = ThreadPool::new(n_workers);
    for _ in 0..n_workers {
        thread::sleep(Duration::from_millis(10));
        pool.execute(move|| {
            let params = [("pa", "true"), ("id", tgt)];
            let client = reqwest::Client::builder()
                .gzip(true).build().unwrap();
            loop {
                let mut res = client.post("https://api.yuncaioo.com/t/pa/pa.php")
                    .form(&params)
                    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36")
                    .header(ORIGIN, "https://api.yuncaioo.com")
                    .header(REFERER, "https://api.yuncaioo.com/t/pa/")
                    .send().unwrap();
                println!("{}",res.text().unwrap());
            }
        });
    }

    pool.join();
    Ok(())

}
