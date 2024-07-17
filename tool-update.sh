#!/bin/bash

# Locally update the VM version

VM_TAG="v1.5.1"

echo "Before:"
moapy config dump
moapy config set dependencies.vmtools.tag $VM_TAG
echo "After:"
moapy config dump

moapy deps install vmtools --overwrite

# Also update the Rust version

moapy deps install rust --tag="nightly" --overwrite
