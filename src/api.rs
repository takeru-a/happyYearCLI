use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Serialize, Deserialize, Debug)]

// ロケーション情報
pub struct Location {
    pub status: String,
    #[serde(alias = "regionName")]
    pub region_name: String,
    pub city: String,
    pub lat: f64,
    pub lon: f64,
}

// 天気情報
#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub current: CurrentWeather,
    pub daily: DailyWeather,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature_2m: f32,
    pub weather_code: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyWeather {
    pub time: Vec<String>,
    pub weather_code: Vec<u32>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
}

// 神社情報
#[derive(Serialize, Deserialize, Debug)]
pub struct Shrine {
    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub tags: Tags,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tags {
    #[serde(default = "none_name")]
    pub name: String,
}

// デフォルト値を提供する関数
fn none_name() -> String {
    "none".to_string()
}

// グローバルIPからロケーション情報を取得する
pub async fn get_location(ip_address: String) -> Result<Location, reqwest::Error> {
    let url = format!("http://ip-api.com/json/{}?fields=16600", ip_address);
    let response = reqwest::get(&url).await?.json::<Location>().await?;
    Ok(response)
}

// ロケーション情報から天気情報を取得する
pub async fn get_weather(location: &Location) -> Result<Weather, reqwest::Error> {

    // 現在の天気と気温、今日の天気、最高気温と最低気温を取得する
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code&daily=weather_code,temperature_2m_max,temperature_2m_min&timezone=Asia%2FTokyo&forecast_days=1",
     location.lat, location.lon);
    let response = reqwest::get(&url).await?;
    let response = response.json::<Weather>().await?;
    Ok(response)
}

// ロケーション情報から近くの神社を取得する
pub async fn get_shrine(location: &Location) -> Result<Shrine, reqwest::Error> {

    // 現在地から5000m以内の神社を取得する
    let query = format!(r#"[out:json];node["amenity"="place_of_worship"]["religion"="shinto"](around:5000,{},{});out;"#,
    location.lat, location.lon);
    let url = format!("https://overpass-api.de/api/interpreter?data={}", query);
    let response = reqwest::get(&url).await?.json::<Shrine>().await?;
    Ok(response)
}