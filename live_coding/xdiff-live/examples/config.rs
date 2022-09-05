use anyhow::Result;
use xdiff_live::{LoadConfig, RequestConfig};

fn main() -> Result<()> {
    let content = include_str!("../fixtures/xreq_test.yml");
    let config = RequestConfig::from_yaml(content)?;

    println!("{:#?}", config);
    Ok(())
}
