use clap::Parser;

#[derive(Parser)]
#[clap(about = "M3U8 WebVTT 字幕流下载工具")]
struct Args {
    /// M3U8 字幕的 URL
    url: String,

    /// 向服务器提交的 User-Agent 字段
    #[clap(long)]
    user_agent: Option<String>,
}

struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    fn new(user_agent: Option<&str>) -> Downloader {
        let mut builder = reqwest::Client::builder();

        if let Some(user_agent) = user_agent {
            builder = builder.user_agent(user_agent);
        }

        Downloader {
            client: builder
                .build()
                .expect("reqwest builder 构建失败"),
        }
    }
}

async fn async_main(args: Args) -> Result<(), anyhow::Error> {
    let downloader = Downloader::new(args.user_agent.as_deref());
    

    Ok(())
}

fn main() {
    let args = Args::parse();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async_main(args)).unwrap();
}
