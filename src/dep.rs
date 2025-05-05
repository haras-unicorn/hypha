use serde::{Deserialize, Serialize};

use crate::r#ref::HyphaRef;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyphaDep<Ref: HyphaRef> {
  pub left: Ref,
  pub right: Ref,
}
