#[derive(Clone)]
pub struct Pipe<'pipe> {
    label: &'pipe str,
    local_port: u16,
    local_socket: &'pipe str,
    remote_port: u16,
    remote_socket: &'pipe str,
}