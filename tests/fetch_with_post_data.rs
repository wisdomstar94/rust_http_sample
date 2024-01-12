use rust_http_sample::apis::{self, common::{self, ResBucketTrait}};

#[tokio::test]
async fn fetch_with_post_data() {
  let req_payload = apis::user::login::ReqPayload {
    username: "mor_2314".to_string(),
    password: "83r5^_".to_string(),
  };
  let res = apis::user::login::fetch(&req_payload).await;
  if let Ok(res_bucket) = res {
    println!("api headers : {:?}", res_bucket.response.headers());
    println!("api status code : {:?}", res_bucket.response.status().as_u16());
    match apis::user::login::ResBucket::parse(res_bucket.response).await {
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
