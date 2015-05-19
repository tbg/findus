#![allow(non_snake_case)]
extern crate protobuf;
extern crate hyper;

use std::io::Read;
use std::string::String;
use std::vec::Vec;

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
    putArgs.mut_header().set_user("root".to_owned());
    putArgs.mut_header().set_key(b"tkey".to_vec());

    let mut sender = Client::new();

    let e = sender.send(
        &putArgs,
        &mut putResp,
    );

    println!("{}", putResp.get_header().get_timestamp().get_wall_time());
}

trait KVSender {
    fn send(&mut self, &Message, &mut Message);
}

impl KVSender for Client {
    fn send(&mut self, args: &Message, reply: &mut Message) {
        let enc = args.write_to_bytes().unwrap();
        //reply.merge_from_bytes(&enc);

        let mut headers = Headers::new();
        headers.set(ContentType("application/x-protobuf".parse().unwrap()));
        let res = self.post("http://localhost:8080/kv/db/Put")
            .body(&*enc) // or &enc[..]
            .headers(headers)
            .send();
        match res {
            Ok(mut resp) => match resp.status {
                StatusCode::Ok => {
                    let proto_res = reply.merge_from(&mut protobuf::CodedInputStream::new(&mut resp));
                },
                _ => {},
            },
            _ => {},
        }
    }
}
