-- Add migration script here
CREATE TYPE LedgerType AS ENUM('derived', 'intermediate', 'leaf');

CREATE TABLE account_type(
    code CHAR(2) NOT NULL,
    description TEXT,
    PRIMARY KEY(code)
);

CREATE TABLE ledger(
    id LedgerId NOT NULL,
    ledger_no TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    ledger_type LEDGERTYPE NOT NULL,
    parent_id LedgerId,
    currency_code CHAR(3),
    PRIMARY KEY(id)
);

CREATE TABLE ledger_intermediate(
    id LedgerId UNIQUE NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_ledger_id FOREIGN KEY(id) REFERENCES ledger(id) ON DELETE RESTRICT
);

CREATE TABLE ledger_leaf(
    id LedgerId UNIQUE NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_ledger_id FOREIGN KEY(id) REFERENCES ledger(id) ON DELETE RESTRICT
);

CREATE TABLE ledger_derived(
    id LedgerId UNIQUE NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_ledger_id FOREIGN KEY(id) REFERENCES ledger(id) ON DELETE RESTRICT
);

ALTER TABLE subsidiary_ledger
    ADD CONSTRAINT fk_ledger_id FOREIGN KEY(ledger_id) REFERENCES ledger(id);

CREATE TABLE ledger_transaction_type(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

INSERT INTO ledger(id, ledger_no, name, ledger_type)
    VALUES(gen_random_uuid(), '0', 'Root', 'intermediate');

INSERT INTO ledger_intermediate
    SELECT id from ledger LIMIT 1;

INSERT INTO general_ledger(id, name, currency_code, root)
    VALUES(gen_random_uuid(), 'Root', 'USD', (SELECT id from ledger LIMIT 1));

ALTER TABLE ledger
    ADD CONSTRAINT ct_parent_id_not_null
        CHECK (parent_id IS NOT NULL) NOT VALID;

INSERT INTO ledger_transaction_type(code, description)
    VALUES('LA', 'Ledger - Account');

INSERT INTO ledger_transaction_type(code, description)
    VALUES('LL', 'Ledger - Ledger');
