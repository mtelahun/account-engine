-- Add migration script here
CREATE TYPE TRANSACTIONSTATE AS ENUM('pending', 'archived', 'posted');
CREATE DOMAIN JournalId AS UUID;

CREATE TABLE general_journal(
    id JournalId NOT NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    PRIMARY KEY(id)
);

CREATE TABLE general_journal_transaction_record(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    explanation TEXT NOT NULL,
    PRIMARY KEY(journal_id, timestamp),
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES general_journal(id)
);

CREATE TABLE general_journal_transaction_line_ledger(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_id LedgerId,
    xact_type XactType NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES general_journal(id),
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES general_journal_transaction_record(journal_id, timestamp),
    CONSTRAINT fk_ledger_id
        FOREIGN KEY(ledger_id)
            REFERENCES ledger(id)
);

CREATE TABLE general_journal_transaction_line_account(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_id LedgerId,
    account_id AccountId,
    xact_type XactType NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    CONSTRAINT fk_journal_id
        FOREIGN KEY(journal_id)
            REFERENCES general_journal(id),
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES general_journal_transaction_record(journal_id, timestamp),
    CONSTRAINT fk_account_id
        FOREIGN KEY(account_id)
            REFERENCES external_account(id)
);
