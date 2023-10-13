use serde::{Deserialize, Serialize};

use crate::client::WorkspaceClient;

use super::workspace::WorkspaceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemId(pub i32);

pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub workspace_id: WorkspaceId,
    pub is_product: bool,
    pub sku: String,
    pub tracking_unit: String,
    pub purchase_unit: String,
    pub unit_conversion: f32,
    pub batch_qty: f32,
    pub image_url: Option<String>,
}

impl ItemId {
    pub async fn to_item(self, _client: &WorkspaceClient) -> anyhow::Result<Item> {
        todo!()
    }
}
