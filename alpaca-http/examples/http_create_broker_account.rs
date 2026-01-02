//! # Create Broker Account
//!
//! This example demonstrates how to create a new broker account
//! using the Alpaca Broker API.
//!
//! ## Prerequisites
//!
//! Set environment variables:
//! - `ALPACA_API_KEY`: Your Alpaca Broker API key
//! - `ALPACA_API_SECRET`: Your Alpaca Broker secret key
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-http --example http_create_broker_account
//! ```
//!
//! **Note**: This example demonstrates the API structure but does not
//! actually create accounts to avoid unintended side effects.

use alpaca_base::{
    Agreement, AgreementType, Contact, CreateBrokerAccountRequest, Disclosures, Identity,
    TaxIdType, TrustedContact,
};

fn main() {
    println!("=== Create Broker Account ===\n");

    // Step 1: Create contact information
    println!("--- Step 1: Contact Information ---");
    let contact = Contact::new("john.doe@example.com", "New York", "10001", "USA")
        .phone("+1-555-123-4567")
        .street("123 Main Street")
        .street("Apt 4B")
        .state("NY");

    println!("  Email: {}", contact.email_address);
    println!("  Phone: {:?}", contact.phone_number);
    println!("  City: {}", contact.city);
    println!("  State: {:?}", contact.state);
    println!("  Postal Code: {}", contact.postal_code);
    println!("  Country: {}", contact.country);

    // Step 2: Create identity information
    println!("\n--- Step 2: Identity Information ---");
    let identity = Identity::new("John", "Doe", "1990-01-15")
        .tax_id("123-45-6789", TaxIdType::UsaSsn)
        .citizenship("USA");

    println!("  Name: {} {}", identity.given_name, identity.family_name);
    println!("  Date of Birth: {}", identity.date_of_birth);
    println!("  Tax ID Type: {:?}", identity.tax_id_type);
    println!("  Citizenship: {:?}", identity.country_of_citizenship);

    // Step 3: Create disclosures
    println!("\n--- Step 3: Disclosures ---");
    let disclosures = Disclosures::new().control_person(false).employment(
        "employed",
        "Tech Corp",
        "Software Engineer",
    );

    println!("  Is Control Person: {}", disclosures.is_control_person);
    println!(
        "  Is Affiliated with Exchange/FINRA: {}",
        disclosures.is_affiliated_exchange_or_finra
    );
    println!(
        "  Is Politically Exposed: {}",
        disclosures.is_politically_exposed
    );
    println!("  Employment Status: {:?}", disclosures.employment_status);
    println!("  Employer: {:?}", disclosures.employer_name);

    // Step 4: Create agreements
    println!("\n--- Step 4: Agreements ---");
    let now = "2024-01-15T10:30:00Z";
    let ip = "192.168.1.100";

    let agreements = vec![
        Agreement::new(AgreementType::CustomerAgreement, now, ip),
        Agreement::new(AgreementType::MarginAgreement, now, ip),
        Agreement::new(AgreementType::AccountAgreement, now, ip),
    ];

    for agreement in &agreements {
        println!(
            "  {:?} - signed at {}",
            agreement.agreement, agreement.signed_at
        );
    }

    // Step 5: Create trusted contact (optional)
    println!("\n--- Step 5: Trusted Contact (Optional) ---");
    let trusted_contact = TrustedContact::new("Jane", "Doe")
        .email("jane.doe@example.com")
        .phone("+1-555-987-6543");

    println!(
        "  Name: {} {}",
        trusted_contact.given_name, trusted_contact.family_name
    );
    println!("  Email: {:?}", trusted_contact.email_address);
    println!("  Phone: {:?}", trusted_contact.phone_number);

    // Step 6: Build the complete request
    println!("\n--- Step 6: Build Request ---");
    let request = CreateBrokerAccountRequest::new(contact, identity, disclosures, agreements)
        .trusted_contact(trusted_contact)
        .enabled_assets(vec!["us_equity".to_string(), "crypto".to_string()]);

    println!("  Request built successfully");
    println!("  Enabled assets: {:?}", request.enabled_assets);

    // API call demonstration
    println!("\n--- API Call ---");
    println!("  To create the account:");
    println!("    let account = client.create_broker_account(&request).await?;");
    println!("    println!(\"Account ID: {{}}\", account.id);");
    println!("    println!(\"Status: {{:?}}\", account.status);");

    // Account lifecycle
    println!("\n--- Account Lifecycle ---");
    println!("  1. ONBOARDING: Initial state after creation");
    println!("  2. SUBMISSION_FAILED: KYC/AML check failed");
    println!("  3. SUBMITTED: Documents submitted for review");
    println!("  4. ACTION_REQUIRED: Additional info needed");
    println!("  5. APPROVAL_PENDING: Under review");
    println!("  6. APPROVED: Ready for trading (limited)");
    println!("  7. ACTIVE: Fully active account");
    println!("  8. REJECTED: Application rejected");
    println!("  9. DISABLED: Account disabled");
    println!(" 10. ACCOUNT_CLOSED: Account closed");

    println!("\n=== Example Complete ===");
}
