# TODO

## Dioxus Integration

* Make default route when requesting HTML forward to frontend index.html
  Same for frontend to /frontend/index.html
* Set env in cargo build tool: DIOXUS_PUBLIC_PATH to ./target/dx/dioxus_web/debug/web/public
  This happens because we're building from the root workspace but the dioxus tooling assumes dioxus to be controlling the workspace structure and for dx to be the current executable, but it isnt loco application is.
* Mount dioxus router so the nesting is automatic on the frontend
* proxy to loco instead of dioxus on dx serve
* Change fallback.html to dioxus normal 404
* other config yml file setups for frontend
* SSR integration into loco

## Template Utility

* Use some template engine (whatever zola or loco uses) to create this project and ask questions
  * Project Name?
  * Subpath for dioxus server fn calls?
  * SSR?