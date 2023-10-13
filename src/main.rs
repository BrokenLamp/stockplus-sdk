use stockplus_sdk::client::StockPlusClient;

#[tokio::main]
async fn main() {
    let client = StockPlusClient::new();
    let user_client = client
        .login("jon.doe@example.com", "Test123")
        .await
        .unwrap();
    println!("{:?}", user_client);
    let workspaces = user_client.list_workspaces().await.unwrap();
    println!("{:#?}", workspaces);
    let workspace_id = workspaces[0].workspace_id;
    let workspace = workspace_id.to_workspace(&user_client).await.unwrap();
    println!("{:#?}", workspace);
    let workspace_client = user_client.auth_workspace(workspace_id).await.unwrap();
    println!("{:?}", workspace_client);
    let users = workspace_client.list_users().await.unwrap();
    println!("{:#?}", users);
}
