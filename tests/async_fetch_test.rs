use serde::{Serialize, Deserialize};
use tokio::try_join;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  id: u32,
  email: String,
  first_name: String,
  last_name: String,
  avatar: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBody {
  page: u32,
  per_page: u32,
  total: u32,
  total_pages: u32,
  data: Vec<Data>,
}

#[tokio::test]
async fn async_fetch_test() {
  let joined = try_join!(call1(), call2(), call3());
  if let Ok((res1, res2, res3)) = joined {
    // ...
    println!("res1 is {:?}", res1);
    println!("res2 is {:?}", res2);
    println!("res3 is {:?}", res3);
  }
}

async fn call1() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=1")
    .await?
    .json::<ResponseBody>()
    .await?;
  println!("call1 완료");
  Ok(resp)
}

async fn call2() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=2")
    .await?
    .json::<ResponseBody>()
    .await?;
  println!("call2 완료");
  Ok(resp)
}

async fn call3() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=3")
    .await?
    .json::<ResponseBody>()
    .await?;
  println!("call3 완료");
  Ok(resp)
}
