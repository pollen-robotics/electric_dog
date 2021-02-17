#!/bin/bash

pushd software
  if [ $TARGET = x86_64-unknown-linux-gnu ]; then
    cargo build --examples

  elif [ $TARGET = thumbv6m-none-eabi ]; then
    xargo build --target $TARGET --examples
  fi
popd
