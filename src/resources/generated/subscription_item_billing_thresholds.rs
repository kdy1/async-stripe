// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionItemBillingThresholds".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice.
    pub usage_gte: Box<Option<i64>>,
}
