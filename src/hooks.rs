use dioxus::prelude::*;

pub fn use_initial_render() -> Signal<bool> {
  let mut is_initial_signal = use_signal(|| true);

  use_effect(move || {
    is_initial_signal.set(false);
  });

  is_initial_signal
}
