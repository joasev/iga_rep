use std::{collections::HashMap};
use anyhow::{Result, anyhow};

use rust_xlsxwriter::{Workbook, Format, XlsxError, FormatAlign, Color};
use crate::model::{iga::Iga, dtos::{AccountData, EntitlementData, CategoryTotals, IdentityData}};

use super::sheets::{Sheet, TotalsSheet, AccountSet, EntitlementSet, IdentitySummaryPrinter, SyncedAccountSet, SyncedEntitlementSet};


pub struct ExcelReportGenerator<'a> {
    iga: &'a Iga,
}
impl<'a> ExcelReportGenerator<'a>{
    pub fn new(data: &'a Iga) -> Self {
        Self {
            iga: data,
        }
    }
    pub fn create_identity_report(&self, ident_uid: &str) -> Result<()> {
        let mut ef = ExcelFileBuilder::new();
        let ident = self.iga.get_identity_data(ident_uid);
        let sync = self.iga.get_ts_sync();
        if let Some(mut ident_data) = ident { 
            ef.add_sheet("Summary",                    
            SheetType::IdentitySummary{identity: &mut ident_data, sync})?;
            ef.add_sheet("All personal access", 
                SheetType::AccountsFull{accounts: ident_data.get_personal_accounts_mut(), sync})?;
            ef.add_sheet("Owned accounts' access",                  
                SheetType::AccountsFull{accounts: ident_data.get_owned_accounts_mut(), sync})?;
            ef.add_sheet("Owned groups (memberships)",                
                SheetType::EntitlementsFull{entitlements: ident_data.get_owned_groups_mut(), sync})?;
            ef.add_sheet("Owned accounts (history)",                    
                SheetType::AccountsListHistory{accounts: ident_data.get_owned_accounts_ref()})?;
            ef.add_sheet("Owned groups (history)", 
                SheetType::EntitlementListHistory{entitlements: ident_data.get_owned_groups_ref()})?;
            
            ef.save(&ident_data.unique_id)?;
            Ok(())
        } else {
            return Err(anyhow!("Identity not found for UID: {}", ident_uid));
        }
        
    }
    pub fn cr_entitlement_type_totals(&self) -> Result<()> {
        let mut ef = ExcelFileBuilder::new();
        for category_totals in self.iga.get_entitlement_count_per_type() {
            ef.add_sheet(&category_totals.ts_uid, SheetType::Totals{totals: &category_totals})?;
        }
        ef.save("Entitlement type totals")?;
        Ok(())
    }
    pub fn cr_entitlement_type_lists(&self) -> Result<()> {
        for categorized_ents in self.iga.get_entitlements_per_type() {
            let mut ef = ExcelFileBuilder::new();
            for (category, v) in categorized_ents.type_lists {   // Change to eference!! &
                let mut data = HashMap::new();
                data.insert(category.clone(), v.iter().collect());
                ef.add_sheet(&category, SheetType::EntitlementList{data})?;
            }
            ef.save(&format!("Entitlement categorization - {}",&categorized_ents.ts_uid))?;
        }
        Ok(())
    }

    pub fn cr_entitlements_in_ou(&self) -> Result<()> {
        let mut ef = ExcelFileBuilder::new();
        for category_totals in self.iga.get_entitlement_count_per_ou() {
            ef.add_sheet(&category_totals.ts_uid, SheetType::Totals{totals: &category_totals})?;
        }
        ef.save("Entitlements cout per ou")?;
        Ok(())
    }
    pub fn cr_account_type_totals(&self) -> Result<()> {
        let mut ef = ExcelFileBuilder::new();
        for category_totals in self.iga.get_account_count_per_type() {
            ef.add_sheet(&category_totals.ts_uid, SheetType::Totals{totals: &category_totals})?;
        }
        ef.save("Accounts type totals")?;
        Ok(())
    }
    pub fn cr_account_type_lists(&self) -> Result<()> {
        for categorized_accts in self.iga.get_accounts_per_type() {
            let mut ef = ExcelFileBuilder::new();
            for (category, v) in categorized_accts.type_lists {   // Change to eference!! &
                let mut data = HashMap::new();
                data.insert(category.clone(), v.iter().collect());
                ef.add_sheet(&category, SheetType::AccountList{data})?;
            }
            ef.save(&format!("Account categorization - {}",&categorized_accts.ts_uid))?;
        }
        Ok(())
    }

    pub fn cr_orphan_accounts_per_system(&self) -> Result<()> {
        let mut ef = ExcelFileBuilder::new();
        let orph = self.iga.get_orphan_accounts();

        for (ts_uid, v) in orph {
            let mut data = HashMap::new();
            data.insert(ts_uid.clone(), v.iter().collect());
            ef.add_sheet(ts_uid.as_str(), SheetType::AccountList{data})?;
        }
        ef.save("All Orphan Accounts")?;
        Ok(())
    }

    pub fn cr_persistent_leaver_accounts(&self) -> Result<()> {
        let mut ef = ExcelFileBuilder::new();
        let pa = self.iga.get_persistent_leaver_accounts();
        for (ts_uid, v) in pa {
            let mut data = HashMap::new();
            data.insert(ts_uid.clone(), v.iter().collect());
            ef.add_sheet(&ts_uid, SheetType::AccountList{data})?;
        }
        ef.save("Persistent accounts")?;
        Ok(())
    }

}
enum SheetType<'a> {
    Totals {totals: &'a CategoryTotals},
    IdentitySummary {identity: &'a mut IdentityData, sync: &'a Option<(String, String)>},
   
    AccountList {data: HashMap<String,Vec<&'a AccountData>>}, 
    EntitlementList {data: HashMap<String,Vec<&'a EntitlementData>>}, 
    
    AccountsFull {accounts: HashMap<String, Vec<&'a mut AccountData>>, sync: &'a Option<(String, String)>}, 
    EntitlementsFull {entitlements: HashMap<String, Vec<&'a mut EntitlementData>>, sync: &'a Option<(String, String)>},
    
    AccountsListHistory {accounts: HashMap<String, Vec<&'a AccountData>>},
    EntitlementListHistory {entitlements: HashMap<String, Vec<&'a EntitlementData>>},
    
}

pub struct ExcelReportFormat {
    pub standard: Format,
    pub bold_format: Format,
    pub header: Format,
    pub header_secondary: Format,
    pub green_format: Format,
    pub grayout_format: Format,
}

struct ExcelFileBuilder {
    workbook: Workbook,
    format: ExcelReportFormat,
}
impl ExcelFileBuilder {
    fn new() -> Self {
        Self {
            workbook: Workbook::new(),
            format: ExcelReportFormat {
                standard: Format::new(),
                bold_format: Format::new().set_bold(),
                header: Format::new().set_bold().set_background_color(Color::RGB(0x3399FF)),
                header_secondary: Format::new().set_bold().set_background_color(Color::RGB(0xADD8E6)),
                green_format: Format::new().set_font_color(Color::RGB(0x90EE90)).set_align(FormatAlign::Center),
                grayout_format: Format::new().set_font_color(Color::RGB(0x808080)),
          }
        }
    }
    fn add_sheet(&mut self, name: &str, sheet_type: SheetType) -> Result<(), XlsxError> { 
        
        let worksheet = self.workbook.add_worksheet();
        worksheet.set_name(name)?;                                  
        
        let mut sheet = Sheet {
            worksheet,
            format: &self.format
        };

        match sheet_type {
            SheetType::Totals {totals} => 
                TotalsSheet(totals).print(&mut sheet)?, 

            SheetType::AccountList { data: accts} => 
                AccountSet(accts).print(&mut sheet)?, 

            SheetType::EntitlementList { data } => 
                EntitlementSet(data).print(&mut sheet)?, 

            SheetType::IdentitySummary { identity, sync} => 
                IdentitySummaryPrinter::from(identity, sync).print(&mut sheet)?,              

            SheetType::AccountsFull {accounts, sync} => 
                {SyncedAccountSet::from(accounts, sync).print(0, &mut sheet, true)?;}, 

            SheetType::EntitlementsFull {entitlements, sync} => 
                {SyncedEntitlementSet::from(entitlements, sync).print(0, &mut sheet, true)?;}, 
            
            SheetType::AccountsListHistory {accounts} => 
                AccountSet(accounts).print_with_history(&mut sheet)?, 

            SheetType::EntitlementListHistory {entitlements: ents} => 
                EntitlementSet(ents).print_with_history(&mut sheet)?, 
        }
        Ok(())
    }
    fn save(&mut self, filename: &str) -> Result<(), XlsxError> {
        // Save the file to disk.
        let filename = format!("output/{}.xlsx", filename);
        self.workbook.save(filename.as_str())?;             
        Ok(())
    }
}

