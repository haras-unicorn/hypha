use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::{FaCircleXmark, FaSquarePlus};
use dioxus_free_icons::Icon;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

use crate::context::{HyphaFileContext, HyphaIssueContext};
use crate::hooks::use_hypha_autofocus;
use crate::issue::HyphaIssue;
use crate::item::HyphaItem;
use crate::r#ref::{
  HyphaFileIssueRef, HyphaFileListRef, HyphaRef, WithHyphaRef,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HyphaList {
  pub title: String,
  #[serde(default)]
  pub issues: Vec<HyphaIssue>,
}

impl HyphaItem for HyphaList {
  fn title(&self) -> &str {
    &self.title
  }
}

impl Default for HyphaList {
  fn default() -> Self {
    let rng = RNG::from(&Language::Roman);
    let title = rng.generate_name();
    HyphaList {
      title,
      issues: vec![HyphaIssue::default()],
    }
  }
}

#[component]
pub fn Component(list_ref: HyphaFileListRef) -> Element {
  let mut file_context = use_context::<HyphaFileContext>();
  let mut issue_context = use_context::<HyphaIssueContext>();
  let mut edit = use_signal(|| false);

  let list_title = format!("list-title-{}", list_ref.stage);
  use_hypha_autofocus(edit, &list_title);

  let file = file_context.get();
  let list = match list_ref.get_item_from_container(&file) {
    Some(list) => list.clone(),
    None => {
      return rsx! {
        p {
          class: "error",
          "List not found {list_ref:?}"
        }
      }
    }
  };
  let board_title = list_ref.board.clone();
  let stage = list_ref.stage;

  rsx! {
    if edit() {
      div {
        class: "flex flex-row pr-4 mb-1",
        h4 {
          class: "grow",
          input {
            id: list_title,
            value: list.title.clone(),
            onblur: move |_| {
              edit.set(false);
            },
            oninput: {
              let list_ref = list_ref.clone();
              let list = list.clone();
              move |e: Event<FormData>| {
                let mut list = list.clone();
                list.title = e.value();
                file_context.update_list(WithHyphaRef {
                  item: list,
                  r#ref: list_ref.clone()
                });
              }
            }
          }
        }
      }
    } else {
      div {
        class: "flex flex-row pr-4 mb-1",
        h4 {
          class: "cursor-pointer grow truncate",
          onclick: {
            move |_| {
              *edit.write() = true;
            }
          },
          {list.title.clone()}
        }
        span {
          class: "cursor-pointer ml-1",
          onclick: {
            let list_ref = list_ref.clone();
            move |_| {
              if let Some(issue_ref) = issue_context.get() {
                if issue_ref.list == list_ref.list {
                  issue_context.set(None);
                }
              }
              file_context.remove_list(list_ref.clone());
            }
          },
          Icon {
            class: "text-red-500 mt-[5px]",
            width: 20,
            height: 20,
            icon: FaCircleXmark
          }
        }
      }
    }
    div {
      class: "w-full h-px bg-zinc-500 my-2 -mt-1"
    }
    button {
      class: "flex flex-row justify-center",
      onclick: {
        let list_ref = list_ref.clone();
        move |_| {
          file_context.add_issue(list_ref.clone());
        }
      },
      Icon {
        class: "text-green-400",
        width: 24,
        height: 24,
        icon: FaSquarePlus
      }
    }
    div {
      class: "w-full h-px bg-zinc-500 my-2"
    }
    div {
      class: "max-h-80 overflow-auto",
      for issue in list.issues.clone().iter() {
        {
          rsx! {
            div {
              class: "flex flex-row pr-4 mb-2",
              strong {
                class: "grow cursor-pointer truncate",
                onclick: {
                  let issue_title = issue.title.clone();
                  let board_title = board_title.clone();
                  let list_title = list.title.clone();
                  move |_| {
                    issue_context.set(Some(HyphaFileIssueRef {
                      issue: issue_title.clone(),
                      list: list_title.clone(),
                      stage,
                      board: board_title.clone()
                    }));
                  }
                },
                {issue.title.clone()}
              }
              span {
                class: "cursor-pointer ml-1",
                onclick: {
                  let issue_title = issue.title.clone();
                  let board_title = board_title.clone();
                  let list_title = list.title.clone();
                  move |_| {
                    let r#ref = HyphaFileIssueRef {
                      issue: issue_title.clone(),
                      list: list_title.clone(),
                      stage,
                      board: board_title.clone()
                    };
                    file_context.remove_issue(r#ref);
                  }
                },
                Icon {
                  class: "text-red-500 mt-[5px]",
                  width: 20,
                  height: 20,
                  icon: FaCircleXmark
                }
              }
            }
          }
        }
      }
    }
  }
}
