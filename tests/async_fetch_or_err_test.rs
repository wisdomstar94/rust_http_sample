use rust_http_sample::apis::{api_1, api_2, api_3, common::{self, ResBucketTrait}};
use tokio::try_join;

#[tokio::test]
async fn async_fetch_or_err_test() {
  let api1_req_payload = api_1::ReqPayload {
    username: "mor_2314".to_string(),
    password: "83r5^_".to_string(),
  };
  let api2_req_payload = api_2::ReqPayload {
    page: 4,
  };
  if let Ok((api1, api2, api3)) = try_join!(api_1::fetch(&api1_req_payload), api_2::fetch(&api2_req_payload), api_3::fetch()) {
    // api1
    println!("api1 response header is {:?}", api1.response.headers());
    println!("api1 response status code is {:?}", api1.response.status().as_u16());
    match api_1::ResBucket::parse(api1.response).await {
      common::ResParse::SuccessBody(x) => {
        match x {
          common::ResData::Struct(s) => {
            println!("api1 success_body(struct) is {:?}", s);
          },
          common::ResData::Text(t) => {
            println!("api1 success_body(text) is {:?}", t);
          },
        }
      },
      common::ResParse::ErrorBody(x) => {
        match x {
          common::ResData::Struct(s) => {
            println!("api1 error_body(struct) is {:?}", s);
          },
          common::ResData::Text(t) => {
            println!("api1 error_body(text) is {:?}", t);
          },
        }
      },
      common::ResParse::Error(x) => {
        if let Some(err) = x {
          println!("api1 error is {:?}", err);
        }
      },
    }

    // api2
    println!("api2 response header is {:?}", api2.response.headers());
    println!("api2 response status code is {:?}", api2.response.status().as_u16());
    match api_2::ResBucket::parse(api2.response).await {
      common::ResParse::SuccessBody(x) => {
        match x {
          common::ResData::Struct(s) => {
            println!("api2 success_body(struct) is {:?}", s);
          },
          common::ResData::Text(t) => {
            println!("api2 success_body(text) is {:?}", t);
          },
        }
      },
      common::ResParse::ErrorBody(x) => {
        match x {
          common::ResData::Struct(s) => {
            println!("api2 error_body(struct) is {:?}", s);
          },
          common::ResData::Text(t) => {
            println!("api2 error_body(text) is {:?}", t);
          },
        }
      },
      common::ResParse::Error(x) => {
        if let Some(err) = x {
          println!("api2 error is {:?}", err);
        }
      },
    }

    // api3
    println!("api3 response header is {:?}", api3.response.headers());
    println!("api3 response status code is {:?}", api3.response.status().as_u16());
    match api_3::ResBucket::parse(api3.response).await {
      common::ResParse::SuccessBody(x) => {
        match x {
          common::ResData::Struct(s) => {
            println!("api3 success_body(struct) is {:?}", s);
          },
          common::ResData::Text(t) => {
            println!("api3 success_body(text) is {:?}", t);
          },
        }
      },
      common::ResParse::ErrorBody(x) => {
        match x {
          common::ResData::Struct(s) => {
            println!("api3 error_body(struct) is {:?}", s);
          },
          common::ResData::Text(t) => {
            println!("api3 error_body(text) is {:?}", t);
          },
        }
      },
      common::ResParse::Error(x) => {
        if let Some(err) = x {
          println!("api3 error is {:?}", err);
        }
      },
    }
  }
}