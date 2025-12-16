use async_trait::async_trait;
use axum::Router;
use dioxus::prelude::{DioxusRouterExt, ServeConfig};
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
pub struct DioxusFrontend;

#[async_trait]
impl Initializer for DioxusFrontend {
    fn name(&self) -> String {
        "Dioxus Frontend".to_string()
    }

    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> Result<Router> {
        let dioxus_router =
            axum::Router::new().serve_dioxus_application(ServeConfig::new(), dioxus_web::App);

        let router = router.nest("/frontend", dioxus_router);

        Ok(router)
    }
}
