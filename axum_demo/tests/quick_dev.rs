use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/hello2/sean").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "sean",
            "pwd": "hello",
        }),
    );
    req_login.await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAAA"
        }),
    ).await?.print().await?;
    hc.do_get(
        "/api/tickets",
    ).await?.print().await?;



    Ok(())
}
