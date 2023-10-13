use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::UserClient;

use super::{permissions::Permissions, user::UserId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub i32);

impl WorkspaceId {
    pub async fn to_workspace(self, client: &UserClient) -> anyhow::Result<Workspace> {
        let workspace = client
            .request(Method::GET, &format!("workspace/{}", self.0))
            .send()
            .await?
            .json()
            .await?;
        Ok(workspace)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceUser {
    pub workspace_id: WorkspaceId,
    pub user_id: UserId,
    pub permissions: Permissions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
}
