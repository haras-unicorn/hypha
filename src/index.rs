use dioxus::prelude::*;

use crate::{
  board,
  context::{HyphaBoardContext, HyphaIssueContext},
  issue,
  resize::HyphaVerticalResize,
};

#[component]
pub fn Index() -> Element {
  let board_context = use_context::<HyphaBoardContext>();
  let issue_context = use_context::<HyphaIssueContext>();

  let board_ref = board_context.get();
  rsx! {
    HyphaVerticalResize {
      class: "mb-8 flex flex-col",
      min: "30vh",
      max: "70vh",
      board::Component { board_ref: board_ref }
    }
    if let Some(issue_ref) = issue_context.get() {
      div {
        class: "mt-8",
        issue::Component { issue_ref: issue_ref }
      }
    } else {
      div { class: "grow" }
    }
  }
}
