use rust_http_sample::api_interface::api_2::Api2;
use rust_http_sample::api_interface::api_1::Api1;
use rust_http_sample::api_interface::api_3::Api3;
use rust_http_sample::api_interface::common::{self, ApiStruct};
use tokio::try_join;

#[tokio::test]
async fn async_call_or_err_test() {
  if let Ok((api1, api2, api3)) = try_join!(call_api1(), call_api2(), call_api3()) {
    // api1
    println!("api1 response header is {:?}", api1.response.headers());
    println!("api1 response status code is {:?}", api1.response.status().as_u16());
    match Api1::parse(api1.response).await {
      common::BodyParse::SuccessBody(x) => {
        match x {
          common::BodyData::Struct(s) => {
            println!("api1 success_body(struct) is {:?}", s);
          },
          common::BodyData::Text(t) => {
            println!("api1 success_body(text) is {:?}", t);
          },
        }
      },
      common::BodyParse::ErrorBody(x) => {
        match x {
          common::BodyData::Struct(s) => {
            println!("api1 error_body(struct) is {:?}", s);
          },
          common::BodyData::Text(t) => {
            println!("api1 error_body(text) is {:?}", t);
          },
        }
      },
      common::BodyParse::Error(x) => {
        if let Some(err) = x {
          println!("api1 error is {:?}", err);
        }
      },
    }

    // api2
    println!("api2 response header is {:?}", api2.response.headers());
    println!("api2 response status code is {:?}", api2.response.status().as_u16());
    match Api2::parse(api2.response).await {
      common::BodyParse::SuccessBody(x) => {
        match x {
          common::BodyData::Struct(s) => {
            println!("api2 success_body(struct) is {:?}", s);
          },
          common::BodyData::Text(t) => {
            println!("api2 success_body(text) is {:?}", t);
          },
        }
      },
      common::BodyParse::ErrorBody(x) => {
        match x {
          common::BodyData::Struct(s) => {
            println!("api2 error_body(struct) is {:?}", s);
          },
          common::BodyData::Text(t) => {
            println!("api2 error_body(text) is {:?}", t);
          },
        }
      },
      common::BodyParse::Error(x) => {
        if let Some(err) = x {
          println!("api2 error is {:?}", err);
        }
      },
    }

    // api3
    println!("api3 response header is {:?}", api3.response.headers());
    println!("api3 response status code is {:?}", api3.response.status().as_u16());
    match Api3::parse(api3.response).await {
      common::BodyParse::SuccessBody(x) => {
        match x {
          common::BodyData::Struct(s) => {
            println!("api3 success_body(struct) is {:?}", s);
          },
          common::BodyData::Text(t) => {
            println!("api3 success_body(text) is {:?}", t);
          },
        }
      },
      common::BodyParse::ErrorBody(x) => {
        match x {
          common::BodyData::Struct(s) => {
            println!("api3 error_body(struct) is {:?}", s);
          },
          common::BodyData::Text(t) => {
            println!("api3 error_body(text) is {:?}", t);
          },
        }
      },
      common::BodyParse::Error(x) => {
        if let Some(err) = x {
          println!("api3 error is {:?}", err);
        }
      },
    }
  }
}

async fn call_api1() -> Result<Api1, reqwest::Error> {
  let response_result: Result<reqwest::Response, reqwest::Error> = reqwest::get("https://fakestoreapi.com/auth/login").await;
  if let Ok(response) = response_result {
    println!("api1 complete");
    return Ok(Api1::new(response));
  } 
  if let Err(err) = response_result {
    println!("api1 error");
    return Err(err);
  }
  panic!("uncatch error..!");
}

async fn call_api2() -> Result<Api2, reqwest::Error> {
  let response_result: Result<reqwest::Response, reqwest::Error> = reqwest::get("https://reqres.in/api/users?page=1").await;
  if let Ok(response) = response_result {
    println!("api2 complete");
    return Ok(Api2::new(response));
  } 
  if let Err(err) = response_result {
    println!("api2 error");
    return Err(err);
  }
  panic!("uncatch error..!");
}

async fn call_api3() -> Result<Api3, reqwest::Error> {
  let response_result: Result<reqwest::Response, reqwest::Error> = reqwest::get("https://fakestoreapi.com/products/categories").await;
  if let Ok(response) = response_result {
    println!("api3 complete");
    return Ok(Api3::new(response));
  } 
  if let Err(err) = response_result {
    println!("api3 error");
    return Err(err);
  }
  panic!("uncatch error..!");
}