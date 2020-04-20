use serde::Serialize;

#[derive(Serialize, Debug, RustcEncodable)]
pub struct SessionGet {
    method: String
}

impl SessionGet{
    pub fn new() -> SessionGet {
        SessionGet { method: String::from("session-get") }
   }
}


pub trait RpcRequestArgument {}
impl RpcRequestArgument for SessionGet{}