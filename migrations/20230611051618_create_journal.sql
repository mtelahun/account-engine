-- Add migration script here
CREATE TYPE JOURNALTYPE AS ENUM('general', 'special');
CREATE TYPE TRANSACTIONSTATE AS ENUM('pending', 'archived', 'posted');
CREATE DOMAIN JournalId AS UUID;
CREATE DOMAIN SubJournalTemplateId AS UUID;
CREATE DOMAIN SubJournalTemplateColId AS UUID;

CREATE TABLE journal(
    id JournalId NOT NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    journal_type JOURNALTYPE NOT NULL,
    ledger_id LedgerId,
    template_id SubJournalTemplateId,
    PRIMARY KEY(id)
);

CREATE TABLE journal_transaction_record(
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
    ledger_id LedgerId,
    xact_type XactType NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES journal(id),
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_record(journal_id, timestamp),
    CONSTRAINT fk_ledger_id
        FOREIGN KEY(ledger_id)
            REFERENCES ledger(id)
);

CREATE TABLE journal_transaction_subsidiary_template(
    id SubJournalTemplateId NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE journal_transaction_subsidiary_template_column(
    id SubJournalTemplateColId NOT NULL,
    template_id SubJournalTemplateId NOT NULL,
    sequence SMALLINT NOT NULL,
    dr_ledger_id AccountId,
    cr_ledger_id AccountId,
    PRIMARY KEY(id)
);

CREATE TABLE journal_transaction_subsidiary(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_id LedgerId,
    account_id AccountId,
    xact_type XactType NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES journal(id),
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_record(journal_id, timestamp),
    CONSTRAINT fk_account_id
        FOREIGN KEY(account_id)
            REFERENCES external_account(id)
);
