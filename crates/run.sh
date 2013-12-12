#!/bin/bash
for f in ./$1/*.mod.rs
do
  rustc --lib $f
done

rustc $1/main.rs -L $1
$1/main
