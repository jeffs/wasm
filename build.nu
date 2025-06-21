#!/usr/bin/env nu

rm -rf dist
mkdir dist

# Building pages in parallel causes some lock contention, but it's still faster
# than building them serially.
[index life pong primes] |
  par-each {cd $in; trunk build --quiet --release; cp dist/* ../dist}

