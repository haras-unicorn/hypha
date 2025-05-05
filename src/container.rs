use crate::{item::HyphaItem, r#ref::HyphaRef};

pub trait HyphaContainer {
  type Item: HyphaItem;
  type Ref: HyphaRef;

  fn items(&self) -> &Vec<Self::Item>;
  fn items_mut(&mut self) -> &mut Vec<Self::Item>;
}

#[allow(dead_code, reason = "Future use.")]
pub trait HyphaContainerOps {
  fn swap(&mut self, left: &str, right: &str) -> bool;
}

impl<
    Item: HyphaItem,
    Ref: HyphaRef,
    Container: HyphaContainer<Item = Item, Ref = Ref>,
  > HyphaContainerOps for Container
{
  fn swap(&mut self, left: &str, right: &str) -> bool {
    let left_idx = match self
      .items()
      .iter()
      .enumerate()
      .find(|(_, item)| item.title() == left)
    {
      Some((idx, _)) => idx,
      None => {
        return false;
      }
    };
    let right_idx = match self
      .items()
      .iter()
      .enumerate()
      .find(|(_, item)| item.title() == right)
    {
      Some((idx, _)) => idx,
      None => {
        return false;
      }
    };
    self.items_mut().swap(left_idx, right_idx);
    true
  }
}
