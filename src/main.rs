use core::result::Result::Ok;
use anyhow::Result;
use happyyear::api::{get_location, get_weather, get_shrine};
use happyyear::{get_message, get_art, get_omikuji, get_global_ip};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use chrono::Utc;
use chrono_tz::Asia::Tokyo;

#[derive(Parser)]
struct Cli {

    // 出力する文章の種類を指定する(0~2)
    #[clap(short='t', default_value_t=0)]
    message_type: usize,

    // アスキーアートを指定する(0~2)
    #[clap(short='a', default_value_t=0)]
    art: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let delay = Duration::from_millis(50);
    let args = Cli::parse();
    let message = get_message(args.message_type);
    let art = get_art(args.art);

    // 現在時刻を取得する
    let utc = Utc::now();
    let jst = utc.with_timezone(&Tokyo);

    // 標準出力
    let out = stdout();
    let mut out = out.lock();

    // 日付を出力する
    write!(out, "\n\t今日は{}です。\n", jst.format("%Y年%m月%d日"))?;
    // メッセージを出力する
    write!(out, "{}\n", message)?;
    // おみくじの内容を出力する
    write!(out, "\tおみくじで貴方の運勢を占います。\n")?;
    write!(out, "\tおみくじの結果は")?;

    let pause = Duration::from_millis(500);
    for _ in 0..4 {
       print!(".");
       std::io::stdout().flush().unwrap();
       sleep(pause);
    }
    write!(out, " {}です!\n", get_omikuji())?;

    // アスキーアートを出力する
    let ascii_art = art.chars();
    let mut count = 0;
    for c in ascii_art {
        write!(out, "{}", c)?;
        if count % 100 == 0 {
            sleep(delay);
        }
        count += 1;
    }
    write!(out, "\n")?;

    // グローバルIPアドレスを取得する
    let ip_address = get_global_ip().await?;
    write!(out, "{}\n", ip_address)?;

    // // ロケーション情報を取得する
    let location = match get_location(ip_address).await {
        Ok(loc) => loc,
        Err(_) => {
            panic!("ロケーション情報を取得できませんでした。")
        }
    };

    // 天気情報を取得する
    match get_weather(&location).await {
        Ok(w) => 
            write!(out, "今日の天気は{}です。\n", w.current.weather_code)?,
        Err(_) => {
            write!(out, "天気情報を取得できませんでした。")?;
        }
    };
    
    // 近くの神社を取得する
    match get_shrine(&location).await {
        Ok(s) => 
            write!(out, "近くの神社は{}です。\n", s.elements[0].tags.name)?,
        Err(_) => {
            write!(out, "神社情報を取得できませんでした。")?;
        }
    };
    
    Ok(())
}