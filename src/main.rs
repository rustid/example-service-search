extern crate example_service_search;
extern crate rs_es;
extern crate iron;
extern crate router;
extern crate serde_json;

use rs_es::operations::search;
use rs_es::query::Query;
use rs_es::error;
use rs_es::Client;

use iron::prelude::*;
use iron::status;
use router::Router;

fn make_client(url: &str) -> rs_es::Client {
    let hostname = url.to_owned();
    Client::new(&hostname).unwrap()
}

fn searchAdress(keyword: &str) {
    unimplemented!()
}

fn main() {
    let mut client = make_client("http://localhost:9200");

    let result_wrapped : Result<search::SearchResult<example_service_search::AddressDocument>,error::EsError> = client.search_query()
                   .with_indexes(&["address"])
                   .with_query(&Query::build_match("name", "gian").build())
                   .send();

    println!("TEST RESULT: {:?}", result_wrapped);

    let mut router = Router::new();

    let ok = |_: &mut Request| -> IronResult<Response> {
        let doc = example_service_search::AddressDocument::new().with_name("Gio");
        let response = serde_json::to_string(&doc).unwrap();
        Ok(Response::with((iron::status::Ok, response.as_str())))
    };

    router.get("/", ok, "ok");
    Iron::new(router).http("localhost:3000").unwrap();
}