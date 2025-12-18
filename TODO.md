# TODO

## Dioxus Integration

* frontend prepended onto wasm
* test docker container
* refcell already borrowed, make sure im not accidentally posting to wrong path or something
* Make it so server functions can call loco stuff by making a shared crate which can be depended on which contains models etc
* Change fallback.html to dioxus normal 404
* Is it possible to have cargo loco routes know about initialized routes for dioxus?
* other config yml file setups for frontend
* SSR integration into loco
* make "frontend" subroute configurable?

## Template Utility

* Use some template engine (whatever zola or loco uses) to create this project and ask questions
  * Project Name?
  * Subpath for dioxus server fn calls?
  * SSR?