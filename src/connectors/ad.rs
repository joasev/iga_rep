use std::{fs, collections::{HashMap, HashSet}};
use chrono::{NaiveDateTime, NaiveDate};
use serde_json::Value;
use anyhow::{Result, Context};

use super::dtos::{AccountDTO, EntitlementDTO};

#[derive(Debug, Clone)] 
pub struct ADConnector {
    pub ad_users_fp: String,
    pub ad_groups_fp: String,

    pub ad_user_attributes: AdUserAttributes,
    pub ad_group_attributes: AdGroupAttributes,
}
impl ADConnector {
    pub fn load_ad_users(&self) -> Result<Vec<AccountDTO>> { 
        self._read_ad_users()
    }
    fn _read_ad_users(&self) -> Result<Vec<AccountDTO>> {

        // Read the file content into a String
        let file_content = fs::read_to_string(&self.ad_users_fp)
            .with_context(|| format!("Failed to read file: {}", &self.ad_users_fp))?;

        let all_objects: Vec<HashMap<String, Value>> = serde_json::from_str(&file_content)
            .with_context(|| format!("Failed to deserialize json after opening file: {}", &self.ad_users_fp))?;
       
        // Unconventional use of serde to allow for configurable import formats
        let accounts: Vec<AccountDTO> = all_objects
            .into_iter()
            .map(|user_values| {
                self._parse_ad_user(&user_values)
            })
            .collect();
        Ok(accounts)
    }
    fn _parse_ad_user(&self, user_data: &HashMap<String, Value>) -> AccountDTO {
        let user_attribs = &self.ad_user_attributes;

        let unique_id = user_data.get_string(&user_attribs.unique_id).unwrap_or("No ID".to_string()); // Preferable to continue with partial data
        let description = user_data.get_string(&user_attribs.description);
        let created = user_data.get_datetime(&user_attribs.created); 
        let last_logon = user_data.get_datetime(&user_attribs.last_logon); 
        let password_last_set = user_data.get_datetime(&user_attribs.password_last_set); 
        let expiration_date = user_data.get_datetime(&user_attribs.expiration_date); 
        let enabled = user_data.get_bool(&user_attribs.enabled);
        let deleted = user_data.get_bool(&user_attribs.deleted);
        let locked = user_data.get_bool(&user_attribs.locked);

        let memberof = user_data.get_string_list(&user_attribs.memberof);
        let ou = user_data.get_string(&user_attribs.ou);

        let other_attributes: HashMap<String, Option<String>> = user_attribs.other_attributes
                                            .iter()
                                            .map(|k| (k.clone(), user_data.get_string(&Some(k.clone())) ))
                                            .collect();
        
        let mut a = AccountDTO { unique_id, display_name: None, description, created, last_logon, password_last_set, expiration_date, enabled, deleted, locked, memberof, ou, other_attributes };
        let set_display_name = user_attribs.display_name_fn;
        set_display_name(&mut a);
        a
    }
    
    pub fn load_ad_groups(&self) -> Result<Vec<EntitlementDTO>> { 
        let mut ents = self._read_ad_groups()?;

        // Post-processing (populate memberof)
        if self.ad_group_attributes.member_groups.is_some() {
            ADConnector::_calculate_memberof(&mut ents);
        }
        
        // Post-processing (nested groups crawling)
        let ents = ADConnector::_calculate_indirect_memberof(ents);
        Ok(ents)
    }
    fn _read_ad_groups(&self) -> Result<Vec<EntitlementDTO>> {

        // Read the file content into a String
        let file_content = fs::read_to_string(&self.ad_groups_fp)
            .with_context(|| format!("Failed to read file: {}", &self.ad_groups_fp))?;
    
        let all_objects: Vec<HashMap<String, Value>> = serde_json::from_str(&file_content)
            .with_context(|| format!("Failed to deserialize json after opening file: {}", &self.ad_groups_fp))?;
        
        // Unconventional use of serde to allow for configurable import formats
        let ents: Vec<EntitlementDTO> = all_objects
            .into_iter()
            .map(|group_values| self._parse_ad_group(&group_values))
            .collect();
        Ok(ents)
    }
    fn _parse_ad_group(&self, group_data: &HashMap<String, Value>) -> EntitlementDTO {
        let group_attribs = &self.ad_group_attributes;

        let unique_id = group_data.get_string(&group_attribs.unique_id).unwrap_or("No ID".to_string()); // Preferable to continue with partial data
        let description = group_data.get_string(&group_attribs.description);
        let created = group_data.get_datetime(&group_attribs.created); 

        let memberof = group_data.get_string_list(&group_attribs.memberof);
        let members = group_data.get_string_list(&group_attribs.members);
        let member_groups = group_data.get_string_list(&group_attribs.member_groups);
        let ou = group_data.get_string(&group_attribs.ou);
        let ts_owners = group_data.get_string_list(&group_attribs.ts_owners);
        
        let other_attributes: HashMap<String, Option<String>> = group_attribs.other_attributes
            .iter()
            .map(|k| (k.clone(), group_data.get_string(&Some(k.clone())) ))
            .collect();

        let mut e = EntitlementDTO { unique_id, display_name: None, description, created, memberof, all_indirect_memberof: Vec::new(), members, member_groups, ou, other_attributes, ts_owners };
        let set_display_name = group_attribs.display_name_fn;
        set_display_name(&mut e);
        e
    }
    fn _calculate_indirect_memberof(ents: Vec<EntitlementDTO>) -> Vec<EntitlementDTO> {
        
        // Consumes and transforms ents into a lookup structure
        let mut ent_lookup = EntitlementLookup::from(ents);
        
        // HashMap of ent_uid and all indirect parents found
        let mut indirect_parents_found: HashMap<String, Vec<String>> = HashMap::new(); 

        // For each EntitlementDTO, it crawls for indirect memberships
        for ent in ent_lookup.0.values() {

            // Set structure to stop recurssion by not crawling on parents already found
            let mut set: HashSet<&String> = HashSet::new();
            if let Some(parents) = &ent.memberof {
                let mut ps: Vec<&String> = parents.iter().collect(); // &Vec<String> to Vec<&String>
                while ps.len() > 0 {
                    let mut new_insertions = Vec::new();
                    for p in ps {
                        let p_ref = p;
                        if set.insert(p){
                            new_insertions.push(p_ref);
                        }
                    }
                    ps = ent_lookup.get_parents_of_all_groups(new_insertions); 
                }
            } 
            // Storing result of crawling for specific EntitlementDTO. Clones only necessary data to be stored
            indirect_parents_found.insert(ent.unique_id.clone(), set.into_iter().map(|e| e.clone()).collect());
        }

        // Move found data to EntitlementsDTO
        for ent in ent_lookup.0.values_mut() {
            if let Some(all_indirect_parents) = indirect_parents_found.remove(&ent.unique_id){
                ent.all_indirect_memberof = all_indirect_parents; 
            }
        }
        let ret_val: Vec<EntitlementDTO> = ent_lookup.0.drain().map(|(_,v)| v).collect();
        ret_val
    }
    fn _calculate_memberof(ents: &mut Vec<EntitlementDTO>) {
        let mut group_memberofs = HashMap::new(); 
            for e in ents.iter() {
                if let Some(mem_groups) = &e.member_groups {
                    for mem_group in mem_groups {
                        // Creates HashMap of all memberof found so far. Clone is fine, this is new data being created
                        let entry = group_memberofs.entry(mem_group.clone()).or_insert(HashSet::from([e.unique_id.clone()]));
                        entry.insert(e.unique_id.clone());
                    }
                }
            }
            for e in ents {
                let memberof_hm = group_memberofs.remove(&e.unique_id);
                //Converts HashSet to Vec
                e.memberof = memberof_hm.map(|hs| hs.into_iter().collect());
            }
    }

}

#[derive(Debug, Clone)]
pub struct AdUserAttributes {
    pub description: Option<String>,
    pub unique_id: Option<String>,

    pub created: Option<String>,
    pub last_logon: Option<String>,
    pub password_last_set: Option<String>,
    pub expiration_date: Option<String>,

    pub enabled: Option<String>,
    pub deleted: Option<String>,
    pub locked: Option<String>,

    pub memberof: Option<String>,
    pub ou: Option<String>,
    pub other_attributes: Vec<String>, 

    pub display_name_fn: fn(&mut AccountDTO),
}

#[derive(Debug, Clone)] 
pub struct AdGroupAttributes{
    pub unique_id: Option<String>,
    pub description: Option<String>,
    
    pub created: Option<String>,
    pub memberof: Option<String>,
    pub members: Option<String>,
    pub member_groups: Option<String>,
    pub ou: Option<String>,

    pub other_attributes: Vec<String>, 
    pub ts_owners: Option<String>,
    
    pub display_name_fn: fn(&mut EntitlementDTO),
}

struct EntitlementLookup(HashMap<String, EntitlementDTO>);
impl EntitlementLookup {
    fn from(ents: Vec<EntitlementDTO>) -> Self {
        let mut el = EntitlementLookup(HashMap::new());
        for ent in ents {
            el.0.insert(ent.unique_id.clone(), ent);
        }
        el
    }
    fn get_parents_of_all_groups(&self, ents_uids: Vec<&String>) -> Vec<&String> {
        let mut accumulator = Vec::new();
        for indirect_ents_uid in ents_uids {
            if let Some(indirect_membership) = self.0.get(indirect_ents_uid) {
                if let Some(parents) = &indirect_membership.memberof {
                    accumulator.extend(parents.iter().collect::<Vec<&String>>()); 
                    //To capture history. Instead of Vec<String>.. Vec<String(original),Vec<String(path)>
                }
            }
        }
        accumulator
    }
}
        
trait ValueHashMap {
    fn get_string(&self, key: &Option<String>) -> Option<String>;
    fn get_datetime(&self, key: &Option<String>) -> Option<NaiveDate>;
    fn get_bool(&self, key: &Option<String>) -> Option<bool>;
    fn get_string_list(&self, key: &Option<String>) -> Option<Vec<String>>;
}
impl ValueHashMap for HashMap<String, Value> {
    fn get_string(&self, key: &Option<String>) -> Option<String> {
        let value;
        if let Some(k) = key {
            value = self.get(k)
                .and_then(Value::as_str) // Extracts &str if the Value is a string
                .map(|s| s.to_string()); // Converts &str to String

        } else {
            value = None;
        }
        value
    }
    fn get_datetime(&self, key: &Option<String>) -> Option<NaiveDate> {
        // Return None if key is None
        let key = match key {
            Some(k) => k,
            None => return None,
        };
        // Return None if no value associated with the key
        let value_str = match self.get(key).and_then(Value::as_str) {
            Some(value) => value,
            None => return None,
        };
        // Attempt to extract and parse the timestamp
        let timestamp = value_str.trim_start_matches("/Date(")
                                  .trim_end_matches(")/")
                                  .parse::<i64>();
        // Preferable to continue with missing data than stopping with error.
        let timestamp = match timestamp {
            Ok(t) => t,
            Err(_) => return None,
        };
    
        NaiveDateTime::from_timestamp_opt(timestamp / 1000, 0)
            .and_then(|dt| Some(dt.date()))
    }
    fn get_bool(&self, key: &Option<String>) -> Option<bool> {
        if let Some(k) = key {
            self.get(k)
                .and_then(Value::as_bool) // Extracts bool if the Value is a bool
        } else {
            None
        }
    }
    fn get_string_list(&self, key: &Option<String>) -> Option<Vec<String>> {
        let value: Vec<String>;
        if let Some(k) = key {
            if let Some(Value::Array(groups)) = self.get(k) {
                value = groups.iter()
                    .filter_map(|val| val.as_str())
                    .map(|s| s.to_string())
                    .collect();
            } else {
                // Preferable to continue with missing data
                value = Vec::new()  
            };
        } else {
            return None;
        }
        Some(value)
    }
}

