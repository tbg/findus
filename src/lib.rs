#![allow(non_snake_case)]
extern crate protobuf;
extern crate hyper;

use std::env;
use std::io::Read;
use std::io;
use std::string::String;
use std::vec::Vec;

use protobuf::parse_from_bytes;
use protobuf::Message;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::mime::Mime;

use call::Request;

mod api;
mod config;
mod data;
mod errors;

mod call;


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
//
//impl api::GetRequest {
//    fn reply(&self) -> api::GetResponse {
//        api::GetResponse::new()
//    }
//}

#[test]
fn make_call() {
    let key = "COCKROACH_PORT";
    let addr = match env::var(key) {
        Ok(val) => val,
        Err(e)  => "tcp://localhost:8080".to_owned(),
    }.replace("tcp://", "http://");
    println!("http endpoint is {}", addr);

    let mut sender = HTTPSender::new(addr);

    let mut c = call::Call::put();

    {
        let args_header = c.args.mut_header();
        args_header.set_raft_id(1);
        args_header.set_user("root".to_owned());
        args_header.set_key(b"tkey".to_vec());
    }

    let e = sender.send(
        c
        //&putArgs,
        //&mut putResp,
    );

    println!("ts={}", call.reply.get_header().get_timestamp().get_wall_time());
    println!("error={}", call.reply.get_header().get_error().get_message());
}

pub enum SendError {
    IoError(io::Error),
    WireError(String),
}

struct HTTPSender {
    client: Client,
    addr: String,
}

impl HTTPSender {
    pub fn new(addr: String) -> HTTPSender {
        HTTPSender{
            client: hyper::Client::new(),
            addr: addr,
        }
    }
}

trait KVSender {
    //fn send(&mut self, &call::Request) -> Box<call::Response>;
    fn send(&mut self, &mut call::Call);
}

impl KVSender for HTTPSender {
    fn send(&mut self, c: &mut call::Call) {
        let enc = c.args.write_to_bytes().unwrap();
        let reply = &mut c.reply;

        let mut headers = Headers::new();
        headers.set(ContentType("application/x-protobuf".parse().unwrap()));
        let res = self.client.post(&self.addr)
            .body(&*enc) // or &enc[..]
            .headers(headers)
            .send();
        match res {
            Ok(mut resp) => match resp.status {
                StatusCode::Ok => {
                    // TODO check unmarshal error.
                    reply.merge_from(&mut protobuf::CodedInputStream::new(&mut resp));
                },
                _ => {
                    let mut err = errors::Error::new();
                    err.set_message("error unmarshaling".to_owned());
                    reply.mut_header().set_error(err)
                }
            },
            _ => {
                let mut err = errors::Error::new();
                err.set_message("request error".to_owned());
                reply.mut_header().set_error(err)
            },
        }
    }
}
