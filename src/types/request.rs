use serde::Serialize;

#[derive(Serialize, Debug, RustcEncodable)]
pub struct SessionGet {
    method: String
}

impl Default for SessionGet{
    fn default() -> SessionGet {
        SessionGet { method: String::from("session-get") }
   }
}