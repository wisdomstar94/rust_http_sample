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
    let api_parse = Api1::parse(api1.response).await;
    
    if let Some(success_body) = api_parse.get_success_body() {
      // status success 응답
      match success_body {
        common::BodyData::Struct(x) => {
          println!("api1 success_body(struct) is {:?}", x);
        },
        common::BodyData::Text(x) => {
          println!("api1 success_body(text) is {:?}", x);
        },
      }
    } else if let Some(error_body) = api_parse.get_error_body() {
      // status error 응답
      match error_body {
        common::BodyData::Struct(x) => {
          println!("api1 error_body(struct) is {:?}", x);
        },
        common::BodyData::Text(x) => {
          println!("api1 error_body(text) is {:?}", x);
        },
      }
    } else if let Some(error) = api_parse.get_error() {
      // reqwest error
      println!("api1 error is {:?}", error);
    }

    // api2
    println!("api2 response header is {:?}", api2.response.headers());
    println!("api2 response status code is {:?}", api2.response.status().as_u16());
    let api_parse = Api2::parse(api2.response).await;
    
    if let Some(success_body) = api_parse.get_success_body() {
      // status success 응답
      match success_body {
        common::BodyData::Struct(x) => {
          println!("api2 success_body(struct) is {:?}", x);
        },
        common::BodyData::Text(x) => {
          println!("api2 success_body(text) is {:?}", x);
        },
      }
    } else if let Some(error_body) = api_parse.get_error_body() {
      // status error 응답
      match error_body {
        common::BodyData::Struct(x) => {
          println!("api2 error_body(struct) is {:?}", x);
        },
        common::BodyData::Text(x) => {
          println!("api2 error_body(text) is {:?}", x);
        },
      }
    } else if let Some(error) = api_parse.get_error() {
      // reqwest error
      println!("api2 error is {:?}", error);
    }

    // api3
    println!("api3 response header is {:?}", api3.response.headers());
    println!("api3 response status code is {:?}", api3.response.status().as_u16());
    let api_parse = Api3::parse(api3.response).await;
    
    if let Some(success_body) = api_parse.get_success_body() {
      // status success 응답
      match success_body {
        common::BodyData::Struct(x) => {
          println!("api3 success_body(struct) is {:?}", x);
        },
        common::BodyData::Text(x) => {
          println!("api3 success_body(text) is {:?}", x);
        },
      }
    } else if let Some(error_body) = api_parse.get_error_body() {
      // status error 응답
      match error_body {
        common::BodyData::Struct(x) => {
          println!("api3 error_body(struct) is {:?}", x);
        },
        common::BodyData::Text(x) => {
          println!("api3 error_body(text) is {:?}", x);
        },
      }
    } else if let Some(error) = api_parse.get_error() {
      // reqwest error
      println!("api3 error is {:?}", error);
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