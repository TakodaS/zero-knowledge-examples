#!/bin/bash
# Uses .zok function and proving key to generate
# a zero-knowledge proof.
# Takes the secret key as a string argument

# Convert input string into decimal output
input=$1
hex=`echo $input | xxd -p`
dec=`echo $((0x$hex))`
echo "Secret hash preimage is $dec"


# generate proof
zokrates compile -i hashexample.zok -o hashexample
zokrates generate-witness -i hashexample -a $dec
zokrates generate-proof -i hashexample


# cleanup
rm witness abi.json hashexample
