use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WindowInfo {
    pub uid: u64,
    pub title: Option<String>,
    pub app_name: Option<String>,
    pub app_class: Option<String>,
}

impl WindowInfo {
    pub fn build(uid: u64) -> Self {
        Self {
            uid,
            title: None,
            app_class: None,
            app_name: None
        }
    }

    pub fn with_title<T> (mut self, title: T) -> Self
        where T: Into<String> {
            self.title = Some(title.into());
            self
    }

    pub fn with_app_name<T> (mut self, app_name: T) -> Self
        where T: Into<String> {
            self.app_name = Some(app_name.into());
            self
    }

    pub fn with_app_class<T> (mut self, app_class: T) -> Self
        where T: Into<String> {
            self.app_class = Some(app_class.into());
            self
        }
}