#!/bin/env fish

for line in (cat input.txt)
  ./target/release/day1 $line >> output.txt
end
