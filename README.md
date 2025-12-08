# advent_of_code
Solves for [advent of code](https://adventofcode.com/) puzzles.

## Introduction
A (WIP) listing of the puzzles and the techniques / technologies used to solve them. This should help me find similar
solutions to former puzzles, which might help to solve newer ones faster.

The following techniques are used in almost every puzzle and therefore apply globally:
- Text Parsing: Transforming the text input into domain specific data
- Pattern Matching: Using Rusts pattern matching powers to process complex constructs of data
- Iterator Operations: Using Rusts iterator implementations (and maybe extended functions provided by itertools) to solve the puzzle
- Range Operations: Using Rusts build in range types to solve puzzles using range operations

A '-' means a puzzle had no extraordinary problem or required special techniques to be solved.

In rare cases, I needed external help to solve a puzzle. I credit the source in a code comment on the puzzle solver.

## Puzzle Solutions
- 2022
  - [1](https://adventofcode.com/2022/day/1) ([Code](./src/y2022/d1.rs)): -
  - [2](https://adventofcode.com/2022/day/2) ([Code](./src/y2022/d2.rs)): -
  - [3](https://adventofcode.com/2022/day/3) ([Code](./src/y2022/d3.rs)): Find the common element in 3 sets using intersections
    - [Set Operations](#set-operations)
  - [4](https://adventofcode.com/2022/day/4) ([Code](./src/y2022/d4.rs)): -
  - [5](https://adventofcode.com/2022/day/5) ([Code](./src/y2022/d5.rs)):
    - [Complex Input Parsing](#complex-input-parsing)
    - [Regexes](#regexes)
  - [6](https://adventofcode.com/2022/day/6) ([Code](./src/y2022/d6.rs)): -
  - [7](https://adventofcode.com/2022/day/7) ([Code](./src/y2022/d7.rs)): Keeping track of directory sizes using console like output (this might also been solvable with a tree) 
    - [State Machine](#state-machine)
    - [Regexes](#regexes)
  - [8](https://adventofcode.com/2022/day/8) ([Code](./src/y2022/d8.rs)): Transform the input into a board and solve the puzzle using board operations 
    - [Board](#board)
  - [9](https://adventofcode.com/2022/day/9) ([Code](./src/y2022/d9.rs)): Move a snake like structure across a board
  - [10](https://adventofcode.com/2022/day/10) ([Code](./src/y2022/d10.rs)): Simulate a computer and computer screen by parsing a list of commands, updating the state based on them and move a cursor to draw on the screen
    - [Regexes](#regexes)
    - [State Machine](#state-machine)
  - [11](https://adventofcode.com/2022/day/11) ([Code](./src/y2022/d11.rs)): Perform addition and multiplication to increase numbers indefinitily, but only their remainders are important. This puzzle also requires the use of multiple different divisors. 
    - [Complex Input Parsing](#complex-input-parsing)
    - [Modular Arithmetic](#modular-arithmethic) 
  - [12](https://adventofcode.com/2022/day/12) ([Code](./src/y2022/d12.rs)): Find the shortest path up a mountain using a given heightmap
    - [Pathfinding](#pathfinding)
- 2024
  - [1](https://adventofcode.com/2024/day/1) ([Code](./src/y2024/d1.rs)): -
  - [2](https://adventofcode.com/2024/day/2) ([Code](./src/y2024/d2.rs)): -
  - [3](https://adventofcode.com/2024/day/3) ([Code](./src/y2024/d3.rs)): Extract and parse commands from a string and perform the commands to multiply numbers.
    - [Regexes](#regexes)
  - [4](https://adventofcode.com/2024/day/4) ([Code](./src/y2024/d4.rs)): Given a 2D array of letters, a word and shape search must be performed.
    - [Board](#board)
  - [5](https://adventofcode.com/2024/day/5) ([Code](./src/y2024/d5.rs)): Check if a sequence of numbers is correctly ordered, based on a set of rules. Later, sort incorrect sequences using these rules.
    - [Complex Input Parsing](#complex-input-parsing)
  - [6](https://adventofcode.com/2024/day/6) ([Code](./src/y2024/d6.rs)): Simulate a robot moving around a board until it leaves the area. Afterwards, find all positions which would cause the robot to get stuck in a loop.
    - [Board](#board)
    - [State Machine](#state-machine)
  - [7](https://adventofcode.com/2024/day/7) ([Code](./src/y2024/d7.rs)): Find the operators which fulfill a given equation.
    - [Permutations](#permutations)
  - [8](https://adventofcode.com/2024/day/8) ([Code](./src/y2024/d8.rs)): Handle relations between antennas on a map
    - [Board](#board)
  - [9](https://adventofcode.com/2024/day/9) ([Code](./src/y2024/d9.rs)): Compress and create the checksum of a "file system" based on a disk map
  - [10](https://adventofcode.com/2024/day/10) ([Code](./src/y2024/d10.rs)): Find trails on a mountain based on a heightmap
    - [Board](#board)
  - [11](https://adventofcode.com/2024/day/11) ([Code](./src/y2024/d11.rs)): Count the number of stones which multiply after each iteration based on a set of rules
    - [Dynamic Programming](#dynamic-programming)
  - [12](https://adventofcode.com/2024/day/12) ([Code](./src/y2024/d12.rs)): Determine the sides of randomly shaped areas (might also be solvable with polygons)
  - [13](https://adventofcode.com/2024/day/13) ([Code](./src/y2024/d13.rs)): Solve a given set of linear equations (If I understand this one day, I could create a section regarding linear algebra)
  - [14](https://adventofcode.com/2024/day/14) ([Code](./src/y2024/d14.rs)): Move robots around a board based on a given set of parameters
    - [Board](#board)
    - [Visualization](#visualization)
- 2025
  - [1](https://adventofcode.com/2025/day/1) ([Code](./src/y2025/d1.rs)): Simulate a safe dial
  - [2](https://adventofcode.com/2025/day/2) ([Code](./src/y2025/d2.rs)): Determine if a string consists of repeating patterns
  - [3](https://adventofcode.com/2025/day/3) ([Code](./src/y2025/d3.rs)): Find the combination of digits which form the largest value
    - [Permutations](#permutations)
  - [4](https://adventofcode.com/2025/day/4) ([Code](./src/y2025/d4.rs)): Filter tiles on a board based on their neighbours
    - [Board](#board)
  - [5](https://adventofcode.com/2025/day/5) ([Code](./src/y2025/d5.rs)): Return the amount of elements in a given list of ranges
  - [6](https://adventofcode.com/2025/day/6) ([Code](./src/y2025/d6.rs)): Perform a math calculation based on a given equation. Part 2 is solved by transforming the input into a board and reading the columns.
    - [Complex Input Parsing](#complex-input-parsing)
    - [Board](#board)
  - [7](https://adventofcode.com/2025/day/7) ([Code](./src/y2025/d7.rs)): Count the times a beam can split when shot through a map and how many different possibilities exist for the beam to travel
    - [Board](#board)
    - [Dynamic Programming](#dynamic-programming)
  - [8](https://adventofcode.com/2025/day/8) ([Code](./src/y2025/d8.rs)): Find the shortest connections between a given set of positions
    - [Graph](#graph) (Kruskal's Algorithm)

## Problems and Techniques Explained
### Complex Input Parsing
The solution involves parsing a rather complex text input, like an ASCII drawing of some scene or plan. 
The challenge is to quickly find an algorithm to transform the input into domain data, using some implementation of From<&str>.

Transforming the input by hand is not considered a valid solution in my opinion, as this is against the spirit of the challenge.

How to approach this depends on the puzzle, but here are some strategies used in the solutions:
- if the input consists of multiple blocks representing different kinds of input, use ``input.split("\n\n")`` to split and process them separately
- sometimes the input contains junk data, just to make it look cooler, so use ``line.replace(to_replace, replacement)`` to transform the text into a more processable form
- sometimes the order of the lines is not optimal, so use ``input.lines().rev()`` to process them from bottom to top
- sometimes the lines themselves consist of blocks, so use ``line.chars().windows(block_size).step_by(step_size)`` to process them

### Regexes
The solution involves using [regexes](https://en.wikipedia.org/wiki/Regular_expression) to extract relevant data from the input.

For convenience, I created the ``regex_captures`` function to easily process all captures returned from a regex process.
It returns an iterator over all matches of the regex and provides a closure processing the captures in the finding (which is just an array of &str):

```rust
let input = "move 2 from 1 to 3";
let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

regex_captures(
  input,
  &regex,
  |caps| format!("move: {}, from: {}, to: {}", caps[0], caps[1], caps[2])
)
```

To easily parse a list of strings to a specific type using regexes, I created the the ``from_regex`` proc macro attribute. It
can be used on structs and enums and generates a method ``from_regex``, which takes a string (the haystack) and creates the
type instance from it. More information can be found in the macros doc. Example:

A list of instructions like this

```
addx 13
addx 4
noop
```

can be parsed to the type `Instruction` like this

```rust
#[derive(Clone, Copy, Debug)]
#[from_regex]
enum Instruction {
    #[reg(r#"noop"#)]
    Noop,
    #[reg(r#"addx (-*\d+)"#)]
    Add(isize)
}  
```

### Set Operations
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


### State Machine
The solution involves parsing the input into a chain of actions and performing an associated
action in a state machine, mutating its state and returning it as the solution, or as part
of the solution. This might involve parsing the input to enum variants and using pattern
matching.


### Board
The solution involves transforming the input into a Board data structure and perform operations on it using the Board methods.
Examples for this are:
- moving a piece from a game or simulation around
- iterating over all the board positions/rows/columns
- accessing board values, given a position


### Modular Arithmethic
The solution has something to do with modulus calculations, like `num % 5`. If only the modulus is the interesting part of the puzzle,
it might be possible to apply properties of the [Modular Arithmethic](https://en.wikipedia.org/wiki/Modular_arithmetic) to it.


### Pathfinding
The solution involves determining the shortest path from one given point to another. Most of the times, a [Board](#board) is involved,
as it contains the actual tiles which are part of the pathfinding problem.

The easiest way to solve this is the [pathfinding](https://github.com/evenfurther/pathfinding) crate and its A* implementation.
A simple example which also uses a board:

```rust
let board = Board::<Tile>::from(input);
let start = p!(0, 0);
let end = p!(32, 32);

let path_res = pathfinding::prelude::astar(
  // The start position
  &start,
  // The successor function which tells what other positions are reachable from the current one and with what weight
  |pos| pos.cardinal_neighbours().filter(|p| board.pos_in_bounds(*p)).map(|p| (p, board.get_tile(*p).unwrap().weight())),
  // The heuristic function, which gives an approximation of the distance from the current position to the goal
  |pos| pos.manhattan_distance(&end),
  // The function which checks if the current position has reached the goal
  |pos| *pos == end
);

match path_res {
  // path.0 are all the positions in the path and path.1 is the length of the path
  Some(path) => println!("Path found"),
  None => println!("Path not found :(")
}
```

### Permutations
The solutions involves finding a specific value or values in a given permutation. The problem is that the permutation
could contain millions or even billions of values.

General strategies:
- Don't try them all. The easier puzzles might be solvable by just iterating over all values, but the harder ones require special tricks
which depend on the puzzle. Often, there is some criteria which filters many of them out.
- Be really fast. Sometimes, even large permutations can be iterated through in acceptable time if the performed operation can be executed quickly.
Using libraries like [rayon](https://github.com/rayon-rs/rayon) can also help to increase parallelization.


### Dynamic Programming
The solution involves the usage of [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming), which means splitting the problem
into sub-problems and solving them recursively.

This is often acompanied by [Memoization](https://en.wikipedia.org/wiki/Memoization), which is just a fancy term for "storing interim results in a cache (like a HashMap)".


### Visualization
The solution involves rendering something and observing the solution or parts of the solution. Examples are:
- rendering ascii art to the console
- rendering a graph, shape, etc. using external software

There are rare cases where the puzzle expects the human solving it to interpret the visualized output and create the solution input from it. For example,
one puzzle renders letters to the console, and the puzzle expects the rendered output interpreted as a string.

These puzzles can be hard to automate. One (really slow) approach is to use the contains_shape method of the Board:

```rust
let shape = Shape::from(shape_string);

if board.contains_shape(shape) {
  // solved
}
  
```
### Graph
The solutions involves interpreting the input as a graph and analyzing it, mostly by using graph algorithms.

The (currently not used) [petgraph](https://docs.rs/petgraph/latest/petgraph/) library might be usable in the future to work with graphs.

The following sections contain some algorithms which are currently used in puzzle solutions.

#### Edges of an undirected graph (without self loops)
The following code shows how all edges of an undirected graph with no self loops can be determined:

```rust
let vertices: Vec<Vertex> = ...;
let num_vertices = vertices.len();
let num_edges = (num_vertices * (num_vertices - 1)) / 2;

let mut edges = Vec::with_capacity(num_vertices);

for i in 0..num_vertices {
    for j in i + 1..num_vertices {
        edges.push((vertices[i], vertices[j]));
    }
}
  
```

#### Kruskal's Algorithm
[Kruskal's Algorithm](https://en.wikipedia.org/wiki/Kruskal%27s_algorithm) can be used to find the minimum spanning tree of a graph.
