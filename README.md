# Competitive Programming and Contest

This repository contains all the solutions for the problems specified for the [Competitive Programming and Contest Course](https://pages.di.unipi.it/rossano/competitive/).

This repository is organized as a standalone rust crate.

## Mandatory Problems
In the `src/mandatory` there can be found the solutions to all mandatory problems, each one comprising a test module that can be verified via `cargo test` from the base directory.

## Optional Problems
In the `src/optional` there can be found the solutions to MOST optional problems. Each problem is located in a `set*` directory. Each set is numbered and contains all problems of the lecture with the same number.
All problems map to a corresponding `test_<problem_name>.rs` file, located in the `test` directory.

> I know, it's really messy, the more I kept solving problems the more difficult became to fix it

## Hands On Assignments
In the base directory, we have the folders:
- `hands_on_1`: solution to Tree Traversal problems
- `hands_on_2`: solution to Segment Trees problems, comprehensive of explanatory main file and 
automated test suite verifiable via `cargo test` from the crate directory
- `hands_on_2`: solution to Dynamic programming problems, comprehensive of explanatory main file and 
automated test suite verifiable via `cargo test` from the crate directory

Other than that each folder, contains the respective hands on report both in .md and .pdf format