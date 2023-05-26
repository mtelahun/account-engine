=================
Ledger
=================

Principles
==========
- Model accurately the relationships of accounting entities
- No duplication of data, except where necessary by accounting standards
- Money is never created of destroyed it is only moved
- Money is always a positive value

Terminology
===========
The following terms define accounting terms as understood by this program. Corrections and
suggestions are welcome.

T-Account - The most widely used account format. So called because its representation on paper
takes the form of the capital letter T. The account name appears at the top and the vertical
line in the letter divides the account into its left and right-hand sides.

Debit - Represents transfer of value to or from an account. It is an entry on the left hand side
of a T-account. It has the effect of denoting an increase in value of Asset and Expense accounts
and a decrease in Liability, Equity and Revenue accounts. It is abbreviated as: 'Dr'.

Credit - Represents a transfer of value to or from an account. It is an entry on the right hand
side ofa T-account. It has the effect of denoting an increase in value of Liability, Equity and
Revenue accounts and a decrease in Asset and Expense accounts. It is abbrevaited as: 'Cr'.

Balance - The difference remaining in an account after all debits and credits are summed up. If
the sum of the debits is greater than the sum of the credits it is said to have a Debit balance.
If the reverse is true it is said to have a Debit balance. If both sums are equal it is said to
have a zero (0) balance.

Accounting Equation - A forumula that shows the relationship between an entity's Assets,
Liabilities, and Equity.
    Assets = Liabilities + (Capital + Withdrawals + Revenue + Expenses)
* Withdrawals is money taken out of the entity by the owner

In any valid double-entry accounting transaction the following 2 statements must hold true:
  Debits = Credits
  Equity = Capital + Withdrawals + Revenue + Expenses
  Balance of Asset Account(s) = Balance of Liability + Equity Account(s)

Ledger - contains the heirarchy of accounts used to maintain an organization's financial position.
The accounts hierarchy is grouped under these headings: Balance Sheet accounts (Assets, 
Liabilities, and Owner's Equity) and Income Statement accounts (Revenues and Expenses). There is
a single ledger per entity or economic unit.

Ledger Account - financial account used to record financial entries related to the business.

Ledger Account Type - There are two types of ledger accounts:
  Leaf - transactions are recorded in this account
  Intermediate - aggregates  accounts below it in the hierarchy. Does not contain transactions.

External Account - an account used to record financial entries about an entity external to
the organization. The balances of external accounts are aggregated in ledger accounts.
For example, each supplier will have a separate External Account, but the balances will be
aggregated under the 'Creditors' intermediate ledger account.
Also referred to as a Journal Account.

Chart of Accounts - a list containing all of the the entity's accounts and account numbers.
Account numbers serve as posting references.

Journal - contains a detailed listing,in chronolgical order, of transactions related to
an external account. There are usually multiple Journals to segragate different types of
transactions. The type and number of journals depends on the business and volume of transactions.
Also referred to as an Account Book.  Common examples include:
    1. Cash Journal (to record cash received and disbursed)
    2. Sales Journal (to record sales on credit)
    3. Purchase Journal (to record purchases on credit)
    4. General Journal (to record transactions that don't belong in any other journal)

Journal Entry - one entry in the Ledger

Posting - The act of copying entries from a journal to the ledger. Specifically, the
transactions in the journal are copied to their respective ledger accounts.

Trial Balance - a summary that lists all the ledger accounts with their balances. Usually
with Assets first, followed by Liability and Owner's Equity (Capital, Withdrawals, Revenues,
Expenses). The sum of debit balances 
must equal the sum of credit balances.

Details of Journalizing and Posting
===================================
The journal and the ledger provide details to create a trail through the accounting records
to facilitate auditing and tracing.

Journalizing
------------
A journal transaction should contain the  following information:
   1. Page number on which transaction was recorded
   2. The date the transaction occurred
   3. The accounts debited and credited
   4. An explanation for the transaction
   5. The amount
   6. A Posting Rererence for each account involved

Posting
-------
Posting means copying information from the journal to the respective ledger accounts. The
posting process includes:
For each account involved in the transaction:
   1. Copy the transaction date from the journal to the ledger
   2. Copy the journal page number to the ledger "Journal Reference" column
   3. Copy the amount of the tranaction to the appropriate (debit or credit) column
   4. Copy the account number from the ledger to the "Posting Reference" column of
      the journal transaction. For clarity's sake the Dr account number should precede
      the Cr account number.

Calculating Account Balance
===========================
For each Account, the CurrentBalance is:

    the AccountStatement.ClosingBalance of the previous period, dated the day after the end of the previous period for convenience

    plus the SUM( Transaction.Amounts ) in the current period, where the AccountTransactionType indicates a Debit

    minus the SUM( Transaction.Amount ) in the current period, where the AccountTransactionType indicates a credit

