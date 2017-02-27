extern crate example_service_search;
extern crate rs_es;

use rs_es::operations::search;
use rs_es::query::Query;
use rs_es::error;
use rs_es::Client;

fn make_client(url: &str) -> rs_es::Client {
    let hostname = url.to_owned();
    Client::new(&hostname).unwrap()
}

fn main() {
    let mut client = make_client("http://localhost:9200");

    let result_wrapped : Result<search::SearchResult<example_service_search::AddressDocument>,error::EsError> = client.search_query()
                   .with_indexes(&["address"])
                   .with_query(&Query::build_match("name", "gian").build())
                   .send();
    
    println!("TEST RESULT: {:?}", result_wrapped);
}