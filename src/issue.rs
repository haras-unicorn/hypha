use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

use crate::{
  context::{HyphaFileContext, HyphaIssueContext},
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

  let file = file_context.get();
  let issue = match issue_ref.get_item_from_container(&file) {
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
    if edit() {
      h3 {
        class: "h-5 mb-2",
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
        class: "w-full h-px bg-indigo-500 mb-4"
      }
      textarea {
        class: "w-full resize-none h-64 mb-4",
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
      p {
        button {
          onclick: move |_| {
            *edit.write() = false;
          },
          "Preview"
        }
      }
    } else {
      h3 {
        class: "h-5 mb-2",
        {issue.title}
      }
      div {
        class: "w-full h-px bg-indigo-500 mb-4"
      }
      p {
        class: "h-64 mb-4 overflow-auto",
        Markdown {
          src: issue.description.clone()
        }
      }
      p {
        button {
          onclick: move |_| {
            *edit.write() = true;
          },
          "Edit"
        }
      }
    }
    p {
      button {
        onclick: move |_| {
          issue_context.set(None);
        },
        "Cancel"
      }
    }
  }
}
