use dioxus::prelude::*;

pub fn use_hypha_initial_render() -> Signal<bool> {
  let mut is_initial_signal = use_signal(|| true);

  use_effect(move || {
    is_initial_signal.set(false);
  });

  is_initial_signal
}

pub fn use_hypha_autofocus(signal: Signal<bool>, id: &str) {
  let id = id.to_string();
  use_effect(move || {
    let id = id.clone();
    let signal = signal();
    if signal {
      spawn(async move {
        let id = id.clone();
        if let Err(err) = document::eval(
          format!("document.getElementById('{id}')?.focus();").as_str(),
        )
        .await
        {
          dioxus::logger::tracing::error!("Autofocus failed {err:?}");
        }
      });
    }
  });
}

pub fn use_hypha_autogrow(signal: Signal<bool>, id: &str) {
  {
    let id = id.to_string();
    use_effect(move || {
      let id = id.clone();
      let changed = signal();
      if changed {
        spawn(async move {
          let id = id.clone();
          if let Err(err) = document::eval(
            format!(
              r#"
                const element = document.getElementById('{id}');
                element.hyphaAutoGrow = () => {{
                  element.style.height = 'auto';
                  element.style.height = element.scrollHeight + 'px';
                }};
                element.hyphaAutoGrow();
                element.addEventListener('input', element.hyphaAutoGrow);
              "#
            )
            .as_str(),
          )
          .await
          {
            dioxus::logger::tracing::error!(
              "Autogrow subscribe failed {err:?}"
            );
          }
        });
      }
    });
  }

  {
    let id = id.to_string();
    use_drop(move || {
      let id = id.clone();
      let signal = signal();
      if signal {
        spawn(async move {
          let id = id.clone();
          if let Err(err) = document::eval(
            format!(
              r#"
                const element = document.getElementById('{id}');
                element.removeEventListener('input', element.hyphaAutoGrow);
              "#
            )
            .as_str(),
          )
          .await
          {
            dioxus::logger::tracing::error!(
              "Autogrow unsubscribe failed {err:?}"
            );
          }
        });
      }
    });
  }
}
