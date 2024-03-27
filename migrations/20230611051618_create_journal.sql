-- Add migration script here
CREATE TYPE JOURNALTYPE AS ENUM('general', 'special');
CREATE TYPE TRANSACTIONSTATE AS ENUM('pending', 'archived', 'posted');
CREATE TYPE SPECIALCOLUMNTYPE AS ENUM(
    'ledger_drcr', 'text', 'account_dr', 'account_cr', 'ledger_dr', 'ledger_cr'
);
CREATE DOMAIN JournalId AS UUID;
CREATE DOMAIN SpecialJournalTemplateId AS UUID;
CREATE DOMAIN TemplateColumnId AS UUID;
CREATE DOMAIN ColumnTotalId AS UUID;
CREATE DOMAIN SEQUENCE as SMALLINT NOT NULL CHECK (VALUE BETWEEN 1 AND 32767);

CREATE TYPE "JournalTransactionKey" AS (
    journal_id JournalId,
    timestamp TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE journal_transaction_special_template(
    id SpecialJournalTemplateId NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE journal_transaction_special_template_column (
    id TemplateColumnId NOT NULL,
    template_id SpecialJournalTemplateId NOT NULL,
    name TEXT NOT NULL,
    seq SEQUENCE NOT NULL,
    column_type SPECIALCOLUMNTYPE NOT NULL,
    dr_ledger_id LedgerId,
    cr_ledger_id LedgerId,
    dr_account_id AccountId,
    cr_account_id AccountId,
    other_dr_ledger_id LedgerId,
    other_cr_ledger_id LedgerId,
    other_dr_account_id AccountId,
    other_cr_account_id AccountId,
    PRIMARY KEY(id),
    CONSTRAINT fk_template_id
        FOREIGN KEY(template_id)
            REFERENCES journal_transaction_special_template(id),
    CONSTRAINT fk_dr_account_id
        FOREIGN KEY(dr_account_id)
            REFERENCES external_account(id),
    CONSTRAINT fk_cr_account_id
        FOREIGN KEY(cr_account_id)
            REFERENCES external_account(id),
    CONSTRAINT fk_other_dr_account_id
        FOREIGN KEY(other_dr_account_id)
            REFERENCES external_account(id),
    CONSTRAINT fk_other_cr_account_id
        FOREIGN KEY(other_cr_account_id)
            REFERENCES external_account(id)
);

CREATE TABLE journal(
    id JournalId NOT NULL,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    journal_type JOURNALTYPE NOT NULL,
    ledger_id LedgerId,
    template_id SpecialJournalTemplateId,
    PRIMARY KEY(id),
    CONSTRAINT fk_template_id
        FOREIGN KEY(template_id)
            REFERENCES journal_transaction_special_template(id)
);

CREATE TABLE journal_transaction(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    explanation TEXT NOT NULL,
    PRIMARY KEY(journal_id, timestamp),
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES journal(id)
);

CREATE TABLE journal_transaction_general(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    dr_ledger_id LedgerId NOT NULL,
    cr_ledger_id LedgerId NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    PRIMARY KEY(journal_id, timestamp),
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES journal(id),
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction(journal_id, timestamp),
    CONSTRAINT fk_dr_ledger_id
        FOREIGN KEY(dr_ledger_id)
            REFERENCES ledger(id),
    CONSTRAINT fk_cr_ledger_id
        FOREIGN KEY(cr_ledger_id)
            REFERENCES ledger(id)
);

CREATE TABLE journal_transaction_special(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    transaction_type_external_code CHAR(2) NOT NULL,
    template_id SpecialJournalTemplateId NOT NULL,
    PRIMARY KEY(journal_id, timestamp),
    CONSTRAINT fk_journal_transaction
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction(journal_id, timestamp),
    CONSTRAINT fk_transaction_type_external_code
        FOREIGN KEY(transaction_type_external_code)
            REFERENCES transaction_type_external(code),
    CONSTRAINT fk_template_id
        FOREIGN KEY(template_id)
            REFERENCES journal_transaction_special_template(id)
);

CREATE TABLE journal_transaction_column_text(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    template_column_id TemplateColumnId,
    value TEXT NOT NULL,
    PRIMARY KEY(journal_id, timestamp, template_column_id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp),
    CONSTRAINT fk_template_column_id
        FOREIGN KEY(template_column_id)
            REFERENCES journal_transaction_special_template_column(id)
);

CREATE TABLE journal_transaction_column_ledger_drcr(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    template_column_id TemplateColumnId,
    amount NUMERIC(20, 8),
    ledger_dr_id LedgerId NOT NULL,
    ledger_cr_id LedgerId NOT NULL,
    PRIMARY KEY(journal_id, timestamp, template_column_id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp),
    CONSTRAINT fk_ledger_dr_id
        FOREIGN KEY(ledger_dr_id)
            REFERENCES ledger(id),
    CONSTRAINT fk_ledger_cr_id
        FOREIGN KEY(ledger_cr_id)
            REFERENCES ledger(id),
    CONSTRAINT fk_template_column_id
        FOREIGN KEY(template_column_id)
            REFERENCES journal_transaction_special_template_column(id)
);

CREATE TABLE journal_transaction_column_ledger_dr(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    template_column_id TemplateColumnId,
    dr_ledger_id LedgerId NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(journal_id, timestamp, template_column_id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp),
    CONSTRAINT fk_template_column_id
        FOREIGN KEY(template_column_id)
            REFERENCES journal_transaction_special_template_column(id)
);

CREATE TABLE journal_transaction_column_ledger_cr(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    template_column_id TemplateColumnId,
    cr_ledger_id LedgerId NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(journal_id, timestamp, template_column_id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp),
    CONSTRAINT fk_template_column_id
        FOREIGN KEY(template_column_id)
            REFERENCES journal_transaction_special_template_column(id)
);

CREATE TABLE journal_transaction_column_account_dr(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    template_column_id TemplateColumnId NOT NULL,
    account_id AccountId NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(journal_id, timestamp, template_column_id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp),
    CONSTRAINT fk_template_column_id
        FOREIGN KEY(template_column_id)
            REFERENCES journal_transaction_special_template_column(id),
    CONSTRAINT fk_account_id
        FOREIGN KEY(account_id)
            REFERENCES external_account(id)
);

CREATE TABLE journal_transaction_column_account_cr(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    template_column_id TemplateColumnId NOT NULL,
    account_id AccountId NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(journal_id, timestamp, template_column_id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp),
    CONSTRAINT fk_template_column_id
        FOREIGN KEY(template_column_id)
            REFERENCES journal_transaction_special_template_column(id),
    CONSTRAINT fk_account_id
        FOREIGN KEY(account_id)
            REFERENCES external_account(id)
);

CREATE TABLE journal_transaction_special_summary(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    PRIMARY KEY(journal_id, timestamp),
    CONSTRAINT fk_journal_transaction
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction(journal_id, timestamp)
);

CREATE TABLE journal_transaction_special_column_sum(
    id ColumnTotalId,
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    sequence SEQUENCE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_journal_transaction_special
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_special(journal_id, timestamp)
);

ALTER TABLE external_account_transaction
    ADD COLUMN journal_ref "JournalTransactionKey";
