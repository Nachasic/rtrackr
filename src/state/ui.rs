pub enum Routes {
    Main,
    Error(String),
    RecordEditor,
}

// TODO: Change the router so it works like a stack
pub struct Router {
    pub active_routes: Vec<Routes>
}

impl Default for Router {
    fn default() -> Self {
        Self {
            active_routes: vec![Routes::Main]
        }
    }
}

impl Router {
    pub fn push(&mut self, route: Routes) {
        self.active_routes.push(route);
    }

    pub fn pop(&mut self) -> Option<Routes> {
        self.active_routes.pop()
    }

    pub fn switch(&mut self, route: Routes) {
        if self.active_routes.len() >= 2 {
            self.pop();
            self.push(route)
        }
    }
}