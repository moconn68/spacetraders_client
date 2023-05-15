// mod api;
// mod utils;

use spacetraders_client::proto;
fn main() {
    proto::test_create_api_client();
    proto::test_read_config();
    proto::test_write_config("TEST_TOKEN");
}
