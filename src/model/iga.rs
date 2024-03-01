use std::collections::HashMap;
use std::mem;
use chrono::NaiveDate;
use serde::Serialize;
use anyhow::{Result, anyhow};
use crate::{connectors::identity_xlsx::IdentityXlsxConnector};
use crate::connectors::dtos::IdentityDTO;
use super::dtos::{IdentityData, AccountData, EntitlementData, CategoryTotals, CategorizedEntitlements, CategorizedAccounts};
use super::ts::{TargetSystem, TargetSystemConfig, HistoryRecord};

#[derive(Debug)] 
pub struct IgaConfig {
    identity_sources: IdentitySourceConfig,
    target_systems: Vec<TargetSystemConfig>,
    ts_sync: Option<(String, String)>,
}
impl IgaConfig {
    pub fn new(id_s: IdentitySourceConfig) -> Self {
        Self {
            identity_sources: id_s,
            target_systems: Vec::new(),
            ts_sync: None,
        }
    }
    pub fn add_target_system(&mut self, t_s: TargetSystemConfig) {
        self.target_systems.push(t_s);
    }
    pub fn add_sync(&mut self, ts_sync: (String, String)) {
        self.ts_sync = Some(ts_sync);
    }
}

#[derive(Debug, Serialize)]
pub struct IdentitySourceConfig {
    pub connector: IdentityXlsxConnector,
}

#[derive(Debug)] 
pub struct Iga {
    config: IgaConfig,

    pub identities: HashMap<String, Identity>,           
    pub target_systems: HashMap<String, TargetSystem>,   
}
impl Iga {
    pub fn new (config: IgaConfig) -> Iga {
        Iga {
            config,
            identities: HashMap::new(), 
            target_systems: HashMap::new(),
        }
    }
    pub fn load_all(&mut self) -> Result<()> {
        self._load_identities()?;
        self._load_target_systems()?;
        Ok(())
    }
    fn _load_identities(&mut self) -> Result<()> {
        let identities_read = self.config.identity_sources.connector.read_identities()?;

        let identities: HashMap<String, Identity> = identities_read
            .into_iter()
            .map(|dto| {
                // Moves data, no cloning
                let identity = Identity::from_identity_dto(dto);
                // Clone ok, only key of hashmap
                (identity.unique_id.clone(), identity)
            })
            .collect();

        self.identities = identities;
        Ok(())
    }
    fn _load_target_systems(&mut self) -> Result<()> {
        for ts_config in mem::take(&mut self.config.target_systems) { 
            let mut ts = TargetSystem::new(ts_config);
            ts.load()?;

            (ts.config.account_matching_rules)(self, &mut ts);
            (ts.config.entitlements_ownership_rules)(self, &mut ts);
            // Cloning ok, is only the key
            self.target_systems.insert(ts.config.unique_id.clone(),ts);

        } 
        Ok(())
    }

    pub fn add_account_history(&mut self, ts_uid: &str, records: HashMap<String, HistoryRecord>) -> Result<()> {
        match self.target_systems.get_mut(ts_uid) {
            Some(ts) => {
                ts.add_account_history(records);
                Ok(())
            },
            None => Err(anyhow!("Target system not found for UID: {}", ts_uid)),
        }
    }
    pub fn add_entitlement_history(&mut self, ts_uid: &str, records: HashMap<String, HistoryRecord>) -> Result<()> {
        match self.target_systems.get_mut(ts_uid) {
            Some(ts) => {
                ts.add_entitlement_history(records);
                Ok(())
            },
            None => Err(anyhow!("Target system not found for UID: {}", ts_uid)),
        }
    }

    pub fn get_ts_sync(&self) -> &Option<(String, String)> {
        &self.config.ts_sync
    }
    pub fn get_identity_data(&self, ident_uid: &str) -> Option<IdentityData> {
        self.identities.get(ident_uid).map(|identity| identity.to_data(&self.target_systems))
    }
    
    pub fn get_orphan_accounts(&self) -> Vec<(String, Vec<AccountData>)> {
        self.target_systems.values()
            .map(|ts| (ts.config.unique_id.clone(), ts.get_orphan_accounts()))
            .collect()
    }
    pub fn get_persistent_leaver_accounts(&self) -> HashMap<String, Vec<AccountData>> {
        self.target_systems.values()
            .map(|ts| (ts.config.unique_id.clone(), ts.get_persistent_leaver_accounts(&self.identities)))
            .collect()
    }
    pub fn get_entitlement_count_per_type(&self) ->  Vec<CategoryTotals> {
        self.target_systems.values()
            .map(|ts| ts.get_entitlement_category_totals())
            .collect()
    }
    pub fn get_account_count_per_type(&self) ->  Vec<CategoryTotals> {
        self.target_systems.values()
            .map(|ts| ts.get_account_category_totals())
            .collect()
    }
    pub fn get_accounts_per_type(&self) ->  Vec<CategorizedAccounts> {
        self.target_systems.values()
            .map(|ts| ts.get_categorized_accounts())
            .collect()
    }
    pub fn get_entitlement_count_per_ou(&self) ->  Vec<CategoryTotals> {
        self.target_systems.values()
            .map(|ts| ts.get_entitlement_count_per_ou())
            .collect()
    }
    pub fn get_entitlements_per_type(&self) ->  Vec<CategorizedEntitlements> {
        self.target_systems.values()
            .map(|ts| ts.get_categorized_entitlements())
            .collect()
    }

}

trait TargetSystems {
    fn get_accounts_data(&self, accts: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<AccountData>>;
}
impl TargetSystems for HashMap<String, TargetSystem> {
    fn get_accounts_data(&self, accts: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<AccountData>> {
        let mut accts_ddtos = HashMap::new();
        for (ts_uid, accts_uids) in accts {
            if let Some(ts) = self.get(ts_uid) {
                accts_ddtos.insert(ts_uid.clone(), ts.get_accounts_data(accts_uids));
            }
        }
        accts_ddtos
    }
}

#[derive(Debug)] 
pub struct Identity {
    pub unique_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub employee_no: String,  
    pub employee_type: String,
    pub enabled: Option<bool>,
    pub manager_key: String,  
    pub hire_date: Option<NaiveDate>,
    pub termination_date: Option<NaiveDate>,
    pub attributes: HashMap<String, String>,
    
    pub matched_personal_accounts: HashMap<String,Vec<String>>, 
    pub matched_owned_accounts: HashMap<String,Vec<String>>, 
    pub matched_owned_groups: HashMap<String,Vec<String>>, 
    
}
impl Identity{
    pub fn from_identity_dto (dto: IdentityDTO) -> Identity{
        Identity {
            unique_id: dto.unique_id,
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            employee_no: dto.employee_no,
            employee_type: dto.employee_type,
            enabled: dto.enabled,

            manager_key: dto.manager_key,
            hire_date: dto.hire_date,
            termination_date: dto.termination_date,
            
            matched_personal_accounts: HashMap::new(),
            matched_owned_accounts: HashMap ::new(),
            matched_owned_groups: HashMap::new(),

            attributes: dto.attributes,
        }
    }
    pub fn to_data(&self, target_systems: &HashMap<String, TargetSystem>) -> IdentityData {
        IdentityData {
            unique_id: self.unique_id.clone(),
            display_name: format!("{} {} ({})", self.first_name, self.last_name, self.unique_id),
            attributes: Vec::new(),
            personal_accounts: self.get_personal_accounts_data(target_systems),
            owned_accounts: self.get_owned_accounts_data(target_systems),
            owned_groups: self.get_owned_entitlements_data(target_systems),
        }
    } 
    fn get_personal_accounts_data(&self, target_systems: &HashMap<String, TargetSystem>) -> HashMap<String, Vec<AccountData>> {
        let accts = &self.matched_personal_accounts;
        target_systems.get_accounts_data(accts)
    }
    fn get_owned_accounts_data(&self, target_systems: &HashMap<String, TargetSystem>) -> HashMap<String, Vec<AccountData>>  {
        let accts = &self.matched_owned_accounts;
        target_systems.get_accounts_data(accts)
    }
    fn get_owned_entitlements_data(&self, target_systems: &HashMap<String, TargetSystem>) -> HashMap<String, Vec<EntitlementData>> {
        let ents = &self.matched_owned_groups;
        let mut ents_ddtos = HashMap::new();
        for (ts_uid, ents_uids) in ents {
            if let Some(ts) = target_systems.get(ts_uid) {
                let ent_list = ts.get_entitlements_data(true, ents_uids);
                ents_ddtos.insert(ts_uid.clone(), ent_list);
            }
        }
        ents_ddtos
    }
    
    pub fn is_inactive(&self) -> bool {
        if let Some(enabled) = self.enabled {
            !enabled
        } else {
            false
        }
    }
}

