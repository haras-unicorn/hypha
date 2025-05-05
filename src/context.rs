use dioxus::prelude::*;

use crate::board::HyphaBoard;
use crate::file::HyphaFile;
use crate::issue::HyphaIssue;
use crate::list::HyphaList;
use crate::r#ref::{
  HyphaFileBoardRef, HyphaFileIssueRef, HyphaFileListRef, HyphaRef,
  WithHyphaRef,
};

#[derive(Debug, Clone, Copy)]
pub struct HyphaFileContext {
  signal: Signal<HyphaFile>,
}

#[derive(Debug, Clone, Copy)]
pub struct HyphaBoardContext {
  signal: Signal<HyphaFileBoardRef>,
}

#[derive(Debug, Clone, Copy)]
pub struct HyphaIssueContext {
  signal: Signal<Option<HyphaFileIssueRef>>,
}

impl HyphaFileContext {
  pub fn new(signal: Signal<HyphaFile>) -> Self {
    Self { signal }
  }

  pub fn get(&self) -> HyphaFile {
    (self.signal)()
  }

  pub fn add_board(&mut self) {
    let board = HyphaBoard::default();
    let board_ref = HyphaFileBoardRef {
      board: board.title.clone(),
    };
    if board_ref.get_item_from_container(&self.get()).is_some() {
      return;
    }

    let mut writer = self.signal.write();
    writer.boards.push(board);
  }

  pub fn update_board(
    &mut self,
    board: WithHyphaRef<HyphaBoard, HyphaFileBoardRef>,
  ) {
    let board_ref = HyphaFileBoardRef {
      board: board.item.title.clone(),
    };
    if board_ref.get_item_from_container(&self.get()).is_some() {
      return;
    }

    let mut writer = self.signal.write();
    for dep in writer.deps.iter_mut() {
      if dep.left.board == board.r#ref.board {
        dep.left.board = board.item.title.clone();
      }
      if dep.right.board == board.r#ref.board {
        dep.right.board = board.item.title.clone();
      }
    }
    if let Some(container_board) =
      board.r#ref.get_item_from_container_mut(&mut writer)
    {
      *container_board = board.item;
    }
  }

  pub fn remove_board(&mut self, board_ref: HyphaFileBoardRef) {
    let mut writer = self.signal.write();
    let removed = board_ref.remove_item_from_container(&mut *writer);
    if removed {
      writer.deps.retain(|dep| {
        dep.left.board != board_ref.board && dep.right.board != board_ref.board
      });
    }
  }

  pub fn add_list(&mut self, board_ref: HyphaFileBoardRef) {
    let list = HyphaList::default();
    let list_ref = HyphaFileListRef {
      list: list.title.clone(),
      stage: 0,
      board: board_ref.board.clone(),
    };
    if list_ref.get_item_from_container(&self.get()).is_some() {
      return;
    }

    let mut writer = self.signal.write();
    if let Some(board) = board_ref.get_item_from_container_mut(&mut *writer) {
      board.lists.push(HyphaList::default());
    }
  }

  pub fn update_list(
    &mut self,
    list: WithHyphaRef<HyphaList, HyphaFileListRef>,
  ) {
    let list_ref = HyphaFileListRef {
      list: list.item.title.clone(),
      stage: 0,
      board: list.r#ref.board.clone(),
    };
    if list_ref.get_item_from_container(&self.get()).is_some() {
      return;
    }

    let mut writer = self.signal.write();
    for dep in writer.deps.iter_mut() {
      if dep.left.board == list.r#ref.board && dep.left.list == list.r#ref.list
      {
        dep.left.list = list.item.title.clone();
      }
      if dep.right.board == list.r#ref.board
        && dep.right.list == list.r#ref.list
      {
        dep.right.list = list.item.title.clone();
      }
    }

    let board_ref = HyphaFileBoardRef {
      board: list.r#ref.board.clone(),
    };
    if let Some(board) = board_ref.get_item_from_container_mut(&mut writer) {
      for dep in board.deps.iter_mut() {
        if dep.left.list == list.r#ref.list {
          dep.left.list = list.item.title.clone();
        }
        if dep.right.list == list.r#ref.list {
          dep.right.list = list.item.title.clone();
        }
      }
    }

    if let Some(container_list) =
      list.r#ref.get_item_from_container_mut(&mut writer)
    {
      *container_list = list.item;
    }
  }

  pub fn remove_list(&mut self, list_ref: HyphaFileListRef) {
    let mut writer = self.signal.write();
    let removed = list_ref.remove_item_from_container(&mut *writer);
    if removed {
      let board_ref = HyphaFileBoardRef {
        board: list_ref.board.clone(),
      };
      if let Some(board) = board_ref.get_item_from_container_mut(&mut *writer) {
        board.deps.retain(|dep| {
          dep.left.list != list_ref.list && dep.right.list != list_ref.list
        });
      }
      writer.deps.retain(|dep| {
        dep.left.list != list_ref.list && dep.right.list != list_ref.list
      });
    }
  }

  pub fn add_issue(&mut self, list_ref: HyphaFileListRef) {
    let issue = HyphaIssue::default();
    let issue_ref = HyphaFileIssueRef {
      issue: issue.title.clone(),
      list: list_ref.list.clone(),
      stage: 0,
      board: list_ref.board.clone(),
    };
    if issue_ref.get_item_from_container(&self.get()).is_some() {
      return;
    }

    let mut writer = self.signal.write();
    if let Some(list) = list_ref.get_item_from_container_mut(&mut *writer) {
      list.issues.push(issue);
    }
  }

  pub fn update_issue(
    &mut self,
    issue: WithHyphaRef<HyphaIssue, HyphaFileIssueRef>,
  ) {
    let issue_ref = HyphaFileIssueRef {
      issue: issue.item.title.clone(),
      list: issue.r#ref.list.clone(),
      stage: 0,
      board: issue.r#ref.board.clone(),
    };
    if issue_ref.get_item_from_container(&self.get()).is_some() {
      return;
    }

    let mut writer = self.signal.write();
    for dep in writer.deps.iter_mut() {
      if dep.left.board == issue.r#ref.board
        && dep.left.list == issue.r#ref.list
        && dep.left.issue == issue.r#ref.issue
      {
        dep.left.issue = issue.item.title.clone();
      }
      if dep.right.board == issue.r#ref.board
        && dep.right.list == issue.r#ref.list
        && dep.right.list == issue.r#ref.issue
      {
        dep.right.issue = issue.item.title.clone();
      }
    }
    let board_ref = HyphaFileBoardRef {
      board: issue.r#ref.board.clone(),
    };
    if let Some(board) = board_ref.get_item_from_container_mut(&mut writer) {
      for dep in board.deps.iter_mut() {
        if dep.left.list == issue.r#ref.list
          && dep.left.issue == issue.r#ref.issue
        {
          dep.left.issue = issue.item.title.clone();
        }
        if dep.right.list == issue.r#ref.list
          && dep.right.issue == issue.r#ref.issue
        {
          dep.right.issue = issue.item.title.clone();
        }
      }
    }
    if let Some(container_issue) =
      issue.r#ref.get_item_from_container_mut(&mut *writer)
    {
      *container_issue = issue.item;
    }
  }

  pub fn remove_issue(&mut self, issue_ref: HyphaFileIssueRef) {
    let mut writer = self.signal.write();
    let removed = issue_ref.remove_item_from_container(&mut *writer);
    if removed {
      let board_ref = HyphaFileBoardRef {
        board: issue_ref.board.clone(),
      };
      if let Some(board) = board_ref.get_item_from_container_mut(&mut *writer) {
        board.deps.retain(|dep| {
          dep.left.issue != issue_ref.issue
            && dep.right.issue != issue_ref.issue
        });
      }
      writer.deps.retain(|dep| {
        dep.left.issue != issue_ref.issue && dep.right.issue != issue_ref.issue
      });
    }
  }
}

impl HyphaBoardContext {
  pub fn new(signal: Signal<HyphaFileBoardRef>) -> Self {
    Self { signal }
  }

  pub fn get(&self) -> HyphaFileBoardRef {
    (self.signal)()
  }

  pub fn set(&mut self, board_ref: HyphaFileBoardRef) {
    *self.signal.write() = board_ref;
  }
}

impl HyphaIssueContext {
  pub fn new(signal: Signal<Option<HyphaFileIssueRef>>) -> Self {
    Self { signal }
  }

  pub fn get(&self) -> Option<HyphaFileIssueRef> {
    (self.signal)()
  }

  pub fn set(&mut self, issue_ref: Option<HyphaFileIssueRef>) {
    *self.signal.write() = issue_ref;
  }
}
