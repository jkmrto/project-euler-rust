#!/bin/bash
rustc ./src/$1.rs -o ./target/$1
./target/$1