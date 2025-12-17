# TODO

## Dioxus Integration

* fix redirect from /frontend to /frontend/index.html
* Production problems: CSS files are not picked up by backend server
* Make it so server functions can call loco stuff by making a shared crate which can be depended on which contains models etc
* Change fallback.html to dioxus normal 404
* Is it possible to have cargo loco routes know about initialized routes for dioxus?
* other config yml file setups for frontend
* SSR integration into loco

## Template Utility

* Use some template engine (whatever zola or loco uses) to create this project and ask questions
  * Project Name?
  * Subpath for dioxus server fn calls?
  * SSR?