#!/bin/sh

CARGO_FEATURE_PURE=1 cargo build --target x86_64-pc-windows-gnu # &&
cp target/x86_64-pc-windows-gnu/debug/cloud-jumper.exe . &&
for f in target/x86_64-pc-windows-gnu/debug/*.dll; do
  cp "$f" .
done

exec ./cloud-jumper.exe "$@"
