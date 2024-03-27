# Special Journals
By their nature special journals do not have a standardized layout. Each special journal is customized to capture all the transaction information for the specialized situation in which it is used.

## Special Journal Column Template
A generic mechanism for programatically defining special journals at run-time.
A special journal template is used to define the information we wish to capture for a special category of transactions. Each special journal template will define a template name and one or more columns. Each column will have a sequence number to denote its relative position in relation to the other columns of the journal.
Each column will also have a type:
* DATETIME - contains a timestamp value
* TEXT     - contains an arbitrary text value
* DRCRAMT  - contains an amount value to be entered into two ledgers (one Dr and one Cr)
* DRAMT    - contains an amount value to be debited to one ledger
* CRAMT    - contains an amount value to be credited to one ledger
* DRACT    - contains an external account to be debited
* CRACT    - contains an external account to be credited
* DROTHER  - contains a composite value with the ledger or exernal acount to be debited and an amount
* CROTHER  - contains a composite column with the ledger or external acount to be credited and an amount

## Categories
Most transaction can be divided into one of five categories. Transaction are recorded in either the general
journal or a special journal, but not in both.

|   | Transaction         | Special Journal       | Abbreviation |
----|---------------------|-----------------------|:------------:|
| 1 | Sale on account     | Sales journal         |      S       |
| 2 | Cash receipt        | Cash receipts journal |      CR      |
| 3 | Purchase on account | Purchase journal      |      P       |
| 4 | Cash payment        | Cash payments journal |      CP      |
| 5 | All others          | General journal       |      J       |

## Sales journal (S)
Template name: Sales journal template
### Columns
#### Date
| Name       | Sequence | Column type |
| ---------- | -------- | ----------- |
| Date       | 1        | DATETIME    |

#### Invoice No.
| Name       | Sequence | Column type |
|------------|----------|-------------|
| Invoice No |2         | TEXT        |

#### Account Debited
| Name            | Sequence | Column type |
| --------------- | -------- | ----------- |
| Account debited | 3        | DRACT       |

DR: External account of customer

#### A/C Dr | Sales Cr
| Name       | Sequence | Column type |
| ---------- | -------- | ----------- |
|            | 4        | DRCRAMT     |

DR: Accounts Receivable ledger<br>
CR: Sales Revenue ledger

#### Cogs | Inventory
| Name | Sequence | Column type |
| ---- | -------- | ----------- |
|      | 5        | DRCRAMT     |

DR: Cost of goods sold ledger<br>
CR: Inventory ledger
