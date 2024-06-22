pub mod proto;

pub use proto::connect;
use proto::connect::{CreateObj, Join};

pub enum ProtoRequest {
    Join(Join),
    CreateObj(CreateObj),
}
