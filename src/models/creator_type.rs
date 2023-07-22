use super::{SkinnyGroup, SkinnyUser};

#[derive(Debug, Clone)]
pub enum CreatorType {
    User(SkinnyUser),
    Group(SkinnyGroup),
}
