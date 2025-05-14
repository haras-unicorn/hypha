use std::path::{Path, PathBuf};
use std::str::FromStr;

use dioxus::logger::tracing::*;
use dioxus::prelude::*;

use crate::context::{HyphaBoardContext, HyphaFileContext, HyphaIssueContext};
use crate::file::HyphaFile;
use crate::r#ref::{HyphaFileBoardRef, HyphaFileIssueRef};

#[component]
pub fn FileProvider(children: Element) -> Element {
  let mut path = use_signal(|| Option::<PathBuf>::None);

  let mut file_resource = use_resource(move || async move {
    if let Some(path) = path() {
      return load(path.as_path()).await;
    }

    match HyphaFile::path() {
      Ok(path) => load(path.as_path()).await,
      Err(e) => {
        error!("Failed to determine hypha file path: {e}. Letting user pick.");
        None
      }
    }
  });

  match *file_resource.state().read() {
    UseResourceState::Ready => match &*file_resource.read() {
      Some(file) => match file {
        Some(file) => rsx! {
          FileProviderInner {
            file: file.clone(),
            {children}
          }
        },
        None => {
          rsx! {
            p {
              input {
                value: path()
                  .and_then(|path| path.to_str().map(|path| path.to_string()))
                  .unwrap_or(String::new()),
                oninput: move |e| {
                  let Ok(new_path) = PathBuf::from_str(e.value().as_str());
                  *path.write() = Some(new_path);
                }
              }
            }
            button {
              onclick: move |_| {
                file_resource.restart();
              },
              "Add"
            }
          }
        }
      },
      None => rsx! {
        p { "Loading..." }
      },
    },
    _ => {
      rsx! {
        p { "Loading..." }
      }
    }
  }
}

#[component]
fn FileProviderInner(file: HyphaFile, children: Element) -> Element {
  let signal = use_signal(|| file);
  use_context_provider(|| HyphaFileContext::new(signal));

  use_drop(move || {
    if let Err(err) = signal().save() {
      error!("Failed to save hypha file: {}", err);
    }
  });

  rsx! {
    button {
      class: "absolute top-0 right-0 mt-4",
      onclick: move |_| {
        if let Err(err) = signal().save() {
          error!("Failed to save hypha file: {}", err);
        }
      },
      "Save"
    }
    { children }
  }
}

#[component]
pub fn BoardProvider(children: Element) -> Element {
  let mut context = use_context::<HyphaFileContext>();
  let mut board_signal = use_signal(|| {
    context.get().boards.first().map(|board| HyphaFileBoardRef {
      board: board.title.clone(),
    })
  });

  match board_signal() {
    Some(board) => {
      rsx! {
        button {
          class: "absolute top-0 left-0 mt-4",
          onclick: move |_| {
            *board_signal.write() = None;
          },
          "Back"
        }
        div {
          class: "mt-4",
          BoardProviderInner {
            board: board,
            {children}
          }
        }
      }
    }
    None => {
      rsx! {
        div {
          class: "flex flex-col items-center",
          div {
            class: "flex flex-row justify-center",
            div {
              class: "w-80 mt-32 flex flex-col items-center max-h-80 overflow-auto",
              for board in context.get().boards {
                div {
                  class: "flex flex-row w-64 mb-4",
                  p {
                    class: "cursor-pointer grow",
                    onclick: {
                      let board_title = board.title.clone();
                      move |_| {
                        *board_signal.write() = Some(HyphaFileBoardRef { board: board_title.clone() });
                      }
                    },
                    "{board.title.clone()}"
                  }
                  span {
                    class: "cursor-pointer",
                    onclick: {
                      let board_title = board.title.clone();
                      move |_| {
                        context.remove_board(HyphaFileBoardRef { board: board_title.clone() });
                      }
                    },
                    "X"
                  }
                }
              }
            }
          }
          p {
            class: "cursor-pointer w-24 text-center mt-4",
            onclick: move |_| {
              context.add_board();
            },
            "Add"
          }
        }
      }
    }
  }
}

#[component]
fn BoardProviderInner(board: HyphaFileBoardRef, children: Element) -> Element {
  let signal = use_signal(|| board);
  use_context_provider(|| HyphaBoardContext::new(signal));

  rsx! {
    { children }
  }
}

#[component]
pub fn IssueProvider(children: Element) -> Element {
  let signal = use_signal(|| Option::<HyphaFileIssueRef>::None);
  use_context_provider(|| HyphaIssueContext::new(signal));

  rsx! {
    { children }
  }
}

async fn load(path: &Path) -> Option<HyphaFile> {
  match HyphaFile::load_async(path).await {
    Ok(file) => Some(file),
    Err(e) => {
      error!(
        "Failed to load config file at {}: {}. Using default.",
        path.display(),
        e
      );
      let default_file = HyphaFile {
        path: path.to_path_buf(),
        ..HyphaFile::default()
      };
      if let Err(save_err) = default_file.save_async().await {
        error!("Failed to save default config file: {save_err}");
      }
      Some(default_file)
    }
  }
}
