use dioxus::prelude::*;
use dioxus_free_icons::{
  icons::fa_regular_icons::{FaCircleXmark, FaEye, FaPenToSquare},
  Icon,
};
use dioxus_markdown::Markdown;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

use crate::{
  context::{HyphaFileContext, HyphaIssueContext},
  hooks::{use_autofocus, use_autogrow},
  item::HyphaItem,
  r#ref::{HyphaFileIssueRef, HyphaRef, WithHyphaRef},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HyphaIssue {
  pub title: String,
  #[serde(default)]
  pub description: String,
}

impl HyphaItem for HyphaIssue {
  fn title(&self) -> &str {
    &self.title
  }
}

impl Default for HyphaIssue {
  fn default() -> Self {
    let rng = RNG::from(&Language::Roman);
    let title = rng.generate_name();
    HyphaIssue {
      title,
      description: String::new(),
    }
  }
}

#[component]
pub fn Component(issue_ref: HyphaFileIssueRef) -> Element {
  let mut file_context = use_context::<HyphaFileContext>();
  let mut issue_context = use_context::<HyphaIssueContext>();
  let mut edit = use_signal(|| false);

  let file = use_memo(move || file_context.get());
  let issue = {
    let issue_ref = issue_ref.clone();
    use_memo(move || issue_ref.get_item_from_container(&file()).cloned())
  };

  let issue_description_id = "issue-description";
  let mut issue_description = use_signal(String::new);
  use_effect(move || {
    if let Some(issue) = issue() {
      issue_description.set(issue.description);
    }
  });
  use_autofocus(edit, &issue_description_id);
  use_autogrow(edit, &issue_description_id);

  let issue = match issue() {
    Some(issue) => issue.clone(),
    None => {
      return rsx! {
        p {
          class: "error",
          "Error getting issue from hypha file: {issue_ref:?}"
        }
      }
    }
  };

  rsx! {
    div {
      class: "w-full flex flex-row justify-between mb-4",
      if edit() {
        button {
          class: "flex flex-row justify-center",
          onclick: move |_| {
            *edit.write() = false;
          },
          Icon {
            class: "mr-2 text-cyan-500 mt-[4px]",
            width: 20,
            height: 20,
            icon: FaEye
          }
          span {
            "Preview issue"
          }
        }
      }
      else {
        button {
          class: "flex flex-row justify-center",
          onclick: move |_| {
            *edit.write() = true;
          },
          Icon {
            class: "mr-2 text-cyan-500 mt-[4px]",
            width: 20,
            height: 20,
            icon: FaPenToSquare
          }
          span {
            "Edit issue"
          }
        }
      }
      button {
        onclick: move |_| {
          issue_context.set(None);
        },
        Icon {
          class: "text-grey-500 mt-[5px]",
          width: 20,
          height: 20,
          icon: FaCircleXmark
        }
      }
    }
    if edit() {
      h3 {
        class: "h-12 mb-2",
        input {
          class: "h-full w-full",
          value: issue.title.clone(),
          oninput: {
            let issue_ref = issue_ref.clone();
            let value = issue.clone();
            move |e: Event<FormData>| {
              let mut new_issue_ref = issue_ref.clone();
              new_issue_ref.issue = e.value();
              issue_context.set(Some(new_issue_ref));

              let mut value = value.clone();
              value.title = e.value();
              file_context.update_issue(WithHyphaRef {
                item: value,
                r#ref: issue_ref.clone()
              });
            }
          },
        }
      }
      div {
        class: "w-full h-px bg-zinc-500 mb-4"
      }
      div {
        class: "flex flex-row mb-4",
        code {
          class: "w-[50%]",
          textarea {
            id: issue_description_id,
            class: "w-full resize-none",
            value: issue.description.clone(),
            oninput: {
              let issue_ref = issue_ref.clone();
              let value = issue.clone();
              move |e: Event<FormData>| {
                let mut value = value.clone();
                value.description = e.value();
                file_context.update_issue(WithHyphaRef {
                  item: value,
                  r#ref: issue_ref.clone()
                });
              }
            }
          }
        }
        div {
          class: "h-full w-8"
        }
        p {
          class: "w-[50%] text-wrap break-word",
          Markdown {
            src: issue.description.clone()
          }
        }
      }
    } else {
      h3 {
        class: "h-12 mb-2 truncate",
        {issue.title}
      }
      div {
        class: "w-full h-px bg-zinc-500 mb-4"
      }
      p {
        class: "w-full mb-4 text-wrap break-word",
        Markdown {
          src: issue.description.clone()
        }
      }
    }
  }
}
