mod ext;

use convert_case::{Case, Casing};
pub use ext::NodeExt;

pub fn to_snake_case(v: &str) -> String {
    v.to_case(Case::Snake)
}
