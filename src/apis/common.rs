use std::future::Future;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResStatusErrorPayload {
  timestamp: u32,
  reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  id: u32,
  email: String,
  first_name: String,
  last_name: String,
  avatar: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResStatusSuccessListPayload {
  pub page: u32,
  pub per_page: u32,
  pub total: u32,
  pub total_pages: u32,
  pub data: Vec<Data>,
}

pub enum ResParse<T, E> {
  SuccessBody(T),
  ErrorBody(E),
  Error(Option<reqwest::Error>),
}

pub enum ResData<T> {
  Struct(T),
  Text(String),
}

pub trait ResBucketTrait<T, E> 
where T: DeserializeOwned,
      E: DeserializeOwned,
{
  fn new(response: reqwest::Response) -> Self;

  // fn get_response(&self) -> reqwest::Response;

  fn parse<'a>(response: reqwest::Response) -> impl Future<Output = ResParse<ResData<T>, ResData<E>>> {

    async fn func<TT, EE>(response: reqwest::Response) -> ResParse<ResData<TT>, ResData<EE>> 
    where TT: DeserializeOwned,
          EE: DeserializeOwned,  
    {
      let mut success_body: Option<ResData<TT>> = None;
      let mut error_body: Option<ResData<EE>> = None;
      let mut error: Option<reqwest::Error> = None;

      if response.status().is_server_error() || response.status().is_client_error() {
        // let error_data = response.json::<ApiError>().await;
        match response.text().await {
          Ok(body) => {
            let struct_try = serde_json::from_str::<EE>(&body);
            match struct_try {
              Ok(t) => {
                error_body = Some(ResData::Struct(t));
              },
              Err(_) => {
                error_body = Some(ResData::Text(body));
              },
            }
          },
          Err(err) => {
            error = Some(err);
          },
        }
      } else {
        match response.text().await {
          Ok(body) => {
            let struct_try = serde_json::from_str::<TT>(&body);
            match struct_try {
              Ok(t) => {
                success_body = Some(ResData::Struct(t));
              },
              Err(_) => {
                success_body = Some(ResData::Text(body));
              },
            }
          },
          Err(err) => {
            error = Some(err);
          },
        }
      }
    
      if let Some(x) = success_body {
        return ResParse::SuccessBody(x);
      }
      if let Some(x) = error_body {
        return ResParse::ErrorBody(x);
      }
      ResParse::Error(error)
    }

    func::<T, E>(response)
  }
}
