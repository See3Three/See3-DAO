# See3 DAO and Gov'nor

This implements the See3 Governance DAO, and the central governance interface: gov'nor. 

## How Do I Use Gov'nor?

Just execute `./govnor.sh` with the appropriate options from inside the `See3_DAO` directory.

```See3 Gov\'nor CLI

Usage: ./govnor.sh [option] ...

   --suri              Your private key, formatted as a hex string that begins with 0x.
   transfer            Send funds to another contract.
                       Usage: ./govnor.sh transfer RECIPIENT_ADDRESS AMOUNT --suri SURI
   kick-keepers        Initiate a vote to replace a keeper.
                       Usage: ./govnor.sh kick-keepers OLD_KEEPER NEW_KEEPER --suri SURI
   untrust             Remove a trusted authority.
                       Usage: ./govnor.sh untrust ACCOUNT_ID --suri SURI
   trust               Add a trusted authority.
                       Usage: ./govnor.sh trust ACCOUNT_ID TRUST_KEY --suri SURI
   get-voting-power    Get the voting power of an account.
                       Usage: ./govnor.sh get-voting-power ACCOUNT_ID
   get-balance         Get the balance of an account.
                       Usage: ./govnor.sh get-balance ACCOUNT_ID
   finalize-vote       Finalize the current vote.
                       Usage: ./govnor.sh finalize-vote --suri SURI
   withdraw            Withdraw funds.
                       Usage: ./govnor.sh withdraw --suri SURI
```

## Why Is Gov'nor CLI Only?

We're working on a smooth web interface for governance, but for now, this is the most user-friendly way to interact with the system.

The CLI works reliably, and is extremely straightforward. 