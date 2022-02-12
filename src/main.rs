use webrtc_test::make_offer;

#[tokio::main]
async fn main() -> Result<(), webrtc::Error> {
    make_offer().await
}
