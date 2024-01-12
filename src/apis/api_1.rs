use serde::{Serialize, Deserialize};
use super::common::{self, ResBucketTrait};

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
pub struct ResBucket {
  pub response: reqwest::Response,
}

// ResBucket 을 ResBucketTrait 으로 확장하여 parse 부분은 ResBucketTrait 에 구현된 기본 parse 함수 그대로 사용
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

  let response_result = client.unwrap().post("https://fakestoreapi.com/auth/login").json(req_payload).send().await;
  if let Ok(response) = response_result {
    println!("api 1 호출 완료");
    return Ok(ResBucket::new(response));
  } 
  if let Err(err) = response_result {
    return Err(err);
  }
  panic!("uncatch error..!");
}