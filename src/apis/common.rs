use std::{future::Future, time::Duration, net::SocketAddr, fmt::Debug};
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

pub enum ResponseMiddlewareData<'a> {
  Response(&'a reqwest::Response),
  Error(&'a reqwest::Error),
}

#[derive(Debug)]
struct ReqwestResponseParse {
  url: reqwest::Url,
  headers: reqwest::header::HeaderMap,
  status: reqwest::StatusCode,
  remote_addr: Option<SocketAddr>,
}

impl ReqwestResponseParse {
  fn parse(response: &reqwest::Response) -> Self {
    let url = response.url().clone();
    let headers = response.headers().clone();
    let status = response.status().clone();
    let remote_addr = response.remote_addr().clone();
    ReqwestResponseParse { url, headers, status, remote_addr }
  }

  fn url(&self) -> &reqwest::Url {
    &self.url
  }

  fn headers(&self) -> &reqwest::header::HeaderMap {
    &self.headers
  }

  fn status(&self) -> &reqwest::StatusCode {
    &self.status
  }

  fn remote_addr(&self) -> &Option<SocketAddr> {
    &self.remote_addr
  }
}

pub trait ResBucketTrait<T, E> 
where T: DeserializeOwned,
      E: DeserializeOwned,
{
  fn new(response: reqwest::Response) -> Self;

  fn response_middleware(response: ResponseMiddlewareData) -> impl Future {
    async fn func(res: ResponseMiddlewareData<'_>) {
      print!("response middleware in!! -> ");
      match res {
        ResponseMiddlewareData::Response(v) => {
          let response_parse = ReqwestResponseParse::parse(v);
          if v.status().is_server_error() || v.status().is_client_error() {
            on_error_response(response_parse).await;
          } else {
            on_success_response(response_parse).await;
          }
        },
        ResponseMiddlewareData::Error(e) => {
          on_error(e).await;
        },
      }    
    }
    func(response) 
  }

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
                // let _ = on_error_response(response_parse).await;
              },
              Err(_) => {
                error_body = Some(ResData::Text(body));
                // let _ = on_error_response(response_parse).await;
              },
            }
          },
          Err(err) => {
            error = Some(err);
            // on_error(&err).await;
          },
        }
      } else {
        match response.text().await {
          Ok(body) => {
            let struct_try = serde_json::from_str::<TT>(&body);
            match struct_try {
              Ok(t) => {
                success_body = Some(ResData::Struct(t));
                // let _ = on_success_response(response_parse).await;
              },
              Err(_) => {
                success_body = Some(ResData::Text(body));
                // let _ = on_success_response(response_parse).await;
              },
            }
          },
          Err(err) => {
            error = Some(err);
            // let _ = on_error(response_parse).await;
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
 
pub fn get_reqwest_client() -> Result<reqwest::Client, reqwest::Error> {
  let mut default_headers = reqwest::header::HeaderMap::new();
  default_headers.insert("X-MY-HEADER", reqwest::header::HeaderValue::from_static("value"));

  let client: Result<reqwest::Client, reqwest::Error> = reqwest::Client::builder()
    .user_agent("my-app/v0.0.1")
    .default_headers(default_headers)
    .timeout(Duration::from_secs(10))
    .build();

  client
}

async fn on_success_response(response_obj: ReqwestResponseParse) {
  let _ = format!("{:?},{:?},{:?}", response_obj.headers(), response_obj.status(), response_obj.remote_addr());

  println!("\non_success_response called!!!");
  println!("url {:?}", response_obj.url());
  // println!("headers {:?}", response_obj.headers());
  // println!("status {:?}", response_obj.status());
  // println!("remote_addr {:?}\n", response_obj.remote_addr());
}

async fn on_error_response(response_obj: ReqwestResponseParse) {
  println!("\non_error_response called!!!");
  println!("url {:?}", response_obj.url());
  // println!("headers {:?}", response_obj.headers());
  // println!("status {:?}", response_obj.status());
  // println!("remote_addr {:?}\n", response_obj.remote_addr());
}

async fn on_error(error: &reqwest::Error) {
  println!("\non_error called!!! : {:?}", error);
  // println!("headers {:?}", response_obj.headers());
  // println!("status {:?}", response_obj.status());
  // println!("remote_addr {:?}\n", response_obj.remote_addr());
}

#[derive(Debug)]
pub struct ReqBucket<'a, T> where T: Serialize + Deserialize<'a> + Debug {
  url: String,
  method: reqwest::Method,
  payload: Option<&'a T>,
}

impl<'a, T> ReqBucket<'a, T> where T: Serialize + Deserialize<'a> + Debug {
  pub fn new(url: String, method: reqwest::Method, payload: Option<&'a T>) -> Self {
    ReqBucket { url, method, payload }
  }

  pub async fn request_middleware(&self) {
    let _ = format!("{},{},{:?}", self.url(), self.method(), self.payload());
    println!("request try!! : {:?}", self.url());
  }

  pub fn url(&self) -> &str {
    self.url.as_str()
  }

  pub fn method(&self) -> &reqwest::Method {
    &self.method
  }

  pub fn payload(&self) -> &Option<&'a T> {
    &self.payload
  }
}