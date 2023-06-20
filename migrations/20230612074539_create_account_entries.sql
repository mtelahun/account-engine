-- Add migration script here
CREATE TABLE ledger_line(
    ledger_id AccountId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_transaction_type_code CHAR(2) NOT NULL,
    journal_id JournalId NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(ledger_id, timestamp),
    CONSTRAINT fk_ledger_transaction_type_code
        FOREIGN KEY(ledger_transaction_type_code)
            REFERENCES ledger_transaction_type(code),
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_record(journal_id, timestamp)
);

CREATE TABLE ledger_transaction(
    ledger_id AccountId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_dr_id AccountId NOT NULL,
    CONSTRAINT fk_ledger_line
        FOREIGN KEY(ledger_id, timestamp)
            REFERENCES ledger_line(ledger_id, timestamp)
);

CREATE TABLE account_transaction(
    ledger_id AccountId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    account_id AccountId NOT NULL,
    transaction_type_code CHAR(2) NOT NULL,
    transaction_type_external_code CHAR(2) NOT NULL,
    CONSTRAINT fk_ledger_line
        FOREIGN KEY(ledger_id, timestamp)
            REFERENCES ledger_line(ledger_id, timestamp),
    CONSTRAINT fk_account_id
        FOREIGN KEY(account_id)
            REFERENCES external_account(id),
    CONSTRAINT fk_transaction_type_code
        FOREIGN KEY(transaction_type_code)
            REFERENCES transaction_type(code),
    CONSTRAINT fk_transaction_type_external_code
        FOREIGN KEY(transaction_type_external_code)
            REFERENCES transaction_type_external(code)
);

CREATE TYPE "LedgerKey" AS (
    ledger_id AccountId,
    timestamp TIMESTAMP WITHOUT TIME ZONE
);

CREATE TYPE "PostingRef" AS (
    key "LedgerKey",
    account_id AccountId
);

ALTER TABLE journal_transaction_line_ledger
    ADD COLUMN posting_ref "PostingRef";

ALTER TABLE journal_transaction_line_account
    ADD COLUMN posting_ref "PostingRef";
