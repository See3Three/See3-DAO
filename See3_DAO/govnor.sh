#!/bin/bash

CONTRACT_ADDRESS="5DhQMCBCP7QibiRPcJ8oZSFLYeEWWknVkuicjQPydTcq5gZa"
DEFAULT_RPC_URL="ws://localhost:9944"
RPC_URL="${RPC_URL:-$DEFAULT_RPC_URL}"

# Check if cargo contract is installed
if ! command -v cargo-contract &> /dev/null; then
    echo "cargo contract could not be found. Please install it before running this script."
    exit 1
fi

display_help() {
    echo "See3 Gov\'nor CLI"
    echo
    echo "Usage: $0 [option] ..."
    echo
    echo "   --suri              Secret URI for signing extrinsic."
    echo "   transfer            Send funds to another contract."
    echo "                       Usage: $0 transfer RECIPIENT_ADDRESS AMOUNT --suri SURI"
    echo "   kick-keepers        Initiate a vote to replace a keeper."
    echo "                       Usage: $0 kick-keepers OLD_KEEPER NEW_KEEPER --suri SURI"
    echo "   untrust             Remove a trusted authority."
    echo "                       Usage: $0 untrust ACCOUNT_ID --suri SURI"
    echo "   trust               Add a trusted authority."
    echo "                       Usage: $0 trust ACCOUNT_ID TRUST_KEY --suri SURI"
    echo "   get-voting-power    Get the voting power of an account."
    echo "                       Usage: $0 get-voting-power ACCOUNT_ID"
    echo "   get-balance         Get the balance of an account."
    echo "                       Usage: $0 get-balance ACCOUNT_ID"
    echo "   finalize-vote       Finalize the current vote."
    echo "                       Usage: $0 finalize-vote --suri SURI"
    echo "   withdraw            Withdraw funds."
    echo "                       Usage: $0 withdraw --suri SURI"
    echo
}

if [ $# -lt 1 ]; then
    display_help
    exit 1
fi

ACTION=$1
SURI=""

case $ACTION in
    transfer)
        if [ $# -ne 4 ]; then
            echo "Invalid number of arguments for transfer."
            exit 1
        fi
        RECIPIENT=$2
        AMOUNT=$3
        SURI=$4
        cargo contract call --url $RPC_URL --contract $CONTRACT_ADDRESS --message transfer --args "$RECIPIENT $AMOUNT" --suri $SURI
        ;;
    kick-keepers)
        if [ $# -ne 4 ]; then
            echo "Invalid number of arguments for kick-keepers."
            exit 1
        fi
        OLD_KEEPER=$2
        NEW_KEEPER=$3
        SURI=$4
        cargo contract call --url $RPC_URL --contract $CONTRACT_ADDRESS --message initialize_vote --args "ChangeKeeper($OLD_KEEPER, $NEW_KEEPER)" --suri $SURI
        ;;
    untrust)
        if [ $# -ne 3 ]; then
            echo "Invalid number of arguments for untrust."
            exit 1
        fi
        ACCOUNT_ID=$2
        SURI=$3
        cargo contract call --url $RPC_URL --contract $CONTRACT_ADDRESS --message initialize_vote --args "RemoveFromTrustList($ACCOUNT_ID)" --suri $SURI
        ;;
    trust)
        if [ $# -ne 4 ]; then
            echo "Invalid number of arguments for trust."
            exit 1
        fi
        ACCOUNT_ID=$2
        TRUST_KEY=$3
        SURI=$4
        cargo contract call --url $RPC_URL --contract $CONTRACT_ADDRESS --message initialize_vote --args "AddToTrustList($ACCOUNT_ID, $TRUST_KEY)" --suri $SURI
        ;;
    get-voting-power | get-balance)
        if [ $# -ne 2 ]; then
            echo "Invalid number of arguments for $ACTION."
            exit 1
        fi
        ACCOUNT_ID=$2
        echo "This action ($ACTION) would typically involve querying the blockchain state directly, which cannot be accomplished solely via cargo contract."
        ;;
    finalize-vote)
        if [ $# -ne 2 ]; then
            echo "Invalid number of arguments for finalize-vote."
            exit 1
        fi
        SURI=$2
        cargo contract call --url $RPC_URL --contract $CONTRACT_ADDRESS --message finalize_vote --suri $SURI
        ;;
    withdraw)
        if [ $# -ne 2 ]; then
            echo "Invalid number of arguments for withdraw."
            exit 1
        fi
        SURI=$2
        cargo contract call --url $RPC_URL --contract $CONTRACT_ADDRESS --message withdraw --suri $SURI
        ;;
    *)
        display_help
        ;;
esac

