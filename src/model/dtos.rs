use std::collections::HashMap;

use super::ts::HistoryRecord;


#[derive(Debug, Clone)]
pub struct IdentityData {
    pub unique_id: String,
    pub display_name: String,
    pub attributes: Vec<(String,String)>,

    pub personal_accounts: HashMap<String,Vec<AccountData>>, 
    pub owned_accounts: HashMap<String,Vec<AccountData>>,
    pub owned_groups: HashMap<String,Vec<EntitlementData>>, 
}
impl IdentityData {
    pub fn get_personal_accounts_mut(&mut self) -> HashMap<String, Vec<&mut AccountData>> {
        self.personal_accounts.iter_mut()
            .map(|(key, vec)| 
                (key.clone(), vec.iter_mut().collect())
            ).collect()
    }
    pub fn get_owned_accounts_mut(&mut self) -> HashMap<String, Vec<&mut AccountData>> {
        self.owned_accounts.iter_mut()
            .map(|(key, vec)| 
                (key.clone(), vec.iter_mut().collect())
            ).collect()
    }
    pub fn get_owned_groups_mut(&mut self) -> HashMap<String, Vec<&mut EntitlementData>> {
        self.owned_groups.iter_mut()
            .map(|(key, vec)| 
                (key.clone(), vec.iter_mut().collect())
            ).collect()
    }
    pub fn get_personal_accounts_ref(&self) -> HashMap<String, Vec<&AccountData>> {
        self.personal_accounts.iter()
            .map(|(key, vec)| 
                (key.clone(), vec.iter().collect())
            ).collect()
    }
    pub fn get_owned_accounts_ref(&self) -> HashMap<String, Vec<&AccountData>> {
        self.owned_accounts.iter()
            .map(|(key, vec)| 
                (key.clone(), vec.iter().collect())
            ).collect()
    }
    pub fn get_owned_groups_ref(&self) -> HashMap<String, Vec<&EntitlementData>> {
        self.owned_groups.iter()
            .map(|(key, vec)| 
                (key.clone(), vec.iter().collect())
            ).collect()
    }
}

#[derive(Debug, Clone)]
pub struct AccountData {
    pub uid: String,
    pub display_name: String,
    pub description: String,
    pub entitlements: Vec<EntitlementData>,
    pub indirect_entitlements: Vec<EntitlementData>,
    pub account_type: String, 
    pub ou: String,
    pub enabled: String,
    pub syncs_to_ts: String,
    pub syncs_to_account: String,
    pub identity_owners: Vec<String>,
    pub macheo: bool,
    pub history: Vec<HistoryRecord>,
}

 #[derive(Debug, Clone)]
pub struct EntitlementData {
    pub uid: String,
    pub display_name: String,
    pub description: String,
    pub member_accounts: Vec<AccountData>,
    pub member_groups: Vec<EntitlementData>,
    pub entitlement_type: String,
    pub ou: String,
    pub syncs_to_ts: String,
    pub syncs_to_entitlement: String,
    pub identity_owners: Vec<String>,
    pub macheo: bool,
    pub history: Vec<HistoryRecord>,
}

#[derive(Debug)]
pub struct CategoryTotals {
    pub ts_uid: String,
    pub categories: String,
    pub totals: HashMap<String, i32>,
}
pub struct CategorizedAccounts {
    pub ts_uid: String,
    pub type_lists: HashMap<String, Vec<AccountData>>,
}
pub struct CategorizedEntitlements {
    pub ts_uid: String,
    pub type_lists: HashMap<String, Vec<EntitlementData>>,
}


