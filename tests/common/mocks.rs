/// @dfns-sdk-rs/tests/common/mocks.rs
use mockito::{Mock, ServerGuard};

pub fn create_mock_server() -> ServerGuard {
    mockito::Server::new()
}

pub fn create_mock_with_server(server: &mut ServerGuard, method: &str, path: &str) -> Mock {
    server.mock(method, path)
}
