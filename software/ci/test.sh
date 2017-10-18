#!/bin/bash

pushd software
  if [ $TARGET = x86_64-unknown-linux-gnu ]; then
    cargo test
  fi
popd
