use std::collections::HashMap;

#[tokio::test]
async fn basic_test() {
  let res = call().await;
  println!("res is {:#?}", res);
}

async fn call() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://httpbin.org/ip")
    .await?
    .json::<HashMap<String, String>>()
    .await?;
  Ok(resp)
}
