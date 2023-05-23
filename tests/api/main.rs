use account_engine::{
    account::LedgerType,
    memory_store::MemoryStore,
    storage::{AccountEngineStorage, StorageError},
};
use rusty_money::iso;

#[test]
fn test_unique_ledger_name() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let ledger1 = db.new_ledger("My Company", iso::USD);
    let ledger2 = db.new_ledger("My Company", iso::EUR);

    // Assert
    assert!(ledger1.is_ok(), "first ledger created successfully");
    assert!(ledger2.is_err(), "failed to create second ledger");
    assert_eq!(
        ledger2.err().unwrap(),
        Err::<(), StorageError>(StorageError::DuplicateRecord(
            "duplicate ledger name".into()
        ))
        .err()
        .unwrap()
    );
    assert_eq!(db.ledgers().len(), 1, "Only one ledger in the list")
}

#[test]
fn test_unique_account_number() {
    // Arrance
    let db = MemoryStore::new();
    let ledger = db.new_ledger("My Company", iso::USD).unwrap();
    let ledger2 = db.new_ledger("Another Company", iso::USD).unwrap();

    // Act
    let assets = db.new_account(
        &ledger,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );
    let liabilities = db.new_account(
        &ledger,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );
    let assets2 = db.new_account(
        &ledger2,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );

    // Assert
    assert!(assets.is_ok(), "first account created successfully");
    assert!(liabilities.is_err(), "failed to create second account");
    assert_eq!(
        liabilities.err().unwrap(),
        Err::<(), StorageError>(StorageError::DuplicateRecord(
            "duplicate account number".into()
        ))
        .err()
        .unwrap()
    );
    assert_eq!(
        db.accounts(&ledger).len(),
        1,
        "Only one account in the ledger"
    );
    assert!(
        assets2.is_ok(),
        "account with duplicate numbe, but in DIFFERENT ledger created successfully"
    );
}

#[test]
fn test_duplicate_account_name() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let ledger = db.new_ledger("My Company", iso::USD).unwrap();
    let assets = db.new_account(
        &ledger,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );
    let assets2 = db.new_account(
        &ledger,
        "Assets",
        "1100",
        LedgerType::Intermediate,
        Some(iso::USD),
    );

    // Assert
    assert!(assets.is_ok(), "first account created successfully");
    assert!(assets2.is_ok(), "second account created successfully");
    assert_eq!(
        assets.unwrap().name,
        assets2.unwrap().name,
        "account with duplicate name created successfully"
    );
    assert_eq!(
        db.accounts(&ledger).len(),
        2,
        "Both accounts appear in the ledger"
    )
}
