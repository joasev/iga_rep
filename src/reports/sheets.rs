use std::collections::HashMap;
use rust_xlsxwriter::XlsxError;
use rust_xlsxwriter::{Worksheet, Format};
use crate::model::ts::HistoryRecord;
use crate::model::dtos::{CategoryTotals, EntitlementData, AccountData, IdentityData};
use super::reports_xlsx::ExcelReportFormat;



pub struct Sheet<'a> {
    pub worksheet: &'a mut Worksheet,
    pub format: &'a ExcelReportFormat,
}

trait XlsxPrint {
    fn print_header(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError>;
    fn print(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError>;
    fn print_histories(&self, i: u32, sheet: &mut Sheet) -> Result<u32, XlsxError>;
}

impl XlsxPrint for AccountData {
    fn print_header(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError> {
        
        sheet.worksheet.write_with_format(i, 0, ts_string, &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 1, "Account", &sheet.format.header)?;
        sheet.worksheet.set_column_width(1, 25)?;
        sheet.worksheet.write_with_format(i, 2, "Type", &sheet.format.header)?;
        sheet.worksheet.set_column_width(2, 25)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header)?;
        sheet.worksheet.set_column_width(3, 15)?;
        sheet.worksheet.write_with_format(i, 4, "Enabled", &sheet.format.header)?;
        sheet.worksheet.set_column_width(4, 15)?;
        sheet.worksheet.write_with_format(i, 5, "Identity owner(s)", &sheet.format.header)?;
        sheet.worksheet.set_column_width(5, 15)?;
        sheet.worksheet.write_with_format(i, 6, "Description", &sheet.format.header)?;
        Ok(i+1)
    }
    fn print(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError> {
        sheet.worksheet.write(i, 1, format!("{}\\{}", ts_string, &self.display_name))?;
        sheet.worksheet.write(i, 2, &self.account_type)?;
        sheet.worksheet.write(i, 3, &self.ou)?;
        sheet.worksheet.write(i, 4, &self.enabled)?; 
        sheet.worksheet.write(i, 5, &self.identity_owners.join(", "))?;
        sheet.worksheet.write(i, 6, &self.description)?;
        Ok(i+1)
    }

    fn print_histories(&self, mut i: u32, sheet: &mut Sheet) -> Result<u32, XlsxError> {
        
        // To avoid cloning, create a Vec of references, and sort the references
        let mut record_refs: Vec<&HistoryRecord> = self.history.iter().collect();
        record_refs.sort_by(|a, b| b.date.cmp(&a.date));

        // Print header
        if record_refs.len() > 0 {
            sheet.worksheet.write_with_format(i, 1, "History date", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 2, "Event name", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 3, "Source", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 4, "State", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 5, "Initiator", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 6, "Info", &sheet.format.header_secondary)?;
            sheet.worksheet.set_column_width(6, 40)?;
            i += 1;
        }

        // Print values
        for record in record_refs {
            sheet.worksheet.write_with_format(i, 1, &record.date.to_string(), &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 2, &record.event_name, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 3, &record.source, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 4, &record.state, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 5, &record.initiator, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 6, &record.description, &sheet.format.standard)?;
            i += 1;
        }
        Ok(i)
    }
}

impl XlsxPrint for EntitlementData {
    fn print_header(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError> {

        sheet.worksheet.write_with_format(i, 0, ts_string, &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 1, "Group", &sheet.format.header)?;
        sheet.worksheet.set_column_width(1, 25)?;
        sheet.worksheet.write_with_format(i, 2, "Type", &sheet.format.header)?;
        sheet.worksheet.set_column_width(2, 25)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header)?;
        sheet.worksheet.set_column_width(3, 15)?;
        sheet.worksheet.write_with_format(i, 4, "Identity owner(s)", &sheet.format.header)?;
        sheet.worksheet.set_column_width(4, 15)?;
        sheet.worksheet.write_with_format(i, 5, "", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 6, "Description", &sheet.format.header)?;
        Ok(i+1)
    }
    fn print(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError> {
        sheet.worksheet.write(i, 1, format!("{}\\{}", ts_string, &self.display_name))?;
        sheet.worksheet.write(i, 2, &self.entitlement_type)?;
        sheet.worksheet.write(i, 3, &self.ou)?;
        sheet.worksheet.write(i, 4, &self.identity_owners.join(", "))?;
        sheet.worksheet.write(i, 6, &self.description)?;
        Ok(i+1)
    }
    
    fn print_histories(&self, mut i: u32, sheet: &mut Sheet) -> Result<u32, XlsxError> {
        
         // To avoid cloning, create a Vec of references, and sort the references
        let mut record_refs: Vec<&HistoryRecord> = self.history.iter().collect();
        record_refs.sort_by(|a, b| b.date.cmp(&a.date));

        // Print header
        if record_refs.len() > 0 {
            sheet.worksheet.write_with_format(i, 1, "History date", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 2, "Event name", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 3, "Source", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 4, "State", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 5, "Initiator", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 6, "Info", &sheet.format.header_secondary)?;
            sheet.worksheet.set_column_width(6, 40)?;
            i += 1;
        }

        // Print values
        for record in record_refs {
            sheet.worksheet.write_with_format(i, 1, &record.date.to_string(), &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 2, &record.event_name, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 3, &record.source, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 4, &record.state, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 5, &record.initiator, &sheet.format.standard)?;
            sheet.worksheet.write_with_format(i, 6, &record.description, &sheet.format.standard)?;
            i += 1;
        }
        Ok(i)
    }
}

pub struct SyncedAccounts<'a> {
    from: &'a mut AccountData,
    to: Option<&'a mut AccountData>,
}
impl SyncedAccounts<'_> {
    fn print_header(&self, i: u32, sheet: &mut Sheet, ts_string: &str, sync_header: bool) -> Result<u32, XlsxError> {

        sheet.worksheet.write_with_format(i, 0, ts_string, &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 1, "Account", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 2, "Type", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 4, "Enabled", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 5, "", &sheet.format.header)?;

        if sync_header {
            sheet.worksheet.write_with_format(i, 6, "Syncs to", &sheet.format.header)?;
            sheet.worksheet.write_with_format(i, 7, "Syncs to Account", &sheet.format.header)?;
        }

        Ok(i+1)
    }
    fn print(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError> {

        sheet.worksheet.write(i, 1, format!("{}\\{}", ts_string, self.from.display_name))?;
        sheet.worksheet.write(i, 2, &self.from.account_type)?;
        sheet.worksheet.write(i, 3, &self.from.ou)?;
        sheet.worksheet.write(i, 4, &self.from.enabled)?;
        
        if let Some (acct_to) = &self.to {
            sheet.worksheet.write(i, 6, &self.from.syncs_to_ts)?;
            sheet.worksheet.write(i, 7, acct_to.display_name.clone())?;
        }
        Ok(i+1)
    }

    fn print_memberships_header(&self, i: u32, sheet: &mut Sheet, sync_header: bool) -> Result<u32, XlsxError> {
        
        sheet.worksheet.write_with_format(i, 1, "Group memberships", &sheet.format.header_secondary)?;
        sheet.worksheet.set_column_width(1, 40)?;
        sheet.worksheet.write_with_format(i, 2, "Category", &sheet.format.header_secondary)?;
        sheet.worksheet.set_column_width(2, 20)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header_secondary)?;
        sheet.worksheet.write_with_format(i, 4, "A.R.", &sheet.format.header_secondary)?;
        sheet.worksheet.write_with_format(i, 5, "Direct", &sheet.format.header_secondary)?;

        if sync_header {
            sheet.worksheet.write_with_format(i, 6, "Syncs to", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 7, "Syncs to Group", &sheet.format.header_secondary)?;
            sheet.worksheet.set_column_width(7, 40)?;
            sheet.worksheet.write_with_format(i, 8, "Direct", &sheet.format.header_secondary)?;
        }

        Ok(i+1)
    }
    fn print_memberships(&self, mut i: u32, sheet: &mut Sheet) -> Result<u32, XlsxError> {
        
        if self.from.entitlements.len() == 0 {
            sheet.worksheet.write(i, 1, "(No memberships)")?;
            i += 1;
        } else {

            // Sort memberships to print (to avoid cloning, the references are copied and serted).
            let mut sorted_refs: Vec<&EntitlementData> = self.from.entitlements.iter().collect(); 
            sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

            for membership_from in sorted_refs {
                let found = if let Some (to) = &self.to {
                     to.entitlements.iter().find(|ent| {ent.uid == membership_from.syncs_to_entitlement})
                } else { None };
                i = self.print_membership(i, sheet.worksheet, &Some(membership_from), "Direct", &found, &sheet.format.standard)?;
            }

            // Sort memberships to print (to avoid cloning, the references are copied and serted).
            let mut sorted_refs: Vec<&EntitlementData> = self.from.indirect_entitlements.iter().collect(); 
            sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

            for membership_from in sorted_refs {
                let found = if let Some (to) = &self.to {
                    to.indirect_entitlements.iter().find(|ent| {ent.uid == membership_from.syncs_to_entitlement})
                } else { None };
                i = self.print_membership(i, sheet.worksheet, &Some(membership_from), "Indirect", &found, &sheet.format.grayout_format)?;
            }
        }

        if let Some (acct_to) = &self.to    {
            for memberships_to_extra in &acct_to.entitlements {
                if memberships_to_extra.macheo == false {
                    i = self.print_membership(i, sheet.worksheet, &None, "Direct", &Some(memberships_to_extra), &sheet.format.standard)?;
                }
                
            }
        }
        if let Some (acct_to) = &self.to    {
            for memberships_to_extra in &acct_to.indirect_entitlements {
                if memberships_to_extra.macheo == false {
                    i = self.print_membership(i, sheet.worksheet, &None, "Indirect", &Some(memberships_to_extra), &sheet.format.grayout_format)?;
                }
                
            }
        }
        Ok(i+1)
    }
}
impl SyncedAccounts<'_> {
    fn print_membership(&self, mut i: u32, worksheet: &mut Worksheet, membership_from: &Option<&EntitlementData>, indirection: &str, membership_to: &Option<&EntitlementData>, format: &Format) -> Result<u32, XlsxError> {
        
        if let Some(membership_from) = membership_from {
            worksheet.write_with_format(i, 1, &membership_from.display_name, format)?;
            worksheet.write_with_format(i, 2, &membership_from.entitlement_type, format)?;
            worksheet.write_with_format(i, 3, &membership_from.ou, format)?;
            worksheet.write_with_format(i, 4, "TBD", format)?;
            worksheet.write_with_format(i, 5, indirection, format)?;
            worksheet.write_with_format(i, 6, &membership_from.syncs_to_ts, format)?;
            
        }
        
        if let Some(membership_to) = membership_to {
            worksheet.write_with_format(i, 7, &membership_to.display_name,format)?;
            worksheet.write_with_format(i, 8, indirection,format)?;
        } else {
            worksheet.write_with_format(i, 7, "",format)?;
            worksheet.write_with_format(i, 8, "Not member",format)?;
        }

        i += 1;
        Ok(i)
    }
}


pub struct SyncedEntitlements<'a> {
    from: &'a mut EntitlementData,
    to: Option<&'a mut EntitlementData>,
}
impl SyncedEntitlements<'_> {
    fn print_header(&self, i: u32, sheet: &mut Sheet, ts_string: &str, sync_header: bool) -> Result<u32, XlsxError> {
        
        sheet.worksheet.write_with_format(i, 0, ts_string, &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 1, format!("Group"), &sheet.format.header)?;
        sheet.worksheet.set_column_width(1, 40)?;
        sheet.worksheet.write_with_format(i, 2, "Type", &sheet.format.header)?;
        sheet.worksheet.set_column_width(2, 20)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 4, "", &sheet.format.header)?;
        sheet.worksheet.write_with_format(i, 5, "", &sheet.format.header)?;

        if sync_header {
            sheet.worksheet.write_with_format(i, 6, "Syncs to", &sheet.format.header)?;
            sheet.worksheet.write_with_format(i, 7, "Syncs to Group", &sheet.format.header)?;
            sheet.worksheet.set_column_width(7, 40)?;
        }

        Ok(i+1)
    }
    fn print(&self, i: u32, sheet: &mut Sheet, ts_string: &str) -> Result<u32, XlsxError> {
        
        sheet.worksheet.write(i, 1, format!("{}\\{}", ts_string, self.from.display_name))?;
        sheet.worksheet.write(i, 2, &self.from.entitlement_type)?;
        sheet.worksheet.write(i, 3, &self.from.ou)?;
        
        if let Some (acct_to_ddto) = &self.to {
            sheet.worksheet.write(i, 6, &self.from.syncs_to_ts)?;
            sheet.worksheet.write(i, 7, &acct_to_ddto.display_name)?;
        }
        
        Ok(i+1)
    }

    fn print_memberships_header(&self, i: u32, sheet: &mut Sheet, sync_header: bool) -> Result<u32, XlsxError> {
        sheet.worksheet.write_with_format(i, 1, "Member User", &sheet.format.header_secondary)?;
        sheet.worksheet.set_column_width(1, 40)?;
        sheet.worksheet.write_with_format(i, 2, "Category", &sheet.format.header_secondary)?;
        sheet.worksheet.set_column_width(2, 20)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header_secondary)?;
        sheet.worksheet.write_with_format(i, 4, "A.R.", &sheet.format.header_secondary)?;
        sheet.worksheet.write_with_format(i, 5, "Direct", &sheet.format.header_secondary)?;

        if sync_header {
            sheet.worksheet.write_with_format(i, 6, "Syncs to", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 7, "Syncs to User", &sheet.format.header_secondary)?;
            sheet.worksheet.set_column_width(7, 40)?;
            sheet.worksheet.write_with_format(i, 8, "Direct", &sheet.format.header_secondary)?;
        }

        Ok(i+1)
    }
    fn print_memberships(&self, mut i: u32, sheet: &mut Sheet) -> Result<u32, XlsxError> {
        if self.from.member_accounts.len() == 0 {
            sheet.worksheet.write(i, 1, "(No member users)")?;
            i += 1;
        } else {
            
            // Sort member accounts to print (to avoid cloning, the references are copied and serted).
            let mut sorted_refs: Vec<&AccountData> = self.from.member_accounts.iter().collect(); 
            sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

            for membership_from in sorted_refs {
                let found = if let Some (to) = &self.to {
                     to.member_accounts.iter().find(|ent| {ent.uid == membership_from.syncs_to_account})
                } else { None };
                i = self.print_membership(i, sheet.worksheet, &Some(membership_from), "Direct", &found, &sheet.format.standard)?;
            }
        }

        Ok(i)
    }
    fn print_group_memberships_header(&self, i: u32, sheet: &mut Sheet, sync_header: bool) -> Result<u32, XlsxError> {
        sheet.worksheet.write_with_format(i, 1, "Member Group", &sheet.format.header_secondary)?;
        sheet.worksheet.set_column_width(1, 40)?;
        sheet.worksheet.write_with_format(i, 2, "Category", &sheet.format.header_secondary)?;
        sheet.worksheet.set_column_width(2, 20)?;
        sheet.worksheet.write_with_format(i, 3, "OU", &sheet.format.header_secondary)?;
        sheet.worksheet.write_with_format(i, 4, "A.R.", &sheet.format.header_secondary)?;
        sheet.worksheet.write_with_format(i, 5, "Direct", &sheet.format.header_secondary)?;

        if sync_header {
            sheet.worksheet.write_with_format(i, 6, "Syncs to", &sheet.format.header_secondary)?;
            sheet.worksheet.write_with_format(i, 7, "Syncs to Group", &sheet.format.header_secondary)?;
            sheet.worksheet.set_column_width(7, 40)?;
            sheet.worksheet.write_with_format(i, 8, "Direct", &sheet.format.header_secondary)?;
        }

        Ok(i+1)
    }
    fn print_group_memberships(&self, mut i: u32, sheet: &mut Sheet) -> Result<u32, XlsxError> {
        if self.from.member_groups.len() == 0 {
            sheet.worksheet.write(i, 1, "(No group members)")?;
        } else {
            
            // Sort member groups to print (to avoid cloning, the references are copied and serted).
            let mut sorted_refs: Vec<&EntitlementData> = self.from.member_groups.iter().collect(); 
            sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

            for membership_from in sorted_refs {
                let found = if let Some (to) = &self.to { 
                     to.member_groups.iter().find(|ent| {ent.uid == membership_from.syncs_to_entitlement})
                } else { None };
                i = self.print_group_membership(i, sheet.worksheet, &Some(membership_from), "Direct", &found, &sheet.format.standard)?;
            }
        }

        Ok(i+1)
    }

}
impl SyncedEntitlements<'_> {
    fn print_membership(&self, mut i: u32, worksheet: &mut Worksheet, membership_from: &Option<&AccountData>, indirection: &str, membership_to: &Option<&AccountData>, format: &Format) -> Result<u32, XlsxError> {
        
        if let Some(membership_from) = membership_from {
            worksheet.write_with_format(i, 1, &membership_from.display_name, format)?;
            worksheet.write_with_format(i, 2, &membership_from.account_type, format)?;
            worksheet.write_with_format(i, 3, &membership_from.ou, format)?;
            worksheet.write_with_format(i, 4, "TBD", format)?;
            worksheet.write_with_format(i, 5, indirection, format)?;
            worksheet.write_with_format(i, 6, &membership_from.syncs_to_ts, format)?;
            
        }
        
        if let Some(membership_to) = membership_to {
            worksheet.write_with_format(i, 7, membership_to.display_name.clone(),format)?;
            worksheet.write_with_format(i, 8, indirection,format)?;
        } else {
            worksheet.write_with_format(i, 7, "",format)?;
            worksheet.write_with_format(i, 8, "Not member",format)?;
        }

        i += 1;
        Ok(i)
    }
    fn print_group_membership(&self, mut i: u32, worksheet: &mut Worksheet, membership_from: &Option<&EntitlementData>, indirection: &str, membership_to: &Option<&EntitlementData>, format: &Format) -> Result<u32, XlsxError> {
        
        if let Some(membership_from) = membership_from {
            worksheet.write_with_format(i, 1, &membership_from.display_name, format)?;
            worksheet.write_with_format(i, 2, &membership_from.entitlement_type, format)?;
            worksheet.write_with_format(i, 3, &membership_from.ou, format)?;
            worksheet.write_with_format(i, 4, "TBD", format)?;
            worksheet.write_with_format(i, 5, indirection, format)?;
            worksheet.write_with_format(i, 6, &membership_from.syncs_to_ts, format)?;
            
        }
        
        if let Some(membership_to) = membership_to {
            worksheet.write_with_format(i, 7, &membership_to.display_name,format)?;
            worksheet.write_with_format(i, 8, indirection,format)?;
        } else {
            worksheet.write_with_format(i, 7, "",format)?;
            worksheet.write_with_format(i, 8, "Not member",format)?;
        }

        i += 1;
        Ok(i)
    }

}


pub struct SyncedAccountSet<'a> {
    sync: Option<(&'a str, &'a str)>,
    synced_from_to: HashMap<String, Vec<SyncedAccounts<'a>>>,
    nosync_to: Vec<&'a mut AccountData>,
}
impl<'a> SyncedAccountSet<'a> {
    pub fn from(mut accts: HashMap<String, Vec<&'a mut AccountData>>, sync: &'a Option<(String, String)>) -> Self {

        let mut set = SyncedAccountSet::default();

        if let Some((sync_from_ts_uid, sync_to_ts_uid)) = sync {
            if accts.contains_key(sync_from_ts_uid) && accts.contains_key(sync_to_ts_uid) {

                set.sync = Some((sync_from_ts_uid, sync_to_ts_uid));

                let from = accts.remove(sync_from_ts_uid).unwrap();
                let to = accts.remove(sync_to_ts_uid).unwrap();

                set._add_with_sync(
                    (sync_from_ts_uid, from), 
                    (sync_to_ts_uid, to));

            }
        } 
        set._add_without_sync(accts);
        set
    }
    
    fn _add_with_sync(&mut self, from_accts: (&'a String, Vec<&'a mut AccountData>), to_accts: (&'a String, Vec<&'a mut AccountData>)) {
        
        let mut to_acctss_hm: HashMap<String, &'a mut AccountData> = to_accts.1.into_iter().map(|acct| (acct.uid.clone(), acct)).collect();
        
        let mut syncs = Vec::new();
        for from_acct in from_accts.1 {
            let mut to_acct = to_acctss_hm.remove(&from_acct.syncs_to_account);

            // Identify direct memberships with no sync-source
            if let Some(to_acct) = &mut to_acct {
                for from_ent in &from_acct.entitlements {
                    for to_ent in &mut to_acct.entitlements { 
                        if from_ent.syncs_to_entitlement == to_ent.uid {
                            to_ent.macheo = true;
                        }
                    }
                }
            }

            // Identify indirect memberships with no sync-source
            if let Some(to_acct) = &mut to_acct {
                for from_ent in &from_acct.indirect_entitlements {
                    for to_ent in &mut to_acct.indirect_entitlements { 
                        if from_ent.syncs_to_entitlement == to_ent.uid {
                            to_ent.macheo = true;
                        }
                    }
                }
            }

            syncs.push(SyncedAccounts {
                from: from_acct,
                to: to_acct,
            });
        }

        let mut synced_accts = HashMap::new();
        synced_accts.insert(from_accts.0.clone(), syncs);

        self.synced_from_to = synced_accts;
        self.nosync_to = to_acctss_hm.into_values().collect();
    
    }
    fn _add_without_sync(&mut self, rest: HashMap<String, Vec<&'a mut AccountData>>) {

        for (ts, accts) in rest {
            let sa = accts.into_iter()
                .map(|acct| SyncedAccounts {
                    from: acct,
                    to: None,
                }).collect();
            self.synced_from_to.insert(ts, sa);
        }

    }
    
    pub fn print(&mut self, mut i: u32, sheet: &mut Sheet, print_memberships: bool) -> Result<u32, XlsxError>  {

        // Print indicated synced ts first
        if let Some((sync_from_ts_uid,_)) = self.sync {
            if let Some(synced_from_to) = self.synced_from_to.remove(sync_from_ts_uid) {

                if print_memberships {
                    i = SyncedAccountSet::_print_with_memberships(i, sheet, sync_from_ts_uid, &synced_from_to, true)?;
                } else {
                    i = SyncedAccountSet::_print(i, sheet, sync_from_ts_uid, &synced_from_to, true)?;
                }
                
            }
        }

        // Then print the rest
        for (sync_from_ts_uid, synced_from_to) in &self.synced_from_to {

            if print_memberships {
                i = SyncedAccountSet::_print_with_memberships(i, sheet, sync_from_ts_uid, synced_from_to, false)?;
            } else {
                i = SyncedAccountSet::_print(i, sheet, sync_from_ts_uid, synced_from_to, false)?;
            }

        }
        
        Ok(i)
    }
    fn _print(mut i: u32, sheet: &mut Sheet, sync_from_ts_uid: &str, synced_from_to: &Vec<SyncedAccounts>, sync_header: bool) -> Result<u32, XlsxError>  {
        
        // Print header
            i = synced_from_to[0].print_header(i, sheet, sync_from_ts_uid, sync_header)?; 
        
            // Sort accounts to print (to avoid cloning, the references are copied and serted).
            let mut sorted_refs: Vec<&SyncedAccounts> = synced_from_to.iter().collect(); 
            sorted_refs.sort_by(|a, b| a.from.display_name.to_lowercase().cmp(&b.from.display_name.to_lowercase()));

            // Print accounts' data
            for synced_acct in sorted_refs {
                i = synced_acct.print(i, sheet, sync_from_ts_uid)?;
            }
        
        Ok(i)
    }
    fn _print_with_memberships(mut i: u32, sheet: &mut Sheet, sync_from_ts_uid: &str, synced_from_to: &Vec<SyncedAccounts>, sync_header: bool) -> Result<u32, XlsxError>  {

        // Sort accounts to print (to avoid cloning, the references are copied and sorted).
        let mut sorted_refs: Vec<&SyncedAccounts> = synced_from_to.iter().collect(); 
        sorted_refs.sort_by(|a, b| a.from.display_name.to_lowercase().cmp(&b.from.display_name.to_lowercase()));

        // Print accounts' headers and data
        for synced_acct in sorted_refs {
            i = synced_acct.print_header(i, sheet, sync_from_ts_uid,sync_header)?;
            i = synced_acct.print(i, sheet, sync_from_ts_uid)?;
            
            i = synced_acct.print_memberships_header(i, sheet, sync_header)?;
            i = synced_acct.print_memberships(i, sheet)?;
            i += 1;
        }

        Ok(i)
    }
}
impl Default for SyncedAccountSet<'_> {
    fn default() -> Self {
        Self { 
            sync: Default::default(), 
            synced_from_to: Default::default(), 
            nosync_to: Default::default(), 
        }
    }
}

pub struct SyncedEntitlementSet<'a> {
    sync: Option<(&'a str, &'a str)>,
    synced_from_to: HashMap<String, Vec<SyncedEntitlements<'a>>>,
    nosync_to: Vec<&'a mut EntitlementData>,
}
impl<'a> SyncedEntitlementSet<'a> {
    pub fn from(mut ents: HashMap<String, Vec<&'a mut EntitlementData>>, sync: &'a Option<(String, String)>) -> Self {

        let mut set = SyncedEntitlementSet::default();

        if let Some((sync_from_ts_uid, sync_to_ts_uid)) = sync {
            if ents.contains_key(sync_from_ts_uid) && ents.contains_key(sync_to_ts_uid) {
                
                set.sync = Some((sync_from_ts_uid, sync_to_ts_uid));

                let from = ents.remove(sync_from_ts_uid).unwrap();
                let to = ents.remove(sync_to_ts_uid).unwrap();

                set._add_with_sync(
                    (sync_from_ts_uid, from), 
                    (sync_to_ts_uid, to));
                
            }
        }
        set._add_without_sync(ents);
        set
    }
    fn _add_with_sync(&mut self, from_ents: (&'a String, Vec<&'a mut EntitlementData>), to_ents: (&'a String, Vec<&'a mut EntitlementData>)) {
        
        let mut to_ents_hm: HashMap<String, &'a mut EntitlementData> = to_ents.1.into_iter().map(|ent| (ent.uid.clone(), ent)).collect();

        let mut syncs = Vec::new();
        for from_ent in from_ents.1 {
            let mut to_ent = to_ents_hm.remove(&from_ent.syncs_to_entitlement);

            // Identify memberships with no sync-source
            if let Some(to_ent) = &mut to_ent {
                for from_mem in &from_ent.member_accounts {
                    for to_mem in &mut to_ent.member_accounts { 
                        if from_mem.syncs_to_account == to_mem.uid {
                            to_mem.macheo = true;
                        }
                    }
                }
            }

            syncs.push(SyncedEntitlements {
                from: from_ent,
                to: to_ent,
            });
        }

        let mut synced_ents = HashMap::new();
        synced_ents.insert(from_ents.0.clone(), syncs);

        self.synced_from_to = synced_ents;
        self.nosync_to = to_ents_hm.into_values().collect();
    
    }
    fn _add_without_sync(&mut self, rest: HashMap<String, Vec<&'a mut EntitlementData>>) {

        for (ts, ents) in rest {
            let se = ents.into_iter()
                .map(|ent| SyncedEntitlements {
                    from: ent,
                    to: None,
                }).collect();
            self.synced_from_to.insert(ts, se);
        }

    }
    
    pub fn print(&mut self, mut i: u32, sheet: &mut Sheet, print_memberships: bool) -> Result<u32, XlsxError>  {
        
        // Print indicated synced ts first
        if let Some((sync_from_ts_uid,_)) = self.sync {
            if let Some(synced_from_to) = self.synced_from_to.remove(sync_from_ts_uid) {

                if print_memberships {
                    i = SyncedEntitlementSet::_print_with_memberships(i, sheet, sync_from_ts_uid, &synced_from_to, true)?;
                } else {
                    i = SyncedEntitlementSet::_print(i, sheet, sync_from_ts_uid, &synced_from_to, true)?;
                }
                
            }
        }
        
        // Then print the rest
        for (sync_from_ts_uid, synced_from_to) in &self.synced_from_to {

            if print_memberships {
                i = SyncedEntitlementSet::_print_with_memberships(i, sheet, sync_from_ts_uid, synced_from_to, false)?;
            } else {
                i = SyncedEntitlementSet::_print(i, sheet, sync_from_ts_uid, synced_from_to, false)?;
            }

        }
        
        Ok(i)
    }
    
    fn _print(mut i: u32, sheet: &mut Sheet, sync_from_ts_uid: &str, synced_from_to: &Vec<SyncedEntitlements>, sync_header: bool) -> Result<u32, XlsxError>  {
        
        // Print header
        i = synced_from_to[0].print_header(i, sheet, sync_from_ts_uid, sync_header)?;
                
        // Sort entitlements to print (to avoid cloning, the references are copied and serted).
        let mut sorted_refs: Vec<&SyncedEntitlements> = synced_from_to.iter().collect(); 
        sorted_refs.sort_by(|a, b| a.from.display_name.to_lowercase().cmp(&b.from.display_name.to_lowercase()));

        // Print entitlements
        for synced_ent in sorted_refs {
            i = synced_ent.print(i, sheet, sync_from_ts_uid)?;
        }
        
        Ok(i)
    }
    
    fn _print_with_memberships(mut i: u32, sheet: &mut Sheet, sync_from_ts_uid: &str, synced_from_to: &Vec<SyncedEntitlements>, sync_header: bool) -> Result<u32, XlsxError>  {

        // Sort entitlements to print (to avoid cloning, the references are copied and serted).
        let mut sorted_refs: Vec<&SyncedEntitlements> = synced_from_to.iter().collect(); 
        sorted_refs.sort_by(|a, b| a.from.display_name.to_lowercase().cmp(&b.from.display_name.to_lowercase()));

        for synced_ent in sorted_refs {
            i = synced_ent.print_header(i, sheet, sync_from_ts_uid, sync_header)?;
            i = synced_ent.print(i, sheet, sync_from_ts_uid)?;
            i = synced_ent.print_memberships_header(i, sheet, sync_header)?;
            i = synced_ent.print_memberships(i, sheet)?;
            i = synced_ent.print_group_memberships_header(i, sheet, sync_header)?;
            i = synced_ent.print_group_memberships(i, sheet)?;
            i += 1;
        }
        

        // PRINT ACA LOS nosync_to
        Ok(i)
    }
}
impl Default for SyncedEntitlementSet<'_> {
    fn default() -> Self {
        Self { 
            sync: Default::default(), 
            synced_from_to: Default::default(), 
            nosync_to: Default::default(),  }
    }
}

pub struct IdentitySummaryPrinter<'a> {
    identity: &'a mut IdentityData,
    sync: &'a Option<(String, String)>,
}
impl<'a> IdentitySummaryPrinter<'a> {
    pub fn from (identity: &'a mut IdentityData, sync: &'a Option<(String, String)>) -> Self {
        Self { 
            identity,
            sync,
        }
    }
    pub fn print(&mut self, sheet: &mut Sheet) -> Result<(), XlsxError>  { 

        let mut i = 0;

        i = SyncedAccountSet::from(self.identity.get_personal_accounts_mut(), self.sync) 
            .print(i, sheet, false)?; 
        
        i = SyncedAccountSet::from(self.identity.get_owned_accounts_mut(), self.sync) 
            .print(i, sheet, false)?; 
        
        SyncedEntitlementSet::from(self.identity.get_owned_groups_mut(), self.sync) 
            .print(i, sheet, false)?;
        Ok(())
    }
}

pub struct AccountSet<'a>(pub HashMap<String, Vec<&'a AccountData>>);
impl AccountSet<'_> {
    pub fn print(&mut self, sheet: &mut Sheet) -> Result<(), XlsxError>  {

        let mut i = 0;

        for (ts, accts) in &self.0 {
            
            if self.0.len() > 0 {
                // Print header
                i = accts[0].print_header(i, sheet, ts)?;

                // Sorting accounts to print (to avoid cloning, the references are copied and sorted).
                let mut sorted_refs = accts.clone(); //Cloning references
                sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

                // Print accounts' data
                for acct in sorted_refs {
                    i = acct.print(i, sheet, &ts)?;
                }
            }

            
        }
        
        Ok(())
    }
    pub fn print_with_history(&mut self, sheet: &mut Sheet) -> Result<(), XlsxError> {

        let mut i = 0;

        for (ts, accts) in &self.0 {

            // Sorting accounts to print (to avoid cloning, the references are copied and sorted).
            let mut sorted_refs = accts.clone(); //Cloning references
            sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

            // Print headers and accounts
            for acct in sorted_refs {
                i = acct.print_header(i, sheet, ts)?;
                i = acct.print(i, sheet, &ts)?;
    
                i = acct.print_histories(i, sheet)?;
                i += 1;
            }
        }
        Ok(())
    }
}

pub struct EntitlementSet<'a>(pub HashMap<String, Vec<&'a EntitlementData>>);
impl EntitlementSet<'_> {
    pub fn print(&mut self, sheet: &mut Sheet) -> Result<(), XlsxError>  {

        let mut i = 0;

        for (ts, ents) in &self.0 {
            
            if ents.len() > 0 {
                // Print header
                i = ents[0].print_header(i, sheet, ts)?;

                // Sorting entitlements to print (to avoid cloning, the references are copied and sorted).
                let mut sorted_refs = ents.clone(); //Cloning references
                sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));
                
                // Print entitlements' data
                for ent in sorted_refs {
                    i = ent.print(i, sheet, ts)?;
                }
            }

            
        }
        
        Ok(())
    }
    pub fn print_with_history(&mut self, sheet: &mut Sheet) -> Result<(), XlsxError> {

        let mut i = 0;

        for (ts, ents) in &self.0 {

            // Sorting entitlements to print (to avoid cloning, the references are copied and sorted).
            let mut sorted_refs = ents.clone(); //Cloning references
            sorted_refs.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));
        
            // Print headers and entitlements
            for ent in sorted_refs {
                i = ent.print_header(i, sheet, ts)?;
                i = ent.print(i, sheet, ts)?;
    
                i = ent.print_histories(i, sheet)?;
                i += 1;
            }
        }
        Ok(())
    }
   
}

pub struct TotalsSheet<'a>(pub &'a CategoryTotals);
impl<'a> TotalsSheet<'a> {
    pub fn print(&mut self, sheet: &mut Sheet) -> Result<(), XlsxError>  {
        sheet.worksheet.write_with_format(0, 0, &self.0.categories, &sheet.format.header)?;
        sheet.worksheet.set_column_width(0, 25)?;
        sheet.worksheet.write_with_format(0, 1, "Amount", &sheet.format.header)?;
        let mut i = 1;
        for total in &self.0.totals {
            sheet.worksheet.write(i, 0, total.0)?; 
            sheet.worksheet.write(i, 1, *total.1)?;
            i += 1;
        }
        Ok(())
    }
}

