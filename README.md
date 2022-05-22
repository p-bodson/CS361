# CS361

## Money

Here is my CS361 project.  It is called Money.

Money is a command double-entry accounting ledger like the ledger-cli
from https://www.ledger-cli.org/

The goal of creating this ledger program, however, is to uphold the 
Cognitive Style Heuristics, and utilize a microservice architecture.

The base program is the ledger that a JSON file containing the accounts
and transactions.

The basic features are viewing a balance, the chart of accounts, 
and a register of an account.  One can also add and delete
transactions and accounts.

## CSH

Here is how I will fulfill the CSH

1. Explain the benefits of using new and existing features

    a. Whenever a new feature is added, a short note will be provided when a the program is first opened. And a link to a tutorial for the new feature will be provided.

2. Explain the costs of using new and existing features

    a. If the features will cause some kind of damage, warning text will be provided.

3. Let people gather as much information as they
want, and no more than they want

    a. A manual will be provided for features and common use cases.

4. Keep familiar features available
   
    a. If you are using a double-entry accounting ledger, then you are probably already familiar with accounting.  Consequently, this app uses the same terminolgy such as balance, register, P/L, and accounts.

5. Make undo/redo and backtracking available

    a. The app supports making backups of your data and an unwind feature to undo the last transaction added.

6. Provide an explicit path through the task

    a. The app will have buttons to provide guided text through adding a transaction.

7. Provide ways to try out different approaches

    a. There is the interactive version, the command line version, and the file version.

8.  Encourage tinkerers to tinker mindfully

    a. Bad, destructive, irreparable things will be prefaced with warnings and forced to make a snapshot before effect so rollbacks can occur.
