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
  - [1](https://adventofcode.com/2022/day/1): -
  - [2](https://adventofcode.com/2022/day/2): -
  - [3](https://adventofcode.com/2022/day/3): 
    - [Set Operations](#set-operations): Find the common element in 3 sets

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