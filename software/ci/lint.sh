#!/bin/bash

pushd software
  cargo fmt -- --write-mode=diff
popd
