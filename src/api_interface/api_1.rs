use super::common::{self, ApiStruct};
pub type Api1Body = common::ListBody;

pub struct Api1 {
  pub response: reqwest::Response,
}

impl ApiStruct<Api1Body, String> for Api1 {
  fn new(response: reqwest::Response) -> Self {
    Api1 { response }
  }
}
