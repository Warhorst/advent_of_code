# advent_of_code
Solvers for [advent of code](https://adventofcode.com/) puzzles

## Puzzle Solution Categories
### Introduction
A (WIP) listing of the puzzles and the techniques / technologies used to solve them. This should help me find similar
solutions to former puzzles, which might help to solver newer ones faster.

The following techniques are used in almost every puzzle and therefore apply globally:
- Text Parsing: Transforming the text input into domain specific data
- Pattern Matching: Using Rusts pattern matching powers to process complex constructs of data
- Iterator Operations: Using Rusts iterator implementations (and maybe extended functions provided by itertools) to solve the puzzle
- Range Operations: Using Rusts build in range types to solve puzzles using range operations

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

```rust
let set_a: HashSet<_> = [1, 2, 3].into_iter().collect();
let set_b: HashSet<_> = [3, 4, 5].into_iter().collect();

// union
let union = set_a.union(&set_b);
let union = &set_a | &set_b;

// intersection
let intersection = set_a.intersection(&set_b);
let intersection = &set_a & &set_b;

// difference
let difference = set_a.difference(&set_b);
let difference = &set_a - &set_b;

// symmetric difference
let symmetric_difference = set_a.symmetric_difference(&set_b);
let symmetric_difference = &set_a ^ &set_b;
```

Note that the methods will return iterators while the operators will return newly allocated sets.