use std::{collections::HashSet, path::PathBuf};

use anyhow::{bail, Context};
use axum::{
  body::Body,
  http::{header::CACHE_CONTROL, HeaderValue, Request, Response},
  routing::{get, get_service},
};
use dioxus::{
  fullstack::{get_server_url, set_server_url},
  prelude::ServeConfig,
  server::{FullstackState, ServerFunction},
};
use dioxus_web::App;
use loco_rs::{prelude::Routes, Result};
use tower::{util::MapResponse, ServiceExt};
use tower_http::services::{fs::ServeFileSystemResponseBody, ServeFile};
use tracing::info;

pub fn routes(base_url: &str) -> anyhow::Result<Routes> {
  if let Some(base_path) = dioxus::cli_config::base_path() {
    let base_path = base_path.trim_matches('/');
    set_server_url(format!("{}/{}", base_url, base_path).leak());
  }
  info!(
    "Setting server url for dioxus fullstack to {}",
    get_server_url()
  );

  let Some(public_path) = public_path() else {
    bail!("Could not find public/asset path for dioxus assets, did you set DIOXUS_PUBLIC_PATH ?")
  };

  // Server fn routes are already pointing to the api where they should, do not
  // nest.
  let state = FullstackState::new(ServeConfig::new(), App);
  let server_fn_routes = collect_server_fn_routes(state.clone());
  let static_routes = collect_static_routes(state.clone(), "frontend", public_path)?;

  // Create a catch-all route for /frontend path
  let fallback_route = Routes::new().prefix("/frontend").add(
    "/{*rest}",
    get(FullstackState::render_handler).with_state(state.clone()),
  );

  Ok(Routes::new().merge_all(vec![server_fn_routes, static_routes, fallback_route]))
}

fn public_path() -> Option<PathBuf> {
  if let Ok(path) = std::env::var("DIOXUS_PUBLIC_PATH") {
    return Some(PathBuf::from(path));
  }

  // The CLI always bundles static assets into the exe/public directory
  Some(
    std::env::current_exe()
      .ok()?
      .parent()
      .unwrap()
      .join("public"),
  )
}

fn collect_static_routes(
  state: FullstackState,
  prefix: &str,
  public_path: PathBuf,
) -> Result<Routes, anyhow::Error> {
  let mut path_queue = vec![public_path.clone()];
  let mut static_routes = Routes::new();
  while let Some(path) = path_queue.pop() {
    let dir =
      std::fs::read_dir(&path).context(format!("Couldn't read public directory: {:?}", path))?;

    for entry in dir.flatten() {
      let subpath = entry.path();
      // Don't serve the index.html file. The SSR handler will generate it.
      if subpath == public_path.join("index.html") {
        info!(
          "Not creating a route for {:?} since it should be served by SSR",
          subpath
        );
        continue;
      }

      let route = format!(
        "/{}",
        subpath
          .strip_prefix(&public_path)
          .unwrap()
          .iter()
          .map(|segment| segment.to_string_lossy())
          .collect::<Vec<_>>()
          .join("/")
      );

      if subpath.is_dir() {
        path_queue.push(subpath);
      } else {
        info!("Adding dioxus static path: {}:{:?}", route, subpath);
        let serve_file = ServeFile::new(&subpath).precompressed_br();
        // All cached assets are served at the root of the asset directory. If we know
        // an asset is hashed for cache busting, we can cache the response on
        // the client side forever. If the asset changes, the hash in the path
        // will also change and the client will refetch it.
        if file_name_looks_immutable(&route) {
          static_routes = static_routes.add(
            &format!("{}/{}", prefix, &route),
            get_service(cache_response_forever(serve_file)),
          );
        } else {
          static_routes = static_routes.add(
            &format!("{}/{}", prefix, &route),
            get_service(serve_file).with_state(state.clone()),
          );
        }
      }
    }
  }

  Ok(static_routes)
}

fn collect_server_fn_routes(state: FullstackState) -> Routes {
  let mut server_fn_routes = Routes::new();
  let mut seen = HashSet::new();
  for func in ServerFunction::collect() {
    if seen.insert(format!("{} {}", func.method(), func.path())) {
      tracing::info!(
        "Registering server function: {} {}",
        func.method(),
        func.path()
      );

      server_fn_routes =
        server_fn_routes.add(func.path(), func.method_router().with_state(state.clone()));
    }
  }

  server_fn_routes
}

/// Adapted from dioxus
fn file_name_looks_immutable(file_name: &str) -> bool {
  // Check if the file name looks like a hash (e.g., "main-dxh12345678.js")
  file_name.rsplit_once("-dxh").is_some_and(|(_, hash)| {
    hash
      .chars()
      .take_while(|c| *c != '.')
      .all(|c| c.is_ascii_hexdigit())
  })
}

type MappedAxumService<S> = MapResponse<
  S,
  fn(Response<ServeFileSystemResponseBody>) -> Response<ServeFileSystemResponseBody>,
>;

fn cache_response_forever<S>(service: S) -> MappedAxumService<S>
where
  S: ServiceExt<Request<Body>, Response = Response<ServeFileSystemResponseBody>>,
{
  service.map_response(|mut response: Response<ServeFileSystemResponseBody>| {
    response.headers_mut().insert(
      CACHE_CONTROL,
      HeaderValue::from_static("public, max-age=31536000, immutable"),
    );
    response
  })
}
