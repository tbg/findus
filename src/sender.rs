use call;

pub trait Sender {
    fn send(&mut self, &mut call::Call<Box<::call::Request>, Box<::call::Response>>);
}


