#!/usr/bin/env nu

[layout magic perf system easel index life primes] |
  par-each {cd $in; cargo --quiet clean}

rm -rf dist
mkdir dist

# Building pages in parallel causes some lock contention, but it's still faster
# than building them serially.
[index life primes] |
  par-each {cd $in; trunk build --quiet --release; mv dist/* ../dist}

