#!/bin/bash
# Uses the shared .zok function and proving key to generate
# a zero-knowledge proof.
# Takes the secret key as a string argument

# To convince yourself that this works, create a
# new directory containing this file, hashexample.zok
# customSha256.zok and proving.key then run this script.

# Convert input string into decimal output
input=$1
hex=`echo $input | xxd -p`
dec=`echo $((0x$hex))`
echo "Secret hash preimage is $dec"


# generate proof
zokrates compile -i hashexample.zok -o hashexample
zokrates compute-witness -i hashexample -a $dec
zokrates generate-proof -i hashexample


# cleanup
rm witness abi.json hashexample
