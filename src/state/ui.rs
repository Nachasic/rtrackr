pub enum Routes {
    Main
}

// TODO: Change the router so it works like a stack
pub struct Router {
    pub active_route: Routes
}

impl Default for Router {
    fn default() -> Self {
        Self {
            active_route: Routes::Main
        }
    }
}

impl Router {
    pub fn go_to(&mut self, route: Routes) {
        self.active_route = route;
    }
}