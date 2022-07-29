use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Parser)]
#[clap(about = "M3U8 WebVTT 字幕流下载工具。")]
struct Args {
    /// M3U8 字幕的 URL
    url: String,

    /// 向服务器提交的 HTTP 头
    #[clap(short = 'h', long = "header")]
    headers: Vec<String>,
}

struct Downloader {
    client: reqwest::Client,
    url: String,
}

impl Downloader {
    fn new(args: Args) -> Downloader {
        let Args { url, headers } = args;

        let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(headers.len());
        headers.iter().for_each(|raw_header| {
            let (key, value) = raw_header.split_once(":").expect(
                "-h/--header 参数值有误。\n\
                    您输入的 HTTP 头应该是 Key: Value 的格式，但是您输入的参数中没有冒号。\n\
                    请您注意：\n\
                    \n\
                    · -h/--header 后面的参数值最好加上引号。\n\
                    · Windows 下可以使用双引号，Linux 下可以使用单引号。\n\
                    \n\
                    示例：\n\
                    ·   --header 'Content-Type: application/json'\n\
                    ",
            );

            let header_name: HeaderName = key.trim().try_into().expect("HTTP 头名称有误。");
            let header_value: HeaderValue = value.trim().try_into().expect("HTTP 头值有误。");

            header_map.append(header_name, header_value);
        });

        let client = reqwest::Client::builder()
            // .connect_timeout(Duration::from_secs(8))
            // .pool_idle_timeout(Duration::from_secs(120))
            // .pool_max_idle_per_host(1024)
            // .default_headers(header_map)
            .build()
            .expect("reqwest Client 构建失败。");

        Downloader { client, url }
    }

    async fn download_main_m3u8(&self) -> Result<String> {
        let response = self.client.get(&self.url).send().await?;
        println!("{} 连接成功，状态码：{}", self.url, response.status());
        Ok(response.text().await?)
    }
}

#[tokio::main]
async fn main() {
    let downloader = Downloader::new(Args::parse());
    let m3u8_text = downloader.download_main_m3u8().await.expect("下载 M3U8 失败。");
    println!("m3u8_text: <{}>", m3u8_text);
}
