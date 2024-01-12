use super::common::{self, ApiStruct};
pub type Api2Body = common::ListBody;

pub struct Api2 {
  pub response: reqwest::Response,
}

impl ApiStruct<Api2Body, String> for Api2 {
  fn new(response: reqwest::Response) -> Self {
    Api2 { response }
  }
}
