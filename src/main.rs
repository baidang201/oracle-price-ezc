use clap::Parser;
use ethers::prelude::U256;
use ethers::signers::LocalWallet;
use hex_literal::hex;
use rand::Rng;
use reqwest::header::USER_AGENT;
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use web3::contract::{Contract, Options};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Random dely in secs
    #[clap(short, long)]
    delay: Option<u64>,
    #[clap(short, long = "task_id")]
    task_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinBasePrice {
    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "assetByUuid")]
    asset_by_uuid: AssetByUuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetByUuid {
    #[serde(rename = "uuid")]
    uuid: String,

    #[serde(rename = "symbol")]
    symbol: String,

    #[serde(rename = "unitPriceScale")]
    unit_price_scale: i64,

    #[serde(rename = "latestQuote")]
    latest_quote: LatestQuote,

    #[serde(rename = "latestPercentChanges")]
    latest_percent_changes: LatestPercentChanges,

    #[serde(rename = "priceDataForHour")]
    price_data_for_hour: PriceDataFor,

    #[serde(rename = "priceDataForDay")]
    price_data_for_day: PriceDataFor,

    #[serde(rename = "priceDataForWeek")]
    price_data_for_week: PriceDataFor,

    #[serde(rename = "priceDataForMonth")]
    price_data_for_month: PriceDataFor,

    #[serde(rename = "priceDataForYear")]
    price_data_for_year: PriceDataFor,

    #[serde(rename = "priceDataForAll")]
    price_data_for_all: PriceDataFor,

    #[serde(rename = "id")]
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestPercentChanges {
    #[serde(rename = "hour")]
    hour: f64,

    #[serde(rename = "day")]
    day: f64,

    #[serde(rename = "week")]
    week: f64,

    #[serde(rename = "month")]
    month: f64,

    #[serde(rename = "year")]
    year: f64,

    #[serde(rename = "all")]
    all: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestQuote {
    #[serde(rename = "baseCurrency")]
    base_currency: String,

    #[serde(rename = "quoteCurrency")]
    quote_currency: String,

    #[serde(rename = "price")]
    price: String,

    #[serde(rename = "timestamp")]
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceDataFor {
    #[serde(rename = "yAxisScalingFactor")]
    y_axis_scaling_factor: i64,

    #[serde(rename = "quotes")]
    quotes: Vec<Quote>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    #[serde(rename = "price")]
    price: String,

    #[serde(rename = "timestamp")]
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinGeckoPrice {
    #[serde(rename = "stats")]
    stats: Vec<Vec<f64>>,

    #[serde(rename = "total_volumes")]
    total_volumes: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinWatch {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "coin")]
    coin: String,

    #[serde(rename = "data")]
    data: Vec<Datum>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Datum {
    #[serde(rename = "date")]
    date: i64,

    #[serde(rename = "rate")]
    rate: f64,

    #[serde(rename = "volume")]
    volume: i64,

    #[serde(rename = "cap")]
    cap: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KucoinPrice {
    #[serde(rename = "code")]
    code: String,

    #[serde(rename = "data")]
    data: KucoinData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KucoinData {
    #[serde(rename = "time")]
    time: i64,

    #[serde(rename = "sequence")]
    sequence: String,

    #[serde(rename = "price")]
    price: String,

    #[serde(rename = "size")]
    size: String,

    #[serde(rename = "bestBid")]
    best_bid: String,

    #[serde(rename = "bestBidSize")]
    best_bid_size: String,

    #[serde(rename = "bestAsk")]
    best_ask: String,

    #[serde(rename = "bestAskSize")]
    best_ask_size: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GateIOData {
    date: u128,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceAndCompleteRes {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "error")]
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteRes {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "error")]
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Cryptorank {
    #[serde(rename = "data")]
    data: CryptorankData,
}

#[derive(Serialize, Deserialize)]
pub struct CryptorankData {
    #[serde(rename = "price")]
    price: Price,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "BTC")]
    btc: f64,

    #[serde(rename = "ETH")]
    eth: f64,

    #[serde(rename = "USD")]
    usd: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Coingecko {
    #[serde(rename = "stats")]
    stats: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize)]
pub struct CoinMarketCap {
    #[serde(rename = "data")]
    data: CoinMarketCapData,
}

#[derive(Serialize, Deserialize)]
pub struct CoinMarketCapData {
    #[serde(rename = "points")]
    points: HashMap<u128, Point>,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "v")]
    v: Vec<f64>,
}

fn timestamp() -> u128 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

async fn get_gateio_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using gateio price");
    let now = timestamp() / 1000;
    let resp =
        reqwest::get(format!("https://www.gate.io/json_svr/query/?u=10&c=9349111&type=tvkline&symbol=dpr_usdt&from={}&to={}&interval=1800", now-24*60*60*1000, now))
            .await?
            .text()
            .await?;
    let prices = resp.split("\n").collect::<Vec<&str>>();
    let price = prices[prices.len() - 2].split(",").collect::<Vec<&str>>();
    Ok((price[4].parse::<f32>()? * 10e17).round() as u64)
}

async fn get_coinmarketcap_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using coinmarketcap price");
    let client = reqwest::Client::new();
    let resp = client
    .get("https://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail/chart?id=8894&range=1D")
    .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
    .send()
    .await?
    .text()
    .await?;
    let price: CoinMarketCap = serde_json::from_str(&resp)?;
    Ok(
        (price.data.points[price.data.points.keys().max().unwrap_or(&0_u128)].v[0] * 10e17).round()
            as u64,
    )
}

async fn get_coingecko_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using coingecko price");
    let resp = reqwest::get("https://www.coingecko.com/price_charts/14748/usd/24_hours.json")
        .await?
        .text()
        .await?;
    let price: Coingecko = serde_json::from_str(&resp)?;
    Ok((price.stats.last().ok_or("No data found")?[1] * 10e17).round() as u64)
}

async fn get_cryptorank_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using cryptorank price");
    let resp = reqwest::get("https://api.cryptorank.io/v0/coins/deeper-network?locale=en")
        .await?
        .text()
        .await?;
    let price: Cryptorank = serde_json::from_str(&resp)?;
    Ok((price.data.price.usd * 10e17).round() as u64)
}

async fn get_kucoin_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using kucoin price");
    let resp =
        reqwest::get("https://api.kucoin.com/api/v1/market/orderbook/level1?symbol=DPR-USDT")
            .await?
            .text()
            .await?;
    let price: KucoinPrice = serde_json::from_str(&resp)?;
    Ok((price.data.price.parse::<f32>()? * 10e17).round() as u64)
}

async fn get_livecoinwatch_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using coinwatch price");
    let now = timestamp();
    let resp = reqwest::get(format!("https://http-api.livecoinwatch.com/coins/history/range?coin=DPR&start={}&end={}&currency=USD",now-86400000,now))
        .await?
        .text()
        .await?;
    let price: CoinWatch = serde_json::from_str(&resp)?;
    Ok((price.data.last().unwrap().rate * 10e17).round() as u64)
}

async fn get_coin360_price() -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Using coin360 price");
    let now = timestamp() / 1000;
    let resp = reqwest::get(format!(
        "https://coin360.com/site-api/coins/deeper-network-dpr/graph?start={}&end={}",
        now - 60 * 60 * 1000,
        now
    ))
    .await?
    .text()
    .await?;
    let price: Vec<Vec<f64>> = serde_json::from_str(&resp)?;
    Ok((price[0][0] * 10e17).round() as u64)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .expect("Failed to init logger");
    if let Some(delay) = args.delay {
        let delay_secs = rand::thread_rng().gen_range(0..delay);
        log::info!("Random delay: {}", delay_secs);
        tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
    }

    // let resp = reqwest::get(format!(
    //     "http://host.docker.internal:8000/race?task_id={}",
    //     args.task_id.unwrap_or(0_u64)
    // ))
    // .await?
    // .text()
    // .await?;
    // let race_res: RaceAndCompleteRes = serde_json::from_str(&resp)?;
    // if !race_res.success {
    //     log::info!("Failed to race task, given up");
    //     std::process::exit(0);
    // }
    let now = timestamp();
    let p = match now % 7 {
        0 => get_livecoinwatch_price().await?,
        1 => get_kucoin_price().await?,
        // 2 => get_coingecko_price().await?,
        3 => get_cryptorank_price().await?,
        4 => get_coinmarketcap_price().await?,
        5 => get_gateio_price().await?,
        6 => get_coin360_price().await?,
        _ => get_gateio_price().await?,
    };
    log::info!("Got price: {}", p);
    let wallet = LocalWallet::decrypt_keystore("/eth.keystore", "VGPUmPKNtBzDvCJK")?;
    // let wallet = LocalWallet::decrypt_keystore("../web3d/key/eth.keystore", "VGPUmPKNtBzDvCJK")?;
    let transport = web3::transports::Http::new("https://mainnet-dev.deeper.network/rpc")?;
    let web3 = web3::Web3::new(transport);
    let eth = web3.eth();
    let contract = Contract::from_json(
        eth,
        hex!("862AF0CF4397C06B4B76288c97aBefdF3eD5F121").into(),
        include_bytes!("../abi.json"),
    )?;
    let reciept = contract
        .signed_call_with_confirmations(
            "setTokenPrice",
            (U256::from(p),),
            Options {
                gas: Some(140850_u64.into()),
                ..Options::default()
            },
            1,
            &SecretKey::from_slice(&wallet.signer().to_bytes()).unwrap(),
        )
        .await?;
    log::info!("{:?}", reciept);
    // let resp = reqwest::get(format!(
    //     "http://host.docker.internal:8000/complete?task_id={}",
    //     args.task_id.unwrap_or(0_u64)
    // ))
    // .await?
    // .text()
    // .await?;
    // let complete_res: RaceAndCompleteRes = serde_json::from_str(&resp)?;
    // log::info!("Complete task request result: {:?}", complete_res);
    Ok(())
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_price() {
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .expect("Failed to init logger");
        let price = get_gateio_price().await.unwrap();
        log::info!("Gate.io price {}", price);
        let price = get_cryptorank_price().await.unwrap();
        log::info!("Cryptorank price {}", price);
        let price = get_coingecko_price().await.unwrap();
        log::info!("Coingecko price {}", price);
        let price = get_coinmarketcap_price().await.unwrap();
        log::info!("CoinMarketCap price {}", price);
        let price = get_coin360_price().await.unwrap();
        log::info!("Coin360 price {}", price);
    }
}
