mod route_main;
mod route;

pub use route_main::*;
pub use route::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Routes {
    Main
}

pub struct Router {
    pub active_route: Routes,
}

impl Router {
    pub fn get_active_route(&self) -> &Routes {
        &self.active_route
    }

    pub fn switch(&mut self, route: Routes) {
        self.active_route = route;
    }
}

