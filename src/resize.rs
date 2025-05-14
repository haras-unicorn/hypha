use dioxus::logger::tracing::*;
use dioxus::prelude::*;

#[component]
pub fn VerticalResize(children: Element) -> Element {
  let mut height = use_signal(|| None as Option<i32>);
  let mut dragging = use_signal(|| false);
  let mut drag = use_signal::<(f64, f64)>(|| (0.0, 0.0));
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
          error!("Failed getting height of {}", id());
        }
      }
    });
  });

  rsx! {
    div {
      class: "w-full",
      id: id(),
      style: match height() {
        Some(height) => format!("height: {height}px;"),
        None => String::new(),
      },
      onmousemove: move |e| {
        if dragging() {
          if let Some(oh) = height() {
            let (_, y) = drag();
            let my = e.client_coordinates().y;
            let dy = my - y;
            let h = oh + dy as i32;
            height.set(Some(h));
          }
        }
      },
      onmouseup: move |_| {
        if dragging() {
          dragging.set(false);
        }
      },
      {children}
      div {
        class: "w-full h-1 bg-zinc-500",
        onmousedown: move |e| {
          dragging.set(true);
          let coordinates = e.client_coordinates();
          drag.set((coordinates.x, coordinates.y));
        }
      }
    }
  }
}
