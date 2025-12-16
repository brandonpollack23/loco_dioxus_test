# TODO

## General local stuff

* dont watch frontend directory, that will rebuild and just update static assets?  New routes excluded

## Dioxus Integration

* Make sure production build works
* fix redirect from /frontend to /frontend/index.html
* Make it so server functions can call loco stuff by making a shared crate which can be depended on which contains models etc
* Set env in cargo build tool: DIOXUS_PUBLIC_PATH to ./target/dx/dioxus_web/debug/web/public
  This happens because we're building from the root workspace but the dioxus tooling assumes dioxus to be controlling the workspace structure and for dx to be the current executable, but it isnt loco application is.
* Mount dioxus router so the nesting is automatic on the frontend
* proxy to loco instead of dioxus on dx serve
* Change fallback.html to dioxus normal 404
* Is it possible to have cargo loco routes know about initialized routes for dioxus?
* other config yml file setups for frontend
* SSR integration into loco

## Template Utility

* Use some template engine (whatever zola or loco uses) to create this project and ask questions
  * Project Name?
  * Subpath for dioxus server fn calls?
  * SSR?