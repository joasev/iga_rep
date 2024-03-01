use std::sync::RwLock;
use calamine::{Reader, Xlsx, open_workbook, DataType};
use chrono::Duration;
use chrono::NaiveDate;
use serde::Serialize;
use anyhow::Result;
use anyhow::Context; 

use super::dtos::IdentityDTO;

#[derive(Debug, Serialize)]
pub struct IdentityXlsxConnector { 
    pub source_path: String,
    pub sheet_name: String,

    pub identity_attributes: RwLock<Vec<ColumnMeaning>>, 
}

#[derive(Debug, Serialize)]
pub struct ColumnMeaning {
    pub xlsx_column_name: String,
    pub identity_attribute: Option<String>,
    pub column_index: usize,
}
impl IdentityXlsxConnector {
    pub fn read_identities(&self) -> Result<Vec<IdentityDTO>>{
        let mut identities = Vec::new();
    
        // Open excel file
        let mut excel: Xlsx<_> = open_workbook(&self.source_path).context("Failed to open the workbook")?; 

        // Open sheet indicated in configuration
        if let Some(Ok(r)) = excel.worksheet_range(&self.sheet_name) {

            // If there is at least one row, it proceeds to load identities
            if let Some(first_row) = r.rows().next() {

                // Goes through the header row and records the index of each column name
                // This is necessary to retrieve data rows later
                self._load_column_indexes(first_row)?;

                // Loads one identity per row, skipping the header
                for row in r.rows().skip(1) {
                    let identity = self._extract_identity(row)?;
                    identities.push(identity);
                }
            }
        } 
        Ok(identities)
    }
    fn _load_column_indexes(&self, first_row: &[DataType]) -> Result<()> {
        
        // Goes through the header row and records the index of each column name
        let mut rww_guard = match self.identity_attributes.write() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(anyhow::Error::msg("Failed to acquire write lock on IdentityXlsxConnector"));
            },
        };
        for column_meaning  in &mut *rww_guard {
            column_meaning.column_index = first_row.iter().position(|cell| cell == column_meaning.xlsx_column_name.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!("Column name '{}' not found in the first row of Xlsx", column_meaning.xlsx_column_name)
                })?;
        }
        Ok(())
    }
    fn _extract_identity(&self, row: &[DataType]) -> Result<IdentityDTO> {

        // Get read permissions on attributes to retrieve from the rows
        let rwr_guard = match self.identity_attributes.read() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(anyhow::Error::msg("Failed to acquire read lock on IdentityXlsxConnector"));
            },
        };

        let mut identity = IdentityDTO::default();

        // For each attribute configured to be read, it gets the column index, retrieves the data
        // and stores it in the IdentityDTO property indicated in the configuration
        for column_meaning in & *rwr_guard { 
            let i = column_meaning.column_index;
            match &column_meaning.identity_attribute {
                None => {identity.attributes.insert(column_meaning.xlsx_column_name.clone(), row[i].to_string());}
                Some(field) => {
                    match field.as_str() {
                        "unique_id" => identity.unique_id = row[i].to_string().to_uppercase(),
                        "first_name" => identity.first_name = row[i].to_string(),
                        "last_name" => identity.last_name = row[i].to_string(),
                        "email" => identity.email = row[i].to_string(),
                        "employee_no" => identity.employee_no = row[i].to_string(),
                        "employee_type" => identity.employee_type = row[i].to_string(),
                        "enabled" => {
                            match row[i].as_i64() { 
                                Some(inactive) => {
                                    if inactive == 0 { identity.enabled = Some(false) }
                                    else if inactive == 3 { identity.enabled = Some(true) }
                                    else {identity.enabled = None}
                                }
                                None => {identity.enabled = None}
                            }
                        }
                        "manager_key" => identity.manager_key = row[i].to_string(),
                        "hire_date" => identity.hire_date = IdentityXlsxConnector::_read_date(&row[i]),
                        "termination_date" => identity.termination_date = IdentityXlsxConnector::_read_date(&row[i]),
                        _ => {}
                    }
                }
            }
        }
        Ok(identity)
    }

    fn _read_date(cell: &DataType) -> Option<NaiveDate> {
        if let DataType::DateTime(serial) = cell {
            let start = NaiveDate::from_ymd_opt(1899, 12, 30).expect("Creating date that certainly exists");
            start.checked_add_signed(Duration::days(*serial as i64))
        } else {
            None
        }
    }
   
}

