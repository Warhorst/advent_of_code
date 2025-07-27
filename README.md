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
  - [5](https://adventofcode.com/2022/day/5) ([Code](./src/y2022/d5.rs)):
    - [Complex Input Parsing](#complex-input-parsing)
    - [Regexes](#regexes)
  - [6](https://adventofcode.com/2022/day/6) ([Code](./src/y2022/d6.rs)): -
  - [7](https://adventofcode.com/2022/day/7) ([Code](./src/y2022/d7.rs)):
    - [State Machine](#state-machine): Keeping track of directory sizes using console like output (this might also been solvable with a tree)
    - [Regexes](#regexes): Console-output to enum parsing
  - [8](https://adventofcode.com/2022/day/8) ([Code](./src/y2022/d8.rs))
    - [Board](#board): Transform the input into a board and solve the puzzle using board operations

### Techniques Explained
#### Complex Input Parsing
The solution involves parsing a rather complex text input, like an ASCII drawing of some scene or plan. 
The challenge is to quickly find an algorithm to transform the input into domain data, using some implementation of From<&str>.

Transforming the input by hand is not considered a valid solution in my opinion, as this is against the spirit of the challenge.

How to approach this depends on the puzzle, but here are some strategies used in the solutions:
- if the input consists of multiple blocks representing different kinds of input, use ``input.split("\n\n")`` to split and process them separately
- sometimes the input contains junk data, just to make it look cooler, so use ``line.replace(to_replace, replacement)`` to transform the text into a more processable form
- sometimes the order of the lines is not optimal, so use ``input.lines().rev()`` to process them from bottom to top
- sometimes the lines themselves consist of blocks, so use ``line.chars().windows(block_size).step_by(step_size)`` to process them

#### Regexes
The solution involves using regexes to extract relevant data from the input.

For convenience, I created the ``regex_captures`` function to easily process all captures returned from a regex process.
It returns an iterator over all matches of the regex and provides a closure processing the captures in the finding (which are just an array of &str)

```rust
let input = "move 2 from 1 to 3";
let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

regex_captures(
  value,
  &regex,
  |caps| format!("move: {}, from: {}, to: {}", caps[0], caps[1], caps[2])
)
```

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


#### State Machine
The solution involves parsing the input into a chain of actions and performing an associated
action in a state machine, mutating its state and returning it as the solution, or as part
of the solution. This might involve parsing the input to enum variants and using pattern
matching.


#### Board
The solution involves transforming the input into a Board data structure and perform operations on it using the Board methods.
Examples for this are:
- moving a piece from a game or simulation around
- iterating over all the board positions/rows/columns
- accessing board values, given a position