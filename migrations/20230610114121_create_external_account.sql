-- Add migration script here
CREATE DOMAIN ExternalEntityId AS UUID;

CREATE TABLE entity_type(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE external_entity(
    id ExternalEntityId NOT NULL,
    entity_type_code CHAR(2),
    name TEXT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE external_account_type(
    code CHAR(2) NOT NULL,
    xact_type_code XactType NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE external_account(
    id AccountId NOT NULL,
    subsidiary_ledger_id SubLedgerId NOT NULL,
    external_entity_id ExternalEntityId NOT NULL,
    account_no TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    date_opened DATE,
    PRIMARY KEY(id),
    CONSTRAINT fk_subsidiary_ledger_id
        FOREIGN KEY(subsidiary_ledger_id)
            REFERENCES subsidiary_ledger(id)
                ON DELETE RESTRICT,
    CONSTRAINT fk_external_entity_id
        FOREIGN KEY(external_entity_id)
            REFERENCES external_entity(id)
                ON DELETE RESTRICT
);

CREATE TABLE external_account_transaction (
    external_account_id AccountId NOT NULL,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    xact_type_code XactType NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    PRIMARY KEY(external_account_id, timestamp),
    CONSTRAINT fk_external_account_id
        FOREIGN KEY(external_account_id)
            REFERENCES external_account(id)
);

CREATE TABLE transaction_type_external(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);
