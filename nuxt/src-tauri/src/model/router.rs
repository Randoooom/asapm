use std::collections::HashMap;
use matchit::{Node, Match};
use tauri::{AppHandle, Wry};
use tauri::http::{Request, Response, method::Method, ResponseBuilder};

// setup types
// from tauri_runtime
type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
type HandlerFn = fn(&AppHandle<Wry>, &Request) -> Result<Response>;

pub struct Router {
  handlers: HashMap<Method, Node<HandlerFn>>,
}

impl Router {
  // create new router
  pub fn new() -> Self {
    Self {
      handlers: HashMap::new()
    }
  }

  /// add handler to router
  fn add_handler(&mut self, path: &str, handler: HandlerFn, method: Method) {
    self.handlers
      .entry(method.clone())
      .or_insert_with(Node::new)
      .insert(path, handler.clone())
      .unwrap()
  }

  /// add get method
  pub fn get(mut self, path: &str, handler: HandlerFn) -> Self
  {
    self.add_handler(
      path,
      handler,
      Method::GET,
    );
    self
  }

  /// add post method
  pub fn post(mut self, path: &str, handler: HandlerFn) -> Self
  {
    self.add_handler(
      path,
      handler,
      Method::POST,
    );
    self
  }

  /// add put method
  pub fn put(mut self, path: &str, handler: HandlerFn) -> Self
  {
    self.add_handler(
      path,
      handler,
      Method::PUT,
    );
    self
  }

  /// add delete method
  pub fn delete(mut self, path: &str, handler: HandlerFn) -> Self
  {
    self.add_handler(
      path,
      handler,
      Method::DELETE,
    );
    self
  }

  /// start the router for one time with the given data and return the response
  pub fn execute(self, app: &AppHandle<Wry>, request: &Request) -> Result<Response> {
    let handlers = self.handlers.clone();

    if let Some(handlers) = handlers.get(request.method()) {
      let path = request.uri().replace("api:///", "");
      if let Ok(Match { value, params }) = handlers.at(path.as_str()) {
        return value(app, request);
      }
    }

    // handle options for preflight
    if request.method() == &Method::OPTIONS {
      return ResponseBuilder::new().status(204).body(Vec::new());
    }

    ResponseBuilder::new().status(404).body(Vec::new())
  }
}

