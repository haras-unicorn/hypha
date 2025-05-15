use dioxus::prelude::*;

use crate::context::HyphaResizeContext;

#[derive(Debug, Clone, Copy)]
pub struct HyphaGlobalResize {
  pub moved: EventHandler<Event<MouseData>>,
  pub left: EventHandler<Event<MouseData>>,
}

#[component]
pub fn HyphaVerticalResize(children: Element) -> Element {
  let mut context = use_context::<HyphaResizeContext>();
  let mut height = use_signal(|| None as Option<i32>);
  let mut position = use_signal::<(f64, f64)>(|| (0.0, 0.0));
  let id = use_signal(|| uuid::Uuid::new_v4().to_string());

  use_effect(move || {
    spawn(async move {
      match document::eval(&format!(
        "return document.getElementById('{}')?.offsetHeight",
        id()
      ))
      .await
      .map(|h| match h {
        serde_json::Value::Number(h) => h.as_i64().map(|h| h as i32),
        _ => None,
      }) {
        Ok(Some(h)) => {
          height.set(Some(h));
        }
        _ => {
          dioxus::logger::tracing::error!("Failed getting height of {}", id());
        }
      }
    });
  });

  rsx! {
    div {
      div {
        class: "w-full overflow-hidden",
        id: id(),
        style: match height() {
          Some(height) => format!("height: {height}px;"),
          None => String::new(),
        },
        {children}
      }
      div {
        class: "w-full h-1 bg-zinc-500 cursor-row-resize",
        onmousedown: move |e: Event<MouseData>| {
          let coordinates = e.client_coordinates();
          position.set((coordinates.x, coordinates.y));
          context.subscribe(HyphaGlobalResize {
            moved: EventHandler::new(move |e: Event<MouseData>| {
              let coordinates = e.client_coordinates();
              let mx = coordinates.x;
              let my = coordinates.y;
              if let Some(oh) = height() {
                let (_, y) = position();
                let dy = my - y;
                let h = oh + dy as i32;
                height.set(Some(h));
              }
              position.set((mx, my));
            }),
            left: EventHandler::new(move |_: Event<MouseData>| {
              context.unsubscribe();
            })
          });
        }
      }
    }
  }
}
