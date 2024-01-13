use serde::{Serialize, Deserialize};
use crate::apis::common::{ReqBucket, ResponseMiddlewareData};

use super::super::common::{self, ResBucketTrait};

// 요청페이로드 규격 정의
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqPayload {
  pub username: String,
  pub password: String,
}

// 응답패이로드 규격 정의
// 응답패이로드 규격 정의 - 성공 응답에 대한 규격
#[derive(Debug, Serialize, Deserialize)]
pub struct ResStatusSuccessPayload {
  pub token: String,
}

// 응답패이로드 규격 정의 - 에러 응답에 대한 규격
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ResStatusErrorPayload {
//   timestamp: u32,
//   reason: String,
// }
pub type ResStatusErrorPayload = common::ResStatusErrorPayload;

// 응답 객체를 받아 parse 처리를 하는 struct 정의
// #[derive(ResBucketMacro)]
pub struct ResBucket {
  pub response: reqwest::Response,
}

impl ResBucketTrait<ResStatusSuccessPayload, ResStatusErrorPayload> for ResBucket {
  fn new(response: reqwest::Response) -> Self {
    ResBucket { response }
  }
}

// 본 api 에 대한 요청을 날리고 응답 데이터를 반환하는 함수 정의
pub async fn fetch(req_payload: &ReqPayload) -> Result<ResBucket, reqwest::Error> {
  let client = common::get_reqwest_client();
  if let Err(err) = client {
    return Err(err);
  }

  let req_bucket = ReqBucket::new(
    "https://fakestoreapi.com/auth/login".to_string(), 
    reqwest::Method::POST, 
    Some(req_payload)
  );
  let _ = req_bucket.request_middleware().await;
  let response_result = client.unwrap().request(req_bucket.method().clone(), req_bucket.url()).json(req_payload).send().await;
  if let Ok(response) = response_result {
    let _ = ResBucket::response_middleware(ResponseMiddlewareData::Response(&response)).await;
    return Ok(ResBucket::new(response));
  } 
  if let Err(err) = response_result {
    let _ = ResBucket::response_middleware(ResponseMiddlewareData::Error(&err)).await;
    return Err(err);
  }
  panic!("uncatch error..!");
}