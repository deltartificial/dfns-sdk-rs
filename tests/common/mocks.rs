/// @dfns-sdk-rs/tests/common/mocks.rs

use mockito::{Mock, Server, ServerGuard};

pub fn create_mock(method: &str, path: &str) -> Mock {
    let mut server = Server::new();
    server.mock(method, path)
}

#[allow(dead_code)]
pub fn setup_mock_server() -> ServerGuard {
    Server::new()
}