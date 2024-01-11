use serde::{Serialize, Deserialize};

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
async fn sync_call_test() {
  println!("req1 start");
  let res1 = call1().await;
  println!("res1 is {:#?}", res1.unwrap());

  println!("req2 start");
  let res2 = call2().await;
  println!("res2 is {:#?}", res2.unwrap());
  
  println!("req3 start");
  let res3 = call3().await;
  println!("res3 is {:#?}", res3.unwrap());
}

async fn call1() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=1")
    .await?
    .json::<ResponseBody>()
    .await?;
  Ok(resp)
}

async fn call2() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=2")
    .await?
    .json::<ResponseBody>()
    .await?;
  Ok(resp)
}

async fn call3() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=3")
    .await?
    .json::<ResponseBody>()
    .await?;
  Ok(resp)
}
