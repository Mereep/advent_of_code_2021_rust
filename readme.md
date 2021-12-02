# Advent of Code 2021

This repository will contain (some of the) algorithmic tasks and riddles presented on https://adventofcode.com/2021. The code is mainly written in Rust. 

**Disclaimer**:
Please note that I am by no means a Rust developer. So *do not* see the code (nor the algorithmic solution) as a blueprint. It may be just bad :-)
## Structure
The data will be structured in a main file (`src/main.rs`) and one module for each day.
Those modules will just be named as `day1` and so on and will contain:
- `task.txt`: textual description if the task
- `testinput.txt`: Small input which can be used for fast algorithmic checks
- `input.txt`: File which contains the actual input data
- `mod.rs`: File containing the code for each day. Usually there will be the two public functions `task1` and `task2`.

## Credits
Please note, that neither the tasks nor the data are my property but rather created by Eric Wastl (Twitter: https://twitter.com/ericwastl).
