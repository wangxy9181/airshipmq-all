pub trait RemotingProcessor: Send + Sync + Sized {
    fn process(&mut self);
}