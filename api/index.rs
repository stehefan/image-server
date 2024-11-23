use std::env;
use opendal::services::VercelBlob;
use opendal::Operator;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let auth_token = env::var("BLOB_READ_WRITE_TOKEN")
        .expect("Provide BLOB_READ_WRITE_TOKEN for authentication against Blob-API");
    let builder = VercelBlob::default()
        .root("/")
        .token(&auth_token[..]);
    let operator = Operator::new(builder)?.finish();
    let vec = operator.list("/").await?;
    let mut files: Vec<String> = Vec::with_capacity(vec.len());

    for entry in vec {
        files.push(String::from(entry.path()));
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "There might be an image at some point",
                "files": files
            })
                .to_string()
                .into(),
        )?)
}