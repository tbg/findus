#![allow(non_snake_case)]
extern crate protobuf;
extern crate hyper;

use std::io::Read;
use std::string::String;

use protobuf::parse_from_bytes;
use protobuf::Message;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::mime::Mime;

mod api;
mod config;
mod data;
mod errors;

struct Call {
    args: Box<protobuf::Message>,
    reply: Box<protobuf::Message>,
}

trait Request {
    fn mut_header(&mut self) -> &mut api::RequestHeader;
}

//impl Request for api::PutRequest {
    // TODO:
    // macro_rules! impl_trait { ($($ty:ty),*) => { $(impl Trait for $ty { fn foo(&self) -> … { … self.get_header() … } })* } }
    // impl_trait!(api::PutRequest, ...)
//    fn mut_header(&mut self) -> &mut api::RequestHeader {
//        self.mut_header()
//    }
//}

impl api::PutRequest {
    fn reply(&self) -> api::PutResponse {
        api::PutResponse::new()
    }
}

fn main() {
    let mut putArgs = api::PutRequest::new();
    let mut putResp = putArgs.reply();
    putArgs.mut_header().set_raft_id(1);
    println!("result: {}", putResp.mut_header().has_error());
}
