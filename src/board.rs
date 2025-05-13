use dioxus::prelude::*;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

use crate::container::HyphaContainer;
use crate::context::{HyphaBoardContext, HyphaFileContext, HyphaIssueContext};
use crate::dep::HyphaDep;
use crate::issue;
use crate::item::HyphaItem;
use crate::list::HyphaList;
use crate::r#ref::{
  HyphaBoardIssueRef, HyphaFileBoardRef, HyphaFileIssueRef, HyphaFileListRef,
  HyphaRef, WithHyphaRef,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HyphaBoard {
  pub title: String,
  #[serde(default)]
  pub lists: Vec<HyphaList>,
  #[serde(default)]
  pub deps: Vec<HyphaDep<HyphaBoardIssueRef>>,
}

impl HyphaItem for HyphaBoard {
  fn title(&self) -> &str {
    &self.title
  }
}

impl HyphaContainer for HyphaBoard {
  type Item = HyphaList;
  type Ref = HyphaFileIssueRef;

  fn items(&self) -> &Vec<Self::Item> {
    &self.lists
  }

  fn items_mut(&mut self) -> &mut Vec<Self::Item> {
    &mut self.lists
  }
}

impl Default for HyphaBoard {
  fn default() -> Self {
    let rng = RNG::from(&Language::Roman);
    let title = rng.generate_name();
    Self {
      title,
      lists: vec![
        HyphaList::default(),
        HyphaList::default(),
        HyphaList::default(),
      ],
      deps: vec![],
    }
  }
}

#[component]
pub fn Component(board_ref: HyphaFileBoardRef) -> Element {
  let mut file_context = use_context::<HyphaFileContext>();
  let mut board_context = use_context::<HyphaBoardContext>();
  let mut issue_context = use_context::<HyphaIssueContext>();
  let mut edit = use_signal(|| false);

  let board = match board_ref.get_item_from_container(&file_context.get()) {
    Some(board) => board.clone(),
    None => {
      return rsx! {
        p {
          class: "error",
          "Board {board_ref:?} not found"
        }
      };
    }
  };

  rsx! {
    div {
      class: "w-full flex flex-row justify-center items-start",
      if edit() {
        div {
          class: "w-full flex justify-center",
          h2 {
            input {
              class: "w-full text-center",
              value: board.title.clone(),
              oninput: {
                let board_ref = board_ref.clone();
                let board = board.clone();
                move |e: Event<FormData>| {
                  let mut new_board_ref = board_ref.clone();
                  new_board_ref.board = e.value();
                  board_context.set(new_board_ref.clone());

                  if let Some(mut new_issue_ref) = issue_context.get() {
                    new_issue_ref.board = new_board_ref.board;
                    issue_context.set(Some(new_issue_ref));
                  }

                  let mut board = board.clone();
                  board.title = e.value();
                  file_context.update_board(WithHyphaRef {
                    item: board,
                    r#ref: board_ref.clone()
                  });
                }
              }
            }
          }
          button {
            onclick: move |_| {
              *edit.write() = false;
            }
          }
        }
      } else {
        h2 {
          class: "text-center cursor-pointer",
          onclick: move |_| {
            *edit.write() = true;
          },
          {board.title.clone()}
        }
      }
    }
    div {
      class: "w-full flex flex-row justify-center items-start overflow-auto",
      for (idx, list) in board.lists.iter().enumerate() {
        div {
          class: "flex flex-col border-indigo-500 border min-w-64 p-2 m-2",
          crate::list::Component {
            list_ref: HyphaFileListRef {
              list: list.title.clone(),
              stage: idx,
              board: board.title.clone()
            }
          }
        }
        if idx == board.lists.len() - 1 {
          button {
            onclick: {
              let board_ref = board_ref.clone();
              move |_| {
                file_context.add_list(board_ref.clone());
              }
            },
            "Add"
          }
        }
      }
    }
  }
}
