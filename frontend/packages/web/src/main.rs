#[cfg(feature = "server")]
use dioxus::prelude::dioxus_server;
use dioxus::{logger::tracing::Level, prelude::info};

fn main() {
  dioxus::logger::init(Level::INFO).expect("error initializing logger");

  if cfg!(debug_assertions) {
    console_error_panic_hook::set_once();
  }

  #[cfg(all(not(feature = "server"), feature = "web"))]
  {
    let origin = web_sys::window()
      .expect("could not get web sys window")
      .location()
      .origin()
      .expect("could not get web sys window origin");

    dioxus::fullstack::set_server_url(origin.leak());
  }

  info!(
    "launching web app with origin: {}",
    dioxus::fullstack::get_server_url()
  );

  #[cfg(feature = "web")]
  return dioxus_web_lib::launch::launch(dioxus_web::App, vec![], vec![]);

  #[cfg(feature = "server")]
  return dioxus_server::launch_cfg(dioxus_web::App, vec![], vec![]);

  info!("Currently only web and dev server are supported, see dioxus::launch(App) for how to add other platforms")
}
