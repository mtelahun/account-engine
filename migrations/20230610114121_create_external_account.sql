-- Add migration script here
CREATE TABLE entity_type(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE external_account_type(
    entity_code CHAR(2) NOT NULL,
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(entity_code, code)
);

CREATE TABLE external_account(
    id AccountId NOT NULL,
    subsidiary_ledger_id SubLedgerId NOT NULL,
    entity_code CHAR(2),
    account_no TEXT NOT NULL UNIQUE,
    date_opened DATE,
    PRIMARY KEY(id),
    CONSTRAINT fk_subsidiary_ledger_id
        FOREIGN KEY(subsidiary_ledger_id)
            REFERENCES subsidiary_ledger(id)
                ON DELETE RESTRICT,
    CONSTRAINT fk_entity_code
        FOREIGN KEY(entity_code)
            REFERENCES entity_type(code)
                ON DELETE RESTRICT
);

CREATE TABLE transaction_type_external(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);
