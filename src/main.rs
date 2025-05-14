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
mod hooks;
mod index;
mod issue;
mod item;
mod list;
mod provider;
mod r#ref;
mod resize;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("assets/favicon.ico");
const ROOT_CSS: Asset = asset!("assets/root.css");
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

const INTER_BOLD: Asset = asset!("assets/fonts/Inter_28pt-Bold.ttf");
const INTER_ITALIC: Asset = asset!("assets/fonts/Inter_28pt-Italic.ttf");
const INTER_REGULAR: Asset = asset!("assets/fonts/Inter_28pt-Regular.ttf");

fn main() {
  #[cfg(feature = "desktop")]
  {
    use dioxus::desktop::*;
    LaunchBuilder::new()
      .with_cfg(
        Config::new().with_window(
          WindowBuilder::new()
            .with_decorations(false)
            .with_transparent(true),
        ),
      )
      .launch(App);
  }
}

#[component]
fn App() -> Element {
  let fonts = format!(
    r#"
      @font-face {{
        font-family: 'Inter';
        font-style: normal;
        font-weight: normal;
        font-display: swap;
        src: url({}) format('truetype');
      }}
      @font-face {{
        font-family: 'Inter';
        font-style: italic;
        font-weight: normal;
        font-display: swap;
        src: url({}) format('truetype');
      }}
      @font-face {{
        font-family: 'Inter';
        font-style: normal;
        font-weight: bold;
        font-display: swap;
        src: url({}) format('truetype');
      }}
    "#,
    INTER_REGULAR, INTER_ITALIC, INTER_BOLD
  );

  let text = r#"
    div[id="main"] {
      font-family: 'Inter';
      font-weight: normal;
      font-style: normal;
    }

    h1 {
      font-size: 3em;
      font-weight: bold;
      font-style: normal;
    }

    h2 {
      font-size: 2.5em;
      font-weight: bold;
      font-style: normal;
    }

    h3 {
      font-size: 2em;
      font-weight: bold;
      font-style: normal;
    }

    h4 {
      font-size: 1.75em;
      font-weight: bold;
      font-style: normal;
    }

    h5 {
      font-size: 1.5em;
      font-weight: bold;
      font-style: normal;
    }

    h6 {
      font-size: 1.25em;
      font-weight: bold;
      font-style: normal;
    }

    p, span, code {
      font-size: 1.25em;
      font-weight: normal;
      font-style: normal;
    }

    em, i {
      font-size: 1.25em;
      font-weight: normal;
      font-style: italic;
    }

    strong, b {
      font-size: 1.25em;
      font-weight: bold;
      font-style: normal;
    }

    a {
      text-decoration: none;
    }

    a:hover {
      text-decoration: underline;
    }
  "#;

  rsx! {
    head {
      link { rel: "icon", href: FAVICON }
      link { rel: "stylesheet", href: ROOT_CSS }
      link { rel: "stylesheet", href: TAILWIND_CSS }

      style { {fonts} }
      style { {text} }
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
