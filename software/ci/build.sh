#!/bin/bash

pushd software
  if [ $TARGET = x86_64-unknown-linux-gnu ]; then
    cargo build --features "clippy" --examples

  elif [ $TARGET = thumbv6m-none-eabi ]; then
    xargo build --features "clippy" --target $TARGET
  fi
popd
