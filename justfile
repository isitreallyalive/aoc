create year day +name:
    #!/usr/bin/env bash
    day=$(printf "%02d" {{day}})
    cargo new {{year}}/$day-{{lowercase(replace(name, " ", "-"))}} --name aoc{{year}}-$day

run year day:
    @cargo r -p aoc{{year}}-$(printf "%02d" {{day}})