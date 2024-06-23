pub mod proto;

pub use proto::connect;
use proto::connect::{CreateObj, Join};

#[derive(Debug)]
pub enum ProtoRequest {
    Join(Join),
    CreateObj(CreateObj),
}
