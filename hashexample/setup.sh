#!/bin/bash
# Simplified setup of verification key and 
# associated verifier.sol smart contract.
# a zero-knowledge proof.
# Takes the secret key as a string argument

# Convert input string into decimal output
input=$1
hex=`echo $input | xxd -p`
dec=`echo $((0x$hex))`
echo "Secret hash preimage is $dec"

# Create .zok function verified by this preimage
zokrates compile -i customSha256.zok -o customSha256
zokrates compute-witness -i customSha256 -a $dec
cargo run 

# generate verification key
zokrates compile -i hashexample.zok -o hashexample
zokrates setup -i hashexample

# Optional step to generate a solidity smart contract
zokrates export-verifier

# cleanup
rm customSha256 witness abi.json hashexample
