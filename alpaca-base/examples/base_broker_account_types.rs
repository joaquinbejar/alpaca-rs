//! # Broker Account Types
//!
//! This example demonstrates broker account creation types including
//! Contact, Identity, Disclosures, and Agreement.
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-base --example base_broker_account_types
//! ```

use alpaca_base::{
    Agreement, AgreementType, Contact, Disclosures, Identity, TaxIdType, TrustedContact,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Broker Account Types ===\n");

    // 1. Contact Information
    println!("--- Contact Information ---");
    demonstrate_contact()?;

    // 2. Identity Information
    println!("\n--- Identity Information ---");
    demonstrate_identity()?;

    // 3. Disclosures
    println!("\n--- Disclosures ---");
    demonstrate_disclosures()?;

    // 4. Agreements
    println!("\n--- Agreements ---");
    demonstrate_agreements()?;

    // 5. Trusted Contact
    println!("\n--- Trusted Contact ---");
    demonstrate_trusted_contact()?;

    println!("\n=== Example Complete ===");
    Ok(())
}

fn demonstrate_contact() -> Result<(), Box<dyn std::error::Error>> {
    let contact = Contact::new("john.doe@example.com", "New York", "10001", "USA")
        .phone("+1-555-123-4567")
        .street("123 Wall Street")
        .state("NY");

    println!("  Contact:");
    println!("    Email: {}", contact.email_address);
    println!("    Phone: {:?}", contact.phone_number);
    println!(
        "    Address: {:?}, {}, {:?} {}",
        contact.street_address, contact.city, contact.state, contact.postal_code
    );
    println!("    Country: {}", contact.country);

    let json = serde_json::to_string_pretty(&contact)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}

fn demonstrate_identity() -> Result<(), Box<dyn std::error::Error>> {
    let identity = Identity::new("John", "Doe", "1985-06-15")
        .tax_id("123-45-6789", TaxIdType::UsaSsn)
        .citizenship("USA");

    println!("  Identity:");
    println!("    Name: {} {}", identity.given_name, identity.family_name);
    println!("    DOB: {}", identity.date_of_birth);
    println!("    Tax ID Type: {:?}", identity.tax_id_type);
    println!("    Citizenship: {:?}", identity.country_of_citizenship);
    println!("    Funding Sources: {:?}", identity.funding_source);

    // Tax ID Types
    println!("\n  Tax ID Types:");
    let tax_types = [TaxIdType::UsaSsn, TaxIdType::CanSin, TaxIdType::GbrNino];
    for t in &tax_types {
        println!("    {:?}", t);
    }

    Ok(())
}

fn demonstrate_disclosures() -> Result<(), Box<dyn std::error::Error>> {
    // Default disclosures (all false)
    let disclosures = Disclosures::new();
    println!("  Default Disclosures:");
    println!("    Control Person: {}", disclosures.is_control_person);
    println!(
        "    Affiliated with Exchange/FINRA: {}",
        disclosures.is_affiliated_exchange_or_finra
    );
    println!(
        "    Politically Exposed: {}",
        disclosures.is_politically_exposed
    );
    println!(
        "    Immediate Family Exposed: {}",
        disclosures.immediate_family_exposed
    );

    // Disclosures with control person flag
    let disclosures_control = Disclosures::new().control_person(true);
    println!("\n  Control Person Disclosure:");
    println!(
        "    Control Person: {}",
        disclosures_control.is_control_person
    );

    let json = serde_json::to_string_pretty(&disclosures)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}

fn demonstrate_agreements() -> Result<(), Box<dyn std::error::Error>> {
    let now = chrono::Utc::now().to_rfc3339();
    let ip = "192.168.1.100";

    // Required agreements
    let agreements = vec![
        Agreement::new(AgreementType::CustomerAgreement, &now, ip),
        Agreement::new(AgreementType::AccountAgreement, &now, ip),
        Agreement::new(AgreementType::MarginAgreement, &now, ip),
    ];

    println!("  Required Agreements:");
    for agreement in &agreements {
        println!(
            "    - {:?} (signed from {})",
            agreement.agreement, agreement.ip_address
        );
    }

    // Agreement types
    println!("\n  Agreement Types:");
    let types = [
        AgreementType::CustomerAgreement,
        AgreementType::AccountAgreement,
        AgreementType::MarginAgreement,
        AgreementType::CryptoAgreement,
        AgreementType::OptionsAgreement,
    ];
    for t in &types {
        let json = serde_json::to_string(t)?;
        println!("    {:?} -> {}", t, json);
    }

    Ok(())
}

fn demonstrate_trusted_contact() -> Result<(), Box<dyn std::error::Error>> {
    let trusted = TrustedContact::new("Jane", "Doe")
        .email("jane.doe@example.com")
        .phone("+1-555-987-6543");

    println!("  Trusted Contact:");
    println!("    Name: {} {}", trusted.given_name, trusted.family_name);
    println!("    Email: {:?}", trusted.email_address);
    println!("    Phone: {:?}", trusted.phone_number);

    let json = serde_json::to_string_pretty(&trusted)?;
    println!("\n  JSON:\n{}", json);

    Ok(())
}
