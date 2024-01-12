use super::common::ApiStruct;
pub type Api3Body = Vec<String>;

pub struct Api3 {
  pub response: reqwest::Response,
}

impl ApiStruct<Api3Body, String> for Api3 {
  fn new(response: reqwest::Response) -> Self {
    Api3 { response }
  }
}