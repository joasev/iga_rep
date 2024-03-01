
use std::collections::HashMap;
use chrono::{Local, NaiveDate};
use anyhow::Result;
use crate::{connectors::{dtos::{AccountDTO, EntitlementDTO}, ad::ADConnector}};
use super::{iga::{Iga, Identity}, dtos::{EntitlementData, AccountData, CategorizedAccounts, CategoryTotals, CategorizedEntitlements}};


#[derive(Debug, Clone)]
pub struct TargetSystemConfig{
    pub unique_id: String,
    pub connector: ADConnector,                              
    pub account_matching_rules: fn(&mut Iga, &mut TargetSystem),
    pub entitlements_ownership_rules: fn(&mut Iga, &mut TargetSystem),
    pub other_attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub struct TargetSystem {
    pub config: TargetSystemConfig,
    pub accounts: HashMap<String, Account>,
    pub entitlements: HashMap<String, Entitlement>,
}
impl TargetSystem {
    pub fn new(config: TargetSystemConfig) -> Self {
        Self {
            config,
            accounts: HashMap::new(),
            entitlements: HashMap::new(),
        }
    }
    pub fn load(&mut self) -> Result<()> {
        let users_dto = self.config.connector.load_ad_users()?;
        let entitlements_dto = self.config.connector.load_ad_groups()?;
    
        self.accounts = users_dto
            .into_iter()
            // Only key for hashmap is cloned. Data object is moved
            .map(|dto| (dto.unique_id.clone(), Account::from_dto(dto)))
            .collect();

        self.entitlements = entitlements_dto
            .into_iter()
            // Only key for hashmap is cloned. Data object is moved
            .map(|dto| (dto.unique_id.clone(), Entitlement::from_dto(dto)))
            .collect();

        self._populate_account_indirect_access();

        Ok(())
    }
    fn _populate_account_indirect_access(&mut self) {
        for acct in self.accounts.values_mut() {
            if let Some(ents) = &acct.memberof {
                for ent_uid in ents {
                    if let Some(ent) = self.entitlements.get(ent_uid) {
                        // Clone is keys and is ok, it is generating new data
                        acct.memberof_indirect.extend(ent.all_indirect_memberof.clone());
                    }
                }
                // Remove the indirect access that the account already has directly
                acct.memberof_indirect.retain(|e| !ents.contains(e));
            }
        }
    }

    pub fn add_account_history(&mut self, mut records: HashMap<String, HistoryRecord>) {
        for a in self.accounts.values_mut() {
            if let Some(record) = records.remove(&a.unique_id) {
                a.history.push(record);
            }
        }
    }
    pub fn add_entitlement_history(&mut self, mut records: HashMap<String, HistoryRecord>) {
        for a in self.entitlements.values_mut() {
            if let Some(record) = records.remove(&a.unique_id) {
                a.history.push(record);
            }
        }
    }

    pub fn get_accounts_data(&self, accts_uids: &Vec<String>) -> Vec<AccountData> {
        accts_uids.into_iter()
                    .filter_map(|uid| self.accounts.get(uid))
                    .map(|acct| acct.to_data(self) )
                    .collect()
    }
    pub fn get_orphan_accounts(&self) -> Vec<AccountData> {
        self.accounts.values()
            .filter(|a| a.is_orphan())
            .map(|acct| acct.to_data(self))
            .collect()
    }
    pub fn get_persistent_leaver_accounts(&self, idents: &HashMap<String, Identity>) -> Vec<AccountData> {
        self.accounts.values()
            .filter(|a| a.is_persistent_leaver(idents))
            .map(|acct| acct.to_data(self))
            .collect()
    }

    pub fn get_account_category_totals(&self) -> CategoryTotals {
        let mut type_counter = HashMap::new();
        for acct in self.accounts.values() {
            let counter = type_counter.entry(acct.account_type.clone()).or_insert(0);
            *counter += 1;
        }
        CategoryTotals{
            ts_uid: self.config.unique_id.clone(),
            categories: "Account type".to_string(),
            totals: type_counter,
        }
    }
    pub fn get_categorized_accounts(&self) -> CategorizedAccounts {
        let mut type_lists = HashMap::new(); 
            
        for acct in self.accounts.values() {
            let list = type_lists.entry(acct.account_type.clone()).or_insert(Vec::new());
            list.push(acct.to_data(self));
        }
        CategorizedAccounts{
            ts_uid: self.config.unique_id.clone(),
            type_lists,
        }
    }
    
    pub fn get_entitlement_category_totals(&self) -> CategoryTotals {
        let mut type_counter = HashMap::new();
        for ent in self.entitlements.values() {
            let counter = type_counter.entry(ent.entitlement_type.clone()).or_insert(0);
            *counter += 1;
        }
        CategoryTotals{
            ts_uid: self.config.unique_id.clone(),
            categories: "Entitlement type".to_string(),
            totals: type_counter,
        }
    }
    pub fn get_categorized_entitlements(&self) -> CategorizedEntitlements {
        let mut type_lists = HashMap::new(); 
            
        for ent in self.entitlements.values() {
            let list = type_lists.entry(ent.entitlement_type.clone()).or_insert(Vec::new());
            list.push(ent.to_data(false, self));
        }
        CategorizedEntitlements{
            ts_uid: self.config.unique_id.clone(),
            type_lists,
        }
    }
    pub fn get_entitlement_count_per_ou(&self) -> CategoryTotals {
        let mut type_counter = HashMap::new(); 
        for ent in self.entitlements.values() {
            let t = ent.ou.clone().unwrap_or("No OU".to_string());
            let counter = type_counter.entry(t).or_insert(0);
            *counter += 1;
        }
        CategoryTotals{
            ts_uid: self.config.unique_id.clone(),
            categories: "Count of entitlements in OU".to_string(),
            totals: type_counter,
        }
    }
    
    pub fn get_entitlements_data(&self, memberships: bool, ents_uids: &Vec<String>) -> Vec<EntitlementData> {
        
        ents_uids.into_iter()
                    .filter_map(|uid| 
                        self.entitlements.get(uid))
                    .map(|ent| {
                        ent.to_data(memberships, self)})
                    .collect()
    }

}

#[derive(Debug)]
pub struct Account {
    pub unique_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    
    pub created: Option<NaiveDate>,
    pub last_logon: Option<NaiveDate>,
    pub password_last_set: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,

    pub enabled: Option<bool>,
    pub deleted: Option<bool>,
    pub locked: Option<bool>,

    pub memberof: Option<Vec<String>>,
    pub memberof_indirect: Vec<String>,
    pub ou: Option<String>,

    pub other_attributes: HashMap<String, Option<String>>, 
    pub account_type: String,

    pub syncs_from_ts: Option<String>,
    pub syncs_from_account: Option<String>,
    pub syncs_to_ts: Option<String>,
    pub syncs_to_account: Option<String>,
    pub identity_owners: Vec<String>,

    pub history: Vec<HistoryRecord>,
}
impl Account {
    pub fn from_dto(dto: AccountDTO) -> Self {

        let mut history = Vec::new();
        if let Some(created) = dto.created.clone() {
            history.push(HistoryRecord { 
                link_key: "".to_string(), 
                date: created, 
                source: "Account data".to_string(), 
                event_name: "Account creation".to_string(), 
                initiator: "".to_string(), 
                state: "".to_string(), 
                description: "".to_string(), });
        }
        
        Self {
            unique_id: dto.unique_id,
            display_name: dto.display_name,
            description: dto.description,
    
            created: dto.created.clone(),
            last_logon: dto.last_logon,
            password_last_set: dto.password_last_set,
            expiration_date: dto.expiration_date,

            enabled: dto.enabled,
            deleted: dto.deleted,
            locked: dto.locked,

            memberof: dto.memberof,
            memberof_indirect: Vec::new(),
            ou: dto.ou,
            other_attributes: dto.other_attributes, 
            account_type: "".to_string(),
            
            syncs_from_ts: None,
            syncs_from_account: None,
            syncs_to_ts: None,
            syncs_to_account: None,
            identity_owners: Vec::new(),    
            
            history,  
        }
    }
    pub fn to_data(&self, ts: &TargetSystem) -> AccountData {

        // Clones to de-reference data for serialization
        AccountData {
            uid: self.unique_id.clone(),
            display_name: self.display_name.clone().unwrap_or(self.unique_id.clone()),
            description: self.description.clone().unwrap_or("".to_string()),
            account_type: self.account_type.clone(),
            ou: self.ou.clone().unwrap_or("".to_string()),
            enabled: self.enabled.map(|e| if e { "Yes ".to_string()} else { "No ".to_string()}).unwrap_or("N/A".to_string()),
            syncs_to_ts: self.syncs_to_ts.clone().unwrap_or("".to_string()),
            syncs_to_account: self.syncs_to_account.clone().unwrap_or("".to_string()),
            entitlements: self._get_entitlements_data(ts),
            indirect_entitlements: self._get_indirect_entitlements_data(ts),
            identity_owners: self.identity_owners.clone(),
            macheo: false,
            history: self.history.clone(),
        }
    }
    
    fn _get_entitlements_data(&self, ts: &TargetSystem) -> Vec<EntitlementData> {
        if let Some(ents_uids) = &self.memberof {
            ts.get_entitlements_data(false, ents_uids)
        } else {
            Vec::new()
        }
    }
    fn _get_indirect_entitlements_data(&self, ts: &TargetSystem) -> Vec<EntitlementData> {
        if self.memberof_indirect.len() > 0 {
            ts.get_entitlements_data(false, &self.memberof_indirect)
        } else {
            Vec::new()
        }
    }

    pub fn get_total_entitlements(&self) -> usize {
        self.memberof.as_ref().map(|memberof| memberof.len()).unwrap_or(0)
    }
    pub fn is_orphan(&self) -> bool {
        self.enabled == Some(true) && self.identity_owners.len() == 0
    }
    pub fn is_persistent_leaver(&self, idents: &HashMap<String, Identity>) -> bool {
        if self.identity_owners.len() == 0 {
            return false
        }
        self.enabled == Some(true) &&
        self.identity_owners.iter().all(|ident_uid| 
            idents.get(ident_uid).map_or(false, 
                |ident| ident.is_inactive() && ident.termination_date.map(|d| d < Local::now().naive_local().into() ).unwrap_or(false)
            ))
    }

}

#[derive(Debug)] 
pub struct Entitlement {
    pub unique_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub created: Option<NaiveDate>,

    pub memberof: Option<Vec<String>>,
    pub all_indirect_memberof: Vec<String>,
    pub members: Option<Vec<String>>,
    pub ou: Option<String>,

    pub other_attributes: HashMap<String, Option<String>>, 
    pub entitlement_type: String,

    pub syncs_from_ts: Option<String>,
    pub syncs_from_entitlement: Option<String>,
    pub syncs_to_ts: Option<String>,
    pub syncs_to_entitlement: Option<String>,
    pub identity_owners: Vec<String>,

    pub ts_owners: Option<Vec<String>>,

    pub history: Vec<HistoryRecord>,
}
impl Entitlement {
    pub fn from_dto(dto: EntitlementDTO) -> Self {

        let mut history = Vec::new();
        if let Some(created) = dto.created.clone() {
            history.push(HistoryRecord { 
                link_key: "".to_string(), 
                date: created, 
                source: "Group data".to_string(), 
                event_name: "Group creation".to_string(), 
                initiator: "".to_string(), 
                state: "".to_string(), 
                description: "".to_string(), });
        }

        Self {
            unique_id: dto.unique_id,
            display_name: dto.display_name,
            description: dto.description,
            created: dto.created,
            memberof: dto.memberof,
            all_indirect_memberof: dto.all_indirect_memberof,
            members: dto.members,
            ou: dto.ou,
            other_attributes: dto.other_attributes,
            entitlement_type: "".to_string(),

            syncs_from_ts: None,
            syncs_from_entitlement: None,
            syncs_to_ts: None,
            syncs_to_entitlement: None,
            identity_owners: Vec::new(),
            ts_owners: None,
            history,
        }
    }

    pub fn to_data(&self, memberships: bool, ts: &TargetSystem) -> EntitlementData {
        let mut member_accounts = Vec::new();
        let mut member_groups = Vec::new();
        if memberships {
            member_accounts = self._get_member_accounts_data(ts);
            member_groups = self._get_member_groups_data(ts);
        }
        // Clones to de-reference data for serialization
        EntitlementData { 
            uid: self.unique_id.clone(),
            display_name: self.display_name.clone().unwrap_or(self.unique_id.clone()),
            description: self.description.clone().unwrap_or("".to_string()),
            member_accounts,
            member_groups,
            entitlement_type: self.entitlement_type.clone(), 
            ou: self.ou.clone().unwrap_or("".to_string()),
            syncs_to_ts: self.syncs_to_ts.clone().unwrap_or("".to_string()),
            syncs_to_entitlement: self.syncs_to_entitlement.clone().unwrap_or("".to_string()),
            identity_owners: self.identity_owners.clone(),
            macheo: false,
            history: self.history.clone(),
        }
    }
    
    fn _get_member_accounts_data(&self, ts: &TargetSystem) -> Vec<AccountData> {

        if let Some(member_uids) = &self.members {
            let sams: Vec<String> = member_uids.clone().into_iter().map(|dn|
                dn.split(',')
                .next()
                .filter(|part| part.starts_with("CN="))
                .map(|part| part.trim_start_matches("CN="))
                .unwrap_or(&dn).to_string()
            ).collect();
            ts.get_accounts_data(&sams)
        } else {
            Vec::new()
        }
    }
    fn _get_member_groups_data(&self, ts: &TargetSystem) -> Vec<EntitlementData> {
        if let Some(member_uids) = &self.members {
            ts.get_entitlements_data(false, member_uids)
        } else {
            Vec::new()
        }
    }
}


#[derive(Debug, Clone)] 
pub struct HistoryRecord {
    pub link_key: String,
    pub date: NaiveDate,
    pub source: String,
    pub event_name: String,
    pub initiator: String,
    pub state: String,
    pub description: String,
}




