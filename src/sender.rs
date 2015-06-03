use call;

pub trait Sender {
    fn send(&mut self, &mut call::Call);
}


