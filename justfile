new year day +name:
    #!/usr/bin/env bash
    day=$(printf "%02d" {{day}})
    path={{year}}/$day-{{lowercase(replace(name, " ", "-"))}}
    cargo new $path --name aoc{{year}}-$day
    cd $path && cargo add aoc

run year day:
    @cargo r -p aoc{{year}}-$(printf "%02d" {{day}})