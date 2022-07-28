use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
#[clap(about = "M3U8 WebVTT 字幕流下载工具")]
struct Args {
    /// M3U8 字幕的 URL
    url: String,

    /// 向服务器提交的 HTTP 头
    #[clap(long, short)]
    user_agent: Option<String>,
}

struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    fn new(args: &Args) -> Downloader {
        let mut builder = reqwest::Client::builder();

        if let Some(ref user_agent) = args.user_agent {
            builder = builder.user_agent(user_agent);
        }

        let client = builder
            .pool_idle_timeout(Duration::from_secs(120))
            .pool_max_idle_per_host(1024)
            .default_headers()
            .build()
            .expect("reqwest builder 构建失败");

        Downloader { client }
    }

    async fn download_main_m3u8(&self, url: &str) -> Result<String, anyhow::Error> {
        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }
}

async fn async_main(args: Args) -> Result<(), anyhow::Error> {
    let downloader = Downloader::new(args.user_agent.as_deref());
    let main_m3u8 = downloader.download_main_m3u8(&args.url).await?;
    println!("{main_m3u8}");

    Ok(())
}

fn main() -> () {
    let args = Args::parse();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async_main(args)).unwrap();
}
