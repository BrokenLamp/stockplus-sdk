use std::collections::HashMap;

use reqwest::{Method, RequestBuilder};

use crate::{
    auth_token::AuthToken,
    models::workspace::{WorkspaceId, WorkspaceUser},
};

#[derive(Debug, Clone)]
pub struct StockPlusClient {
    base_url: String,
    request_client: reqwest::Client,
}

impl StockPlusClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.stockplus.one".to_string(),
            request_client: reqwest::Client::new(),
        }
    }

    pub fn base_url<S: ToString>(mut self, base_url: S) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    pub fn request(&self, method: reqwest::Method, path: &str) -> RequestBuilder {
        self.request_client
            .request(method, &format!("{}/{}", self.base_url, path))
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::GET, path)
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::POST, path)
    }

    pub async fn login(self, email: &str, password: &str) -> anyhow::Result<UserClient> {
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("password", password);
        let token_str = self.post("/login").json(&body).send().await?.text().await?;
        Ok(UserClient {
            base_client: self,
            user_token: AuthToken(token_str),
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserClient {
    pub base_client: StockPlusClient,
    pub user_token: AuthToken,
}

impl UserClient {
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.base_client
            .request(method, path)
            .bearer_auth(&self.user_token.0)
    }

    pub async fn auth_workspace(
        &self,
        workspace_id: WorkspaceId,
    ) -> anyhow::Result<WorkspaceClient> {
        let token = self
            .request(Method::POST, &format!("workspace/{}/auth", workspace_id.0))
            .send()
            .await?
            .text()
            .await?;
        Ok(WorkspaceClient {
            base_client: self.base_client.clone(),
            workspace_token: AuthToken(token),
        })
    }

    pub async fn list_workspaces(&self) -> anyhow::Result<Vec<WorkspaceUser>> {
        let workspaces = self
            .request(Method::GET, "/workspaces")
            .send()
            .await?
            .json()
            .await?;
        Ok(workspaces)
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceClient {
    pub base_client: StockPlusClient,
    pub workspace_token: AuthToken,
}

impl WorkspaceClient {
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.base_client
            .request(method, path)
            .bearer_auth(&self.workspace_token.0)
    }

    pub async fn list_users(&self) -> anyhow::Result<Vec<WorkspaceUser>> {
        let users = self
            .request(Method::GET, "/workspace/users")
            .send()
            .await?
            .json()
            .await?;
        Ok(users)
    }
}
