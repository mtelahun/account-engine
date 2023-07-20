-- Add migration script here
CREATE TYPE XactType AS ENUM('cr', 'dr');
CREATE DOMAIN GeneralLedgerId AS UUID;
CREATE DOMAIN SubLedgerId AS UUID;
CREATE DOMAIN LedgerId AS UUID;
CREATE DOMAIN AccountId AS UUID;

CREATE TABLE transaction_type(
    code CHAR(2) NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY(code)
);

CREATE TABLE general_ledger(
    id GeneralLedgerId NOT NULL,
    name TEXT NOT NULL,
    root LedgerId NOT NULL,
    currency_code CHAR(3) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE subsidiary_ledger(
    id SubLedgerId NOT NULL,
    name TEXT NOT NULL,
    ledger_id LedgerId NOT NULL,
    PRIMARY KEY(id)
);
