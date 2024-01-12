use rust_http_sample::apis::api_1;
use rust_http_sample::apis::common::{self, ResBucketTrait};

#[tokio::test]
async fn fetch_with_post_data() {
  let req_payload = api_1::ReqPayload {
    username: "mor_2314".to_string(),
    password: "83r5^_".to_string(),
  };
  let res = api_1::fetch(&req_payload).await;
  if let Ok(res_bucket) = res {
    println!("api headers : {:?}", res_bucket.response.headers());
    println!("api status code : {:?}", res_bucket.response.status().as_u16());
    match api_1::ResBucket::parse(res_bucket.response).await {
      common::ResParse::SuccessBody(x) => {
        match x {
          common::ResData::Struct(p) => {
            println!("api success payload json : {:?}", p);
          },
          common::ResData::Text(p) => {
            println!("api success payload text : {:?}", p);
          },
        }
      },
      common::ResParse::ErrorBody(x) => {
        match x {
          common::ResData::Struct(p) => {
            println!("api error payload json : {:?}", p);
          },
          common::ResData::Text(p) => {
            println!("api error payload text : {:?}", p);
          },
        }
      },
      common::ResParse::Error(x) => {
        println!("api error : {:?}", x);
      },
    }
  }
}
