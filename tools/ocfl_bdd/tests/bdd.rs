mod steps;

use cucumber::World;
use steps::OcflWorld;

#[tokio::main]
async fn main() {
    OcflWorld::run("tests/features").await;
}
