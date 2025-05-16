use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use dioxus::html::geometry::ClientPoint;
use dioxus::logger::tracing::*;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::{
  FaCircleLeft, FaCircleXmark, FaSquarePlus,
};
use dioxus_free_icons::Icon;

use crate::context::{
  HyphaBoardContext, HyphaFileContext, HyphaIssueContext, HyphaResizeContext,
  HyphaSearchContext,
};
use crate::file::HyphaFile;
use crate::hooks::use_hypha_initial_render;
use crate::r#ref::{HyphaFileBoardRef, HyphaFileIssueRef};
use crate::resize::HyphaGlobalResize;
use crate::search::HyphaSearch;

#[component]
pub fn HyphaFileProvider(children: Element) -> Element {
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
          HyphaFileProviderInner {
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
fn HyphaFileProviderInner(file: HyphaFile, children: Element) -> Element {
  let file_signal = use_signal(|| file);
  use_context_provider(|| HyphaFileContext::new(file_signal));

  let mut is_out_of_date = use_signal(|| false);
  let is_initial_render = use_hypha_initial_render()();

  use_effect(move || {
    let file_changed = matches!(file_signal(), HyphaFile { .. });
    if file_changed && !is_initial_render {
      is_out_of_date.set(true);
    }
  });

  use_drop(move || {
    if let Err(err) = file_signal().save() {
      error!("Failed to save hypha file: {}", err);
    }
  });

  rsx! {
    button {
      class: "absolute top-0 right-0 flex flex-row items-start z-10",
      onclick: move |_| {
        if let Err(err) = file_signal().save() {
          error!("Failed to save hypha file: {}", err);
        }
        is_out_of_date.set(false);
      },
      span {
        "Save file"
      }
      if is_out_of_date() {
        Icon {
          class: "ml-2 text-gray-500 mt-[5px]",
          width: 20,
          height: 20,
          icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleDot
        }
      }
      else {
        Icon {
          class: "ml-2 text-green-500 mt-[5px]",
          width: 20,
          height: 20,
          icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleCheck
        }
      }
    }
    { children }
  }
}

#[component]
pub fn HyphaBoardProvider(children: Element) -> Element {
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
          class: "absolute top-0 left-0 flex flex-row z-10",
          onclick: move |_| {
            *board_signal.write() = None;
          },
          Icon {
            class: "mr-2 text-grey-500 mt-[4px]",
            width: 20,
            height: 20,
            icon: FaCircleLeft
          }
          span {
            "Back to boards"
          }
        }
        HyphaBoardProviderInner {
          board: board,
          {children}
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
              class: "w-[30rem] mt-32 flex flex-col items-center max-h-[40rem] overflow-auto",
              HyphaSearchProvider {
                class: "mt-2 mb-4",
                render: use_callback(move |search| {
                  let boards = context
                    .get().boards
                    .iter()
                    .cloned()
                    .filter(|board| board.title.to_lowercase().starts_with(&search))
                    .collect::<Vec<_>>();
                  rsx! {
                    for board in boards {
                      div {
                        class: "flex flex-row w-[26rem] mb-4",
                        strong {
                          class: "cursor-pointer grow truncate",
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
                })
              }
            }
          }
          button {
            class: "w-40 text-center mt-4 flex flex-row justify-center",
            onclick: move |_| {
              context.add_board();
            },
            Icon {
              class: "mr-2 text-green-500 mt-[4px]",
              width: 20,
              height: 20,
              icon: FaSquarePlus
            }
            span {
              "Add board"
            }
          }
        }
      }
    }
  }
}

#[component]
fn HyphaBoardProviderInner(
  board: HyphaFileBoardRef,
  children: Element,
) -> Element {
  let signal = use_signal(|| board);
  use_context_provider(|| HyphaBoardContext::new(signal));

  rsx! {
    { children }
  }
}

#[component]
pub fn HyphaIssueProvider(children: Element) -> Element {
  let signal = use_signal(|| Option::<HyphaFileIssueRef>::None);
  use_context_provider(|| HyphaIssueContext::new(signal));

  rsx! {
    { children }
  }
}

#[component]
pub fn HyphaResizeProvider(children: Element) -> Element {
  let mut signal = use_signal(|| HyphaGlobalResize {
    position: ClientPoint::zero(),
    resizes: HashMap::new(),
  });
  use_context_provider(|| HyphaResizeContext::new(signal));

  rsx! {
    div {
      class: "min-h-[100vh]",
      onmousemove: move |e| {
        let position_before = signal.read().position;
        let position_after = e.client_coordinates();
        let height_difference = position_after.y - position_before.y;
        let mut writer = signal.write();
        writer.resizes.iter_mut().for_each(move |(_, (height, dragging))| {
          *height += (*dragging as i32) * (height_difference as i32)
        });
        writer.position = position_after;
      },
      onmouseleave: move |_| {
        let mut writer = signal.write();
        writer.resizes.iter_mut().for_each(|(_, (_, dragging))| {
          *dragging = false;
        });
      },
      onmouseup: move |_| {
        let mut writer = signal.write();
        writer.resizes.iter_mut().for_each(|(_, (_, dragging))| {
          *dragging = false;
        });
      },
      {children}
    }
  }
}

#[component]
pub fn HyphaSearchProvider(
  class: Option<String>,
  style: Option<String>,
  render: Callback<String, Element>,
) -> Element {
  let class = class.unwrap_or(String::new());
  let style = style.unwrap_or(String::new());

  let mut signal = use_signal(|| HyphaSearch {
    search: String::new(),
  });
  use_context_provider(|| HyphaSearchContext::new(signal));

  rsx! {
    input {
      class: class,
      style: style,
      value: signal.read().search.clone(),
      oninput: move |e| {
        signal.write().search = e.value();
      }
    }
    {render(signal.read().search.to_lowercase())}
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
