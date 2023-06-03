use tide::{Body, Request, Response, StatusCode};

use crate::State;

pub async fn hello_world(req: Request<State>) -> tide::Result {
    println!("Got file request from {}", req.peer_addr().unwrap());
    let mut response = Response::new(StatusCode::Ok);
    let file = req.state().file.clone();
    response.set_body(Body::from_file(&file).await?);
    response.insert_header(
        "Content-Disposition",
        format!("attachment; filename=\"{}\"", &file),
    );
    // Ok("Hello world!".into())
    Ok(response)
}
