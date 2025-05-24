# advent_of_code
Solvers for [advent of code](https://adventofcode.com/) puzzles

## Puzzle Solution Categories
### Introduction
A (WIP) listing of the puzzles and the techniques / technologies used to solve them. This should help me find similar
solutions to former puzzles, which might help to solver newer ones faster.

The following techniques are used in almost every puzzle and therefore apply globally:
- text parsing
- (complex) pattern matching
- Iterator Operations

A '-' means a puzzle did not require special techniques to be solved

### Techniques used in puzzle solutions
- 2022
  - [1](https://adventofcode.com/2022/day/1) ([Code](./src/y2022/d1.rs)): -
  - [2](https://adventofcode.com/2022/day/2) ([Code](./src/y2022/d2.rs)): -
  - [3](https://adventofcode.com/2022/day/3) ([Code](./src/y2022/d3.rs)): 
    - [Set Operations](#set-operations): Find the common element in 3 sets using intersections
  - [4](https://adventofcode.com/2022/day/4) ([Code](./src/y2022/d4.rs)): -

### Techniques Explained
#### Set Operations
The solution involves transforming the input into sets and performing
[basic set operations](https://en.wikipedia.org/wiki/Set_(mathematics)#Basic_operations) on them to solve the problem.

Rust and its HashSet provide methods and operators to perform set operations:
- Union: union or |
- Intersection: intersection or &
- Difference: difference or -
- Symmetric Difference: symmetric_difference or ^

Note that the methods will return iterators while the operators will return newly allocated sets.