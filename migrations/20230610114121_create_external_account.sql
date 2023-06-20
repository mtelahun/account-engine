-- Add migration script here
CREATE TABLE entity_type(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE external_account_type(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE external_account(
    id AccountId NOT NULL,
    supporting_id SubLedgerId NOT NULL,
    account_no TEXT NOT NULL UNIQUE,
    entity_type_code CHAR(2),
    external_account_type_code CHAR(2),
    PRIMARY KEY(id),
    CONSTRAINT fk_supporting_id
        FOREIGN KEY(supporting_id)
            REFERENCES supporting_ledger(id)
                ON DELETE RESTRICT,
    CONSTRAINT fk_entity_type_code
        FOREIGN KEY(entity_type_code)
            REFERENCES entity_type(code)
                ON DELETE RESTRICT,
    CONSTRAINT fk_external_account_type_code
        FOREIGN KEY(external_account_type_code)
            REFERENCES external_account_type(code)
                ON DELETE RESTRICT
);

CREATE TABLE transaction_type_external(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);
