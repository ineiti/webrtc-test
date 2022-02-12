use webrtc_test::make_offer;

#[tokio::main]
async fn main() -> Result<(), webrtc::Error> {
    env_logger::init();
    make_offer().await
}

#[tokio::test(flavor = "multi_thread")]
async fn test_make_offer() -> Result<(), webrtc::Error> {
    env_logger::init();
    make_offer().await
}