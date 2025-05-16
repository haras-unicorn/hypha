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
pub fn HyphaVerticalResize(children: Element) -> Element {
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
      div {
        class: "w-full overflow-hidden",
        id: id().to_string(),
        style: match context.height(&id()) {
          Some(height) => format!("height: {height}px;"),
          None => String::new(),
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
}
