use webrtc_test::make_offer;

#[tokio::test]
async fn test_make_offer() -> Result<(), webrtc::Error> {
    make_offer().await
}