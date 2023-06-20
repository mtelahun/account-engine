-- Add migration script here
CREATE TYPE TRANSACTIONSTATE AS ENUM('pending', 'archived', 'posted');
CREATE DOMAIN JournalId AS UUID;

CREATE TABLE journal(
    id JournalId NOT NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE journal_transaction_record(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    explanation TEXT NOT NULL,
    PRIMARY KEY(journal_id, timestamp)
);

CREATE TABLE journal_transaction_line_ledger(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_id AccountId,
    xact_type XactType NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_record(journal_id, timestamp),
    CONSTRAINT fk_ledger_id
        FOREIGN KEY(ledger_id)
            REFERENCES ledger(id)
);

CREATE TABLE journal_transaction_line_account(
    journal_id JournalId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ledger_id AccountId,
    account_id AccountId,
    xact_type XactType NOT NULL,
    state TRANSACTIONSTATE NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    CONSTRAINT fk_journal_transaction_record
        FOREIGN KEY(journal_id, timestamp)
            REFERENCES journal_transaction_record(journal_id, timestamp),
    CONSTRAINT fk_account_id
        FOREIGN KEY(account_id)
            REFERENCES external_account(id)
);

-- CREATE TABLE journal_line(
--     id JournalLineId NOT NULL,
--     timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
--     journal_id JournalId NOT NULL,
--     ledger_id AccountId NOT NULL,
--     amount NUMERIC(20, 8) NOT NULL,
--     ledger_xact_type_code CHAR(2) NOT NULL,
--     state TRANSACTIONSTATE NOT NULL,
--     description TEXT NOT NULL,
--     PRIMARY KEY(id),
--     CONSTRAINT fk_journal_id
--         FOREIGN KEY(journal_id)
--             REFERENCES journal(id),
--     CONSTRAINT fk_ledger_id
--         FOREIGN KEY(ledger_id)
--             REFERENCES ledger(id),
--     CONSTRAINT fk_ledger_transaction_type_code
--         FOREIGN KEY(ledger_transaction_type_code)
--             REFERENCES ledger_transaction_type(code)
-- );

-- CREATE TABLE journal_line_ledger(
--     id JournalLineId NOT NULL,
--     ledger_dr_id AccountId NOT NULL,
--     CONSTRAINT fk_id
--         FOREIGN KEY(id)
--             REFERENCES ledger(id)
--                 ON DELETE RESTRICT,
--     CONSTRAINT fk_ledger_dr_id
--         FOREIGN KEY(ledger_dr_id)
--             REFERENCES ledger(id)
-- );

-- CREATE TABLE journal_line_account(
--     id JournalLineId NOT NULL,
--     account_id AccountId NOT NULL,
--     transaction_type_code CHAR(2) NOT NULL,
--     transaction_type_external_code CHAR(2) NOT NULL,
--     CONSTRAINT fk_id
--         FOREIGN KEY(id)
--             REFERENCES ledger(id)
--                 ON DELETE RESTRICT,
--     CONSTRAINT fk_account_id
--         FOREIGN KEY(account_id)
--             REFERENCES external_account(id),
--     CONSTRAINT fk_transaction_type_code
--         FOREIGN KEY(transaction_type_code)
--             REFERENCES transaction_type(code),
--     CONSTRAINT fk_transaction_type_external_code
--         FOREIGN KEY(transaction_type_external_code)
--             REFERENCES transaction_type_external(code)
-- );
