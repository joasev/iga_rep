# Access Analyzer Tool

## Overview

The Access Analyzer Tool is designed to provide a comprehensive analysis of access data from various sources, including multiple Active Directory (AD) domains, HR records, and ServiceNow. It processes this data in-memory to generate detailed reports on the current state of access, calculate risk scores per entitlement, and identify orphaned accounts and entitlements. The tool also compiles all historical metadata related to these findings, significantly reducing the investigation effort required by operations teams.

## Features

- **Multi-Source Data Aggregation**: Integrates data from multiple AD domains, HR systems, and ServiceNow.
- **In-Memory Processing**: Efficiently processes large datasets in-memory for rapid analysis and report generation.
- **Comprehensive Reporting**: Generates detailed reports on the current state of access, including risk scores and historical metadata.
- **Risk Score Calculation**: Computes risk scores for each entitlement based on approval and provisioning records.
- **Orphaned Account Detection**: Identifies and reports orphaned accounts and entitlements to streamline the cleanup process.
- **Historical Metadata Compilation**: Provides access to historical metadata to facilitate detailed investigations and audits.

## Reports

The tool generates various reports that provide insights into access management, risk assessment, and orphaned entities. These reports can be accessed via the `reports` directory in the project root.

### Report Types

- **Access Overview Report**: Summarizes the current state of access across all integrated sources.
- **Risk Score Report**: Lists entitlements along with their calculated risk scores.
- **Orphaned Accounts Report**: Details accounts that have no associated users or are no longer needed.
- **Orphaned Entitlements Report**: Identifies entitlements that are not linked to any active accounts.
- **Historical Metadata Report**: Provides a detailed history of all access changes, approvals, and provisioning events.

## Technical Details

The Access Analyzer Tool is implemented in Rust, demonstrating fluency in the Rust programming language and showcasing good coding practices for a mid-sized project. It leverages Rust's strong type system, ownership model, and data structures to ensure memory safety and high performance.
