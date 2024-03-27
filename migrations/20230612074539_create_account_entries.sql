-- Add migration script here
CREATE TABLE ledger_transaction(
    ledger_id LedgerId NOT NULL,
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
            REFERENCES journal_transaction(journal_id, timestamp)
);

CREATE TABLE ledger_transaction_ledger(
    ledger_id LedgerId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_dr_id LedgerId NOT NULL,
    CONSTRAINT fk_ledger_transaction
        FOREIGN KEY(ledger_id, timestamp)
            REFERENCES ledger_transaction(ledger_id, timestamp)
);

CREATE TABLE ledger_transaction_account(
    ledger_id LedgerId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    account_id AccountId NOT NULL,
    transaction_type_code CHAR(2) NOT NULL,
    transaction_type_external_code CHAR(2) NOT NULL,
    CONSTRAINT fk_ledger_transaction
        FOREIGN KEY(ledger_id, timestamp)
            REFERENCES ledger_transaction(ledger_id, timestamp),
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

CREATE TABLE ledger_transaction_account_sum(
    ledger_id LedgerId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    special_journal_column_sum_id ColumnTotalId NOT NULL,
    transaction_type_code CHAR(2) NOT NULL,
    transaction_type_external_code CHAR(2) NOT NULL,
    CONSTRAINT fk_ledger_transaction
        FOREIGN KEY(ledger_id, timestamp)
            REFERENCES ledger_transaction(ledger_id, timestamp),
    CONSTRAINT fk_special_journal_column_sum_id
        FOREIGN KEY(special_journal_column_sum_id)
            REFERENCES journal_transaction_special_column_sum(id),
    CONSTRAINT fk_transaction_type_code
        FOREIGN KEY(transaction_type_code)
            REFERENCES transaction_type(code),
    CONSTRAINT fk_transaction_type_external_code
        FOREIGN KEY(transaction_type_external_code)
            REFERENCES transaction_type_external(code)
);

CREATE TYPE "LedgerKey" AS (
    ledger_id LedgerId,
    timestamp TIMESTAMP WITHOUT TIME ZONE
);

CREATE TYPE "SubsidiaryLedgerKey" AS (
    account_id AccountId,
    timestamp TIMESTAMP WITHOUT TIME ZONE
);

CREATE TYPE "LedgerPostingRef" AS (
    key "LedgerKey",
    ledger_id LedgerId
);

CREATE TYPE "LedgerAccountPostingRef" AS (
    key "LedgerKey",
    account_id AccountId
);

CREATE TYPE "AccountPostingRef" AS (
    key "SubsidiaryLedgerKey"
);

ALTER TABLE journal_transaction_general
    ADD COLUMN dr_posting_ref "LedgerPostingRef",
    ADD COLUMN cr_posting_ref "LedgerPostingRef";

ALTER TABLE journal_transaction_special
    ADD COLUMN posting_ref "LedgerAccountPostingRef";

ALTER TABLE journal_transaction_special_column_sum
    ADD COLUMN posting_ref_dr "LedgerPostingRef",
    ADD COLUMN posting_ref_cr "LedgerPostingRef";

ALTER TABLE journal_transaction_column_account_dr
    ADD COLUMN posting_ref "AccountPostingRef";