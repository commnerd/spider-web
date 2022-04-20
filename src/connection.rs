use crate::pipe::Pipe;

#[derive(Default)]
pub struct Connection {
    pipe: std::vec::Vec<Pipe>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection{
            pipe: vec![],
        }
    }
}