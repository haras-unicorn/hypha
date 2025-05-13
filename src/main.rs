#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

mod board;
mod container;
mod context;
mod dep;
mod file;
mod index;
mod issue;
mod item;
mod list;
mod provider;
mod r#ref;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("assets/favicon.ico");
const ROOT_CSS: Asset = asset!("assets/root.css");
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");
const NORMALIZE_CSS: Asset = asset!("assets/normalize.css");

fn main() {
  launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    head {
      link { rel: "icon", href: FAVICON }
      link { rel: "stylesheet", href: ROOT_CSS }
      link { rel: "stylesheet", href: TAILWIND_CSS }
      link { rel: "stylesheet", href: NORMALIZE_CSS }
    }

    div {
      class: "relative mx-auto mt-2 container flex flex-col",
      provider::FileProvider {
        provider::BoardProvider {
          provider::IssueProvider {
            index::Index {  }
          }
        }
      }
    }
  }
}
