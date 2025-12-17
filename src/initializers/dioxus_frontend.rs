use async_trait::async_trait;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use dioxus::prelude::{DioxusRouterExt, ServeConfig};
use dioxus::server::{FullstackState, IncrementalRendererConfig};
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
pub struct DioxusFrontend;

#[async_trait]
impl Initializer for DioxusFrontend {
    fn name(&self) -> String {
        "Dioxus Frontend".to_string()
    }

    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> Result<Router> {
        let dx_server_functions =
            axum::Router::new()
                .register_server_functions()
                .with_state(FullstackState::new(
                    ServeConfig::new().incremental(IncrementalRendererConfig::new()),
                    dioxus_web::App,
                ));
        let dx_static_assets = axum::Router::new()
            .serve_static_assets()
            .fallback(get(FullstackState::render_handler))
            .with_state(FullstackState::new(ServeConfig::new(), dioxus_web::App));

        // The above is a modified expansion of this, see the dioxus docs for details
        // let dioxus_router = axum::Router::new().serve_dioxus_application(
        //     ServeConfig::new().incremental(IncrementalRendererConfig::new()),
        //     dioxus_web::App,
        // );

        // Add redirect BEFORE nesting so it takes precedence
        let router = router
            .route(
                "/",
                get(|| async { Redirect::permanent("/frontend/index.html") }),
            )
            .merge(dx_server_functions)
            .nest("/frontend", dx_static_assets);

        Ok(router)
    }
}
