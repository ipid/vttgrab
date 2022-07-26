use anyhow::{Result, Error};

fn main() -> Result<()> {
    let fucking_string = String::from("Fuck you!");

    let request_baidu = async {
        let client = reqwest::Client::builder()
            .gzip(true)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36")
            .build()?;

        let response = client.get("https://www.baidu.com").send().await?;
        let text = response.text_with_charset("utf-8").await?;

        println!("{}", text);
        std::mem::drop(fucking_string);

        Result::<(), Error>::Ok(())
    };

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(request_baidu)?;

    println!("{}", fucking_string);

    Ok(())
}
