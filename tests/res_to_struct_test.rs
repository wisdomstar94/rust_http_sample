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
async fn res_to_struct_test() {
  let res = call().await;
  println!("res is {:#?}", res.unwrap());
}

async fn call() -> Result<ResponseBody, Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://reqres.in/api/users?page=2")
    .await?
    .json::<ResponseBody>()
    .await?;
  Ok(resp)
}
