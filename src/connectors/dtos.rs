use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct IdentityDTO {
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

}
impl Default for IdentityDTO {
    fn default () -> Self {
        Self {
            unique_id: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            employee_no: String::new(),
            employee_type: String::new(),
            enabled: None,
            manager_key: String::new(),
            hire_date: None,
            termination_date: None,

            attributes: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct AccountDTO {
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
    pub ou: Option<String>,
    pub other_attributes: HashMap<String, Option<String>>, 

}

#[derive(Debug)] 
pub struct EntitlementDTO {
    pub unique_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    
    pub created: Option<NaiveDate>,
    pub memberof: Option<Vec<String>>,
    pub all_indirect_memberof: Vec<String>,
    pub members: Option<Vec<String>>,
    pub member_groups: Option<Vec<String>>,
    pub ou: Option<String>,

    pub other_attributes: HashMap<String, Option<String>>, 
    pub ts_owners: Option<Vec<String>>,
}


