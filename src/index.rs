use dioxus::prelude::*;

use crate::{
  board,
  context::{HyphaBoardContext, HyphaIssueContext},
  issue,
};

#[component]
pub fn Index() -> Element {
  let board_context = use_context::<HyphaBoardContext>();
  let issue_context = use_context::<HyphaIssueContext>();

  let board_ref = board_context.get();
  rsx! {
    board::Component { board_ref: board_ref }
    if let Some(issue_ref) = issue_context.get() {
      issue::Component { issue_ref: issue_ref }
    }
  }
}
