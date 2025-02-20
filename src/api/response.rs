use bytes::{Bytes, BytesMut};
use reqwest::Response;

pub struct ResponseHandler {}

impl ResponseHandler {
    // extract chunk bytes from the response
    pub async fn get_response_chunk(
        response: &mut Response,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        let mut res = BytesMut::new();
        while let Some(chunk) = response.chunk().await? {
            res.extend_from_slice(&chunk);
        }
        Ok(res.freeze())
    }
}
