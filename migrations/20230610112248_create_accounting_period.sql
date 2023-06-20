-- Add migration script here
CREATE TYPE INTERIMTYPE AS ENUM('calendar_month', '4week', '445week');
CREATE DOMAIN PeriodId AS UUID;
CREATE DOMAIN InterimPeriodId AS UUID;

CREATE TABLE accounting_period(
    id PeriodId NOT NULL,
    fiscal_year INT NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    period_type INTERIMTYPE NOT NULL DEFAULT 'calendar_month',
    PRIMARY KEY(id)
);

CREATE TABLE interim_accounting_period(
    id InterimPeriodId NOT NULL,
    parent_id PeriodId NOT NULL,
    interim_start DATE NOT NULL,
    interim_end DATE NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_parent_id FOREIGN KEY(parent_id) REFERENCES accounting_period(id)
);
