use std::future::Future;

use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  id: u32,
  email: String,
  first_name: String,
  last_name: String,
  avatar: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBody {
  page: u32,
  per_page: u32,
  total: u32,
  total_pages: u32,
  data: Vec<Data>,
}

pub enum BodyParse<T, E> {
  SuccessBody(T),
  ErrorBody(E),
  Error(Option<reqwest::Error>),
}

pub enum BodyData<T> {
  Struct(T),
  Text(String),
}

pub trait ApiStruct<T, E> 
where T: DeserializeOwned,
      E: DeserializeOwned,
{
  fn new(response: reqwest::Response) -> Self;

  // fn get_response(&self) -> reqwest::Response;

  fn parse<'a>(response: reqwest::Response) -> impl Future<Output = BodyParse<BodyData<T>, BodyData<E>>> {

    async fn func<TT, EE>(response: reqwest::Response) -> BodyParse<BodyData<TT>, BodyData<EE>> 
    where TT: DeserializeOwned,
          EE: DeserializeOwned,  
    {
      let mut success_body: Option<BodyData<TT>> = None;
      let mut error_body: Option<BodyData<EE>> = None;
      let mut error: Option<reqwest::Error> = None;

      if response.status().is_server_error() || response.status().is_client_error() {
        // let error_data = response.json::<ApiError>().await;
        match response.text().await {
          Ok(body) => {
            let struct_try = serde_json::from_str::<EE>(&body);
            match struct_try {
              Ok(t) => {
                error_body = Some(BodyData::Struct(t));
              },
              Err(_) => {
                error_body = Some(BodyData::Text(body));
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
                success_body = Some(BodyData::Struct(t));
              },
              Err(_) => {
                success_body = Some(BodyData::Text(body));
              },
            }
          },
          Err(err) => {
            error = Some(err);
          },
        }
      }
    
      if let Some(x) = success_body {
        return BodyParse::SuccessBody(x);
      }
      if let Some(x) = error_body {
        return BodyParse::ErrorBody(x);
      }
      BodyParse::Error(error)
    }

    func::<T, E>(response)
    // let mut success_body: Option<BodyData<T>> = None;
    // let mut error_body: Option<BodyData<E>> = None;
    // let mut error: Option<reqwest::Error> = None;

    // if response.status().is_server_error() || response.status().is_client_error() {
    //   // let error_data = response.json::<ApiError>().await;
    //   match response.text().await {
    //     Ok(body) => {
    //       let struct_try = serde_json::from_str::<E>(&body);
    //       match struct_try {
    //         Ok(t) => {
    //           error_body = Some(BodyData::Struct(t));
    //         },
    //         Err(_) => {
    //           error_body = Some(BodyData::Text(body));
    //         },
    //       }
    //     },
    //     Err(err) => {
    //       error = Some(err);
    //     },
    //   }
    // } else {
    //   match response.text().await {
    //     Ok(body) => {
    //       let struct_try = serde_json::from_str::<T>(&body);
    //       match struct_try {
    //         Ok(t) => {
    //           success_body = Some(BodyData::Struct(t));
    //         },
    //         Err(_) => {
    //           success_body = Some(BodyData::Text(body));
    //         },
    //       }
    //     },
    //     Err(err) => {
    //       error = Some(err);
    //     },
    //   }
    // }
   
    // BodyParse::new(success_body, error_body, error)
  }

  // async fn parse<'a, T, E>(response: reqwest::Response) -> BodyParse<T, E> 
  // where T: DeserializeOwned,
  //       E: DeserializeOwned,
  // {
  //   let mut success_body: Option<T> = None;
  //   let mut error_body: Option<E> = None;
  //   let mut error: Option<reqwest::Error> = None;
  //   if response.status().is_server_error() || response.status().is_client_error() {
  //     // let error_data = response.json::<ApiError>().await;
  //     match response.json::<E>().await {
  //       Ok(body) => {
  //         error_body = Some(body);
  //       },
  //       Err(err) => {
  //         error = Some(err);
  //       },
  //     }
  //   } else {
  //     // let success_data = response.json::<ApiBody>().await;
  //     match response.json::<T>().await {
  //       Ok(body) => {
  //         success_body = Some(body);
  //       },
  //       Err(err) => {
  //         error = Some(err);
  //       },
  //     }
  //   }
  //   BodyParse::new(success_body, error_body, error)
  // }
}

pub trait TypeWrapper {
  fn check(&self) -> Option<Self>
  where
    Self: Sized;
}

impl TypeWrapper for String {
  fn check(&self) -> Option<String> {
    Some("String".to_string())
  }
}

pub fn type_check<U>(x: U) -> Option<U>
where
  U: TypeWrapper,
{
  x.check()
}