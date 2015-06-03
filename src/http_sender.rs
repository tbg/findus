extern crate std;
extern crate protobuf;
extern crate hyper;

use std::string::String;
use std::error::Error;
use std::fmt::Display;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::header::Headers;

use errors;
use call;
use sender;

#[cfg(test)]
mod http_sender_test {
    use std::env;
    #[test]
    fn make_call() {
        let key = "COCKROACH_PORT";
        let addr = match env::var(key) {
            Ok(val) => val,
            Err(_)  => "tcp://localhost:8080".to_owned(),
        }.replace("tcp://", "http://");
        println!("http endpoint is {}", addr);
    
        let mut sender = super::HTTPSender::new(addr);
    
        let mut c = ::call::Call::put();
    
        {
            let args_header = c.args.mut_header();
            args_header.set_raft_id(1);
            args_header.set_user("root".to_owned());
            args_header.set_key(b"tkey".to_vec());
        }
    
        sender.send(&mut c);
    
        println!("ts={}", c.reply.mut_header().get_timestamp().get_wall_time());
        println!("error={}", c.reply.mut_header().get_error());
        assert!(!c.reply.mut_header().get_error().has_message());
    }
}

impl Display for errors::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.get_message().fmt(f)
    }
}

impl Error for errors::Error {
    fn description(&self) -> &str {
        return self.get_message();
    }
}

pub struct HTTPSender {
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
    pub fn send(&mut self, c: &mut call::Call) {
        let enc = c.args.write_to_bytes().unwrap();
        let reply = &mut c.reply;

        let mut headers = Headers::new();
        headers.set(ContentType("application/x-protobuf".parse().unwrap()));
        // TODO retry logic (on HTTP errors)
        let endpoint = self.addr.clone() + "/kv/db/" + c.args.method();
        let res = self.client.post(&endpoint)
            .body(&*enc) // or &enc[..]
            .headers(headers)
            .send();
        match res {
            Ok(mut resp) => match resp.status {
                StatusCode::Ok => {
                    match reply.merge_from(&mut protobuf::CodedInputStream::new(&mut resp)) {
                        Err(e) => {
                            let mut err = errors::Error::new();
                            err.set_message(e.description().to_owned());
                            reply.mut_header().set_error(err)
                        },
                        _ => {},
                    }
                },
                _ => {
                    let mut err = errors::Error::new();
                    err.set_message(format!("unexpected response status: {}",
                                            resp.status));
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

impl sender::Sender for HTTPSender {
    fn send(&mut self, c: &mut call::Call) {
        self.send(c)
    }
}
