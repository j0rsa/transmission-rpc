use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    arguments: T,
    result: String
}