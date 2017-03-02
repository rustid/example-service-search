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
    let mut client = make_client("http://192.168.100.16:9200");

    let result_wrapped : Result<search::SearchResult<example_service_search::AddressDocument>,error::EsError> = client.search_query()
                   .with_indexes(&["address"])
                   .with_query(&Query::build_match("name", "gian").build())
                   .send();

    println!("TEST RESULT: {:?}", result_wrapped);

    let mut router = Router::new();

    let ok = |req: &mut Request| -> IronResult<Response> {
        // let ref name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("/");
        
        let result_wrapped : Result<search::SearchResult<example_service_search::AddressDocument>,error::EsError> = client.search_query()
                   .with_indexes(&["address"])
                   .with_query(&Query::build_match("name", "").build())
                   .send();
        
        let result = result_wrapped.unwrap();
        let search_result = result.hits.hits;
        println!("TEST RESULT: {:?}", result_wrapped);
        //let response = serde_json::to_string(&search_result).unwrap();
        Ok(Response::with((iron::status::Ok, "")))
    };

    router.get("/:name", ok, "ok");
    Iron::new(router).http("localhost:3000").unwrap();
}