use crate::{Request, Requests};
use reqwest::blocking::Client;
use std::error::Error;

pub struct Reqwest {
    client: Client,
}

impl Default for Reqwest {
    fn default() -> Self {
        Self::new()
    }
}

impl Reqwest {
    pub fn new() -> Self {
        Reqwest {
            client: Client::new(),
        }
    }
}

impl Requests for Reqwest {
    fn send(
        &self,
        signed: Request,
    ) -> Result<(u16, String), Box<dyn Error>> {
        let resp = self
            .client
            .post(signed.uri().to_string())
            .headers(signed.headers().clone())
            .body(signed.body().clone())
            .send()?;
        let status = resp.status().as_u16();
        let body = resp.text()?;
        //println!("\nresp {} {}", status, body);
        Ok((status, body))
    }
}
