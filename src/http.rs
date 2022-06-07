#[derive(Clone, Copy)]
pub enum Method {
    Get,
    Put,
}

impl Method {
    pub fn name(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Put => "PUT",
        }
    }
}
