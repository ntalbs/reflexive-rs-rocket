use std::collections::BTreeMap;

use rocket::{Route, Request, Data, route};
use serde::{Serialize, ser::SerializeStruct};

struct EchoResponse {
    method: String,
    path: String,
    queries: BTreeMap<String, String>,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Serialize for EchoResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Echo", 5)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("queries", &self.queries)?;
        state.serialize_field("headers", &self.headers)?;
        state.serialize_field("body", &self.body)?;
        state.end()
    }
}

fn echo<'a>(req: &'a Request, _: Data<'a>) -> route::BoxFuture<'a> {

    let echo = EchoResponse {
        method: req.method().to_string(),
        path: req.uri().path().to_string(),
        queries: BTreeMap::new(),
        headers: BTreeMap::new(),
        body: "BODY".to_string(),
    };

    route::Outcome::from(req, serde_json::to_string(&echo).unwrap()).pin()
}

#[rocket::launch]
fn rocket() -> _ {
    use rocket::http::Method::*;
    let mut routes = vec![];
    for m in &[Get, Put, Post, Delete, Options, Head, Trace, Connect, Patch] {
        routes.push(Route::new(*m, "/<path..>", echo));
    }
    rocket::build().mount("/", routes)
}
