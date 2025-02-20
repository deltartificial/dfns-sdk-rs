/// @dfns-sdk-rs/tests/common/mocks.rs
use mockito::{Mock, ServerGuard};
use tokio_test::block_on;

pub fn create_mock_server() -> ServerGuard {
    block_on(async {
        mockito::Server::new()
    })
}

pub fn create_mock_with_server(server: &mut ServerGuard, method: &str, path: &str) -> Mock {
    server.mock(method, path)
}
