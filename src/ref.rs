use serde::{Deserialize, Serialize};

use crate::{
  board::HyphaBoard, container::HyphaContainer, file::HyphaFile,
  issue::HyphaIssue, item::HyphaItem, list::HyphaList,
};

pub trait HyphaRef {
  type Item: HyphaItem + std::fmt::Debug;
  type Container: HyphaContainer;

  fn get_item_from_container<'a>(
    &self,
    container: &'a Self::Container,
  ) -> Option<&'a Self::Item>;
  fn get_item_from_container_mut<'a>(
    &self,
    container: &'a mut Self::Container,
  ) -> Option<&'a mut Self::Item>;
  fn remove_item_from_container(&self, container: &mut Self::Container)
    -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WithHyphaRef<Item: HyphaItem, Ref: HyphaRef> {
  pub item: Item,
  pub r#ref: Ref,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyphaBoardIssueRef {
  pub issue: String,
  pub list: String,
  pub stage: usize,
}

impl HyphaRef for HyphaBoardIssueRef {
  type Item = HyphaIssue;
  type Container = HyphaBoard;

  fn get_item_from_container<'a>(
    &self,
    container: &'a Self::Container,
  ) -> Option<&'a Self::Item> {
    container
      .lists
      .iter()
      .find(|list| list.title == self.list)
      .and_then(|list| {
        list.issues.iter().find(|issue| issue.title == self.issue)
      })
  }

  fn get_item_from_container_mut<'a>(
    &self,
    container: &'a mut Self::Container,
  ) -> Option<&'a mut Self::Item> {
    container
      .lists
      .iter_mut()
      .find(|list| list.title == self.list)
      .and_then(|list| {
        list
          .issues
          .iter_mut()
          .find(|issue| issue.title == self.issue)
      })
  }

  fn remove_item_from_container(
    &self,
    container: &mut Self::Container,
  ) -> bool {
    let list = match container
      .lists
      .iter_mut()
      .find(|list| list.title == self.list)
    {
      Some(list) => list,
      None => return false,
    };

    if let Some((idx, _)) = list
      .issues
      .iter()
      .enumerate()
      .find(|(_, issue)| issue.title == self.issue)
    {
      list.issues.remove(idx);
      return true;
    }

    false
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyphaFileIssueRef {
  pub issue: String,
  pub list: String,
  pub stage: usize,
  pub board: String,
}

impl HyphaRef for HyphaFileIssueRef {
  type Item = HyphaIssue;
  type Container = HyphaFile;

  fn get_item_from_container<'a>(
    &self,
    container: &'a Self::Container,
  ) -> Option<&'a Self::Item> {
    container
      .boards
      .iter()
      .find(|board| board.title == self.board)
      .and_then(|board| {
        board
          .lists
          .iter()
          .find(|list| list.title == self.list)
          .map(|list| {
            list.issues.iter().find(|issue| issue.title == self.issue)
          })
      })
      .flatten()
  }

  fn get_item_from_container_mut<'a>(
    &self,
    container: &'a mut Self::Container,
  ) -> Option<&'a mut Self::Item> {
    container
      .boards
      .iter_mut()
      .find(|board| board.title == self.board)
      .and_then(|board| {
        board
          .lists
          .iter_mut()
          .find(|list| list.title == self.list)
          .and_then(|list| {
            if list.issues.iter().any(|issue| issue.title == self.issue) {
              list
                .issues
                .iter_mut()
                .find(|issue| issue.title == self.issue)
            } else {
              list.issues.push(HyphaIssue::default());
              list.issues.last_mut()
            }
          })
      })
  }

  fn remove_item_from_container(
    &self,
    container: &mut Self::Container,
  ) -> bool {
    let board = match container
      .boards
      .iter_mut()
      .find(|board| board.title == self.board)
    {
      Some(board) => board,
      None => return false,
    };

    let list = match board.lists.iter_mut().find(|list| list.title == self.list)
    {
      Some(list) => list,
      None => return false,
    };

    if let Some((idx, _)) = list
      .issues
      .iter()
      .enumerate()
      .find(|(_, issue)| issue.title == self.issue)
    {
      list.issues.remove(idx);
      return true;
    }

    false
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyphaFileListRef {
  pub list: String,
  pub stage: usize,
  pub board: String,
}

impl HyphaRef for HyphaFileListRef {
  type Item = HyphaList;
  type Container = HyphaFile;

  fn get_item_from_container<'a>(
    &self,
    container: &'a Self::Container,
  ) -> Option<&'a Self::Item> {
    container
      .boards
      .iter()
      .find(|board| board.title == self.board)
      .and_then(|board| board.lists.get(self.stage))
  }

  fn get_item_from_container_mut<'a>(
    &self,
    container: &'a mut Self::Container,
  ) -> Option<&'a mut Self::Item> {
    container
      .boards
      .iter_mut()
      .find(|board| board.title == self.board)
      .and_then(|board| board.lists.get_mut(self.stage))
  }

  fn remove_item_from_container(
    &self,
    container: &mut Self::Container,
  ) -> bool {
    let board = match container
      .boards
      .iter_mut()
      .find(|board| board.title == self.board)
    {
      Some(board) => board,
      None => return false,
    };

    if let Some((idx, _)) = board
      .lists
      .iter()
      .enumerate()
      .find(|(_, list)| list.title == self.list)
    {
      board.lists.remove(idx);
      return true;
    }

    false
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyphaFileBoardRef {
  pub board: String,
}

impl HyphaRef for HyphaFileBoardRef {
  type Item = HyphaBoard;
  type Container = HyphaFile;

  fn get_item_from_container<'a>(
    &self,
    container: &'a Self::Container,
  ) -> Option<&'a Self::Item> {
    container
      .boards
      .iter()
      .find(|board| board.title == self.board)
  }

  fn get_item_from_container_mut<'a>(
    &self,
    container: &'a mut Self::Container,
  ) -> Option<&'a mut Self::Item> {
    container
      .boards
      .iter_mut()
      .find(|board| board.title == self.board)
  }

  fn remove_item_from_container(
    &self,
    container: &mut Self::Container,
  ) -> bool {
    if let Some((idx, _)) = container
      .boards
      .iter()
      .enumerate()
      .find(|(_, board)| board.title == self.board)
    {
      container.boards.remove(idx);
      return true;
    }

    false
  }
}
