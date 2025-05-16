use std::collections::HashMap;

use dioxus::{html::geometry::ClientPoint, prelude::*};
use uuid::Uuid;

use crate::context::HyphaResizeContext;

#[derive(Debug, Clone)]
pub struct HyphaGlobalResize {
  pub position: ClientPoint,
  pub resizes: HashMap<Uuid, (i32, bool)>,
}

#[component]
pub fn HyphaVerticalResize(
  class: Option<String>,
  style: Option<String>,
  min: Option<String>,
  max: Option<String>,
  children: Element,
) -> Element {
  let class = class.unwrap_or(String::new());
  let style = style.unwrap_or(String::new());
  let min = min.unwrap_or_else(|| "1px".to_string());
  let max = max.unwrap_or_else(|| "100vh".to_string());

  let mut context = use_context::<HyphaResizeContext>();
  let id = use_signal(|| uuid::Uuid::new_v4());

  use_effect(move || {
    spawn(async move {
      match document::eval(&format!(
        "return document.getElementById('{}')?.offsetHeight",
        id()
      ))
      .await
      .map(|height| match height {
        serde_json::Value::Number(height) => {
          height.as_i64().map(|height| height as i32)
        }
        _ => None,
      }) {
        Ok(Some(height)) => {
          context.subscribe(id(), height);
        }
        _ => {
          dioxus::logger::tracing::error!("Failed getting height of {}", id());
        }
      }
    });
  });

  use_drop(move || {
    context.unsubscribe(&id());
  });

  rsx! {
    div {
      id: id().to_string(),
      class: class + " relative justify-start w-full grow overflow-hidden flex flex-col",
      style: match context.height(&id()) {
        Some(height) => format!("height: clamp({min}, {height}px, {max}); {style}"),
        None => style,
      },
      {children}
    }
    div {
      class: "w-full h-1 bg-zinc-500 cursor-row-resize",
      onmousedown: move |_| {
        context.dragging(&id());
      }
    }
  }
}
