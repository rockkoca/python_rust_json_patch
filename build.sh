#!/usr/bin/env bash

#!/bin/bash

# Clean previous builds
rm -rf target/wheels/*

# Build the wheel
maturin build --release