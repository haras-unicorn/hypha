pub trait HyphaItem {
  fn title(&self) -> &str;
}

impl<Item: HyphaItem> HyphaItem for &Item {
  fn title(&self) -> &str {
    HyphaItem::title(*self)
  }
}

impl<Item: HyphaItem> HyphaItem for &mut Item {
  fn title(&self) -> &str {
    HyphaItem::title(*self)
  }
}
