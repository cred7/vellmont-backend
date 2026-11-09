use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionSchema {
    pub offer_id: Uuid,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTransactionSchema {
    pub status: String,
}
