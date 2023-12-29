mod api;
mod tools;

use rand::seq::SliceRandom;
use rand::thread_rng;
use core::result::Result::Ok;
use anyhow::Result;
use api::{get_location, get_weather, get_shrine, Element};
use tools::{get_message, get_art, get_omikuji, get_global_ip, code_to_weather};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use chrono::Utc;
use chrono_tz::Asia::Tokyo;

#[derive(Parser)]
#[command(
    name = "Happy New Year Command",
    version = "1.0.0",
    author = "Takeru"
)]
struct Cli {

    // 出力する文章の種類を指定する(0~2)
    #[arg(
        short='t',
        default_value_t=0,
        help="出力する文章の種類を指定する(0~2)", 
        num_args=1, 
        value_parser=clap::value_parser!(u8).range(0..=2)
    )]
    message_type: u8,

    // アスキーアートを指定する(0~2)
    #[arg(
        short='a', 
        default_value_t=0, 
        help="アスキーアートを指定する(0~2)", 
        num_args=1, 
        value_parser = clap::value_parser!(u8).range(0..=2))]
    art: u8,
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Cli::parse();
    let message = get_message(args.message_type);
    let art = get_art(args.art);
    
    // 遅延時間を設定する
    let delay = Duration::from_millis(50);

    // 現在時刻を取得する
    let utc = Utc::now();
    let jst = utc.with_timezone(&Tokyo);

    // 標準出力
    let out = stdout();
    let mut out = out.lock();

    write!(out, "\n==============================================================================================================")?;

    write!(out, r#"
    _   _   ___  ______ ______ __   __  _   _  _____  _    _  __   __ _____   ___  ______
    | | | | / _ \ | ___ \| ___ \\ \ / / | \ | ||  ___|| |  | | \ \ / /|  ___| / _ \ | ___ \
    | |_| |/ /_\ \| |_/ /| |_/ / \ V /  |  \| || |__  | |  | |  \ V / | |__  / /_\ \| |_/ /
    |  _  ||  _  ||  __/ |  __/   \ /   | . ` ||  __| | |/\| |   \ /  |  __| |  _  ||    /
    | | | || | | || |    | |      | |   | |\  || |___ \  /\  /   | |  | |___ | | | || |\ \
    \_| |_/\_| |_/\_|    \_|      \_/   \_| \_/\____/  \/  \/    \_/  \____/ \_| |_/\_| \_|

"#
        )?;

    write!(out, "==============================================================================================================\n\n")?;
    sleep(Duration::from_secs(1));

    // 日付を出力する
    write!(out, "\t{}🎉\n", jst.format("%Y/%m/%d"))?;
    // メッセージを出力する
    write!(out, "{}\n", message)?;
    sleep(Duration::from_secs(1));

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
    write!(out, "\n\n")?;

    // おみくじの内容を出力する
    write!(out, "【おみくじ】❤️\n")?;
    write!(out, "おみくじで貴方の運勢を占います。\n")?;
    write!(out, "結果は")?;
    let pause = Duration::from_millis(700);
    for _ in 0..6 {
        print!(".");
        std::io::stdout().flush().unwrap();
        sleep(pause);
    }
    write!(out, " {}です!\n\n", get_omikuji())?;

    // グローバルIPアドレスを取得する
    let ip_address = get_global_ip().await?;

    // // ロケーション情報を取得する
    let location = match get_location(ip_address).await {
        Ok(loc) => loc,
        Err(_) => {
            panic!("ロケーション情報を取得できませんでした。\n")
        }
    };

    // 天気情報を取得する
    write!(out, "【気候】☀\n")?;
    match get_weather(&location).await {
        Ok(w) => 
            write!(out, "{}\n現在の気温{}℃, 最高気温{}℃, 最高低気温{}℃ 🌡️\n今日の天気は{}です!\n\n",
            location.region_name,
            w.current.temperature_2m, w.daily.temperature_2m_max[0], w.daily.temperature_2m_min[0],
            code_to_weather(w.current.weather_code))?,
        Err(_) => {
            write!(out, "天気情報を取得できませんでした。\n")?;
        }
    };
    
    // 近くの神社を取得する
    write!(out, "【神社】⛩\n")?;
    match get_shrine(&location).await {
        Ok(s) => {

            let mut rng = thread_rng();
            let mut el: Vec<_> = s.elements.iter()
                .filter(|e: &&Element| e.tags.name != "none")
                .map(|e| e.tags.name.as_str())
                .collect();
            el.shuffle(&mut rng);
            
            let result = el.iter().take(5).map(|e| *e).collect::<Vec<_>>().join(", ");

            if result.is_empty() {
                write!(out, "近くに神社はありません。\n")?;
            } else {
                write!(out, "近くには{}があります!\n", result)?;
            }
        },
        Err(_) => {
            write!(out, "神社情報を取得できませんでした。\n")?;
        }
    };

    Ok(())
}