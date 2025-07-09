use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;
use Line::*;

static CD_DIR_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\$ cd ([a-z/]+)"#).unwrap());
static CD_UP_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\$ cd \.\."#).unwrap());
static LS_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\$ ls"#).unwrap());
static DIR_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"dir [a-z/]+"#).unwrap());
static FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(\d+) [a-z.]+"#).unwrap());

pub fn solve_a(input: &str) -> usize {
    let lines = input
        .lines()
        .map(Line::from)
        .collect::<Vec<_>>();
    
    let state_machine = StateMachine::new(lines);
    let dir_sizes = state_machine.get_dir_sizes();
    
    dir_sizes
        .values()
        .filter(|size| **size <= 100000)
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    let lines = input
        .lines()
        .map(Line::from)
        .collect::<Vec<_>>();

    let state_machine = StateMachine::new(lines);
    let dir_sizes = state_machine.get_dir_sizes();
    
    let root_size = dir_sizes.get(&Path::new("/".into())).unwrap();
    let unused = 70000000 - root_size;

    dir_sizes
        .values()
        .filter(|size| unused + **size >= 30000000)
        .copied()
        .min()
        .unwrap()
}

struct StateMachine {
    lines: Vec<Line>,
    dir_stack: Vec<Path>,
    dir_sizes: HashMap<Path, usize>
}

impl StateMachine {
    fn new(lines: Vec<Line>) -> Self {
        StateMachine {
            // reverse to allow performant popping
            lines: lines.into_iter().rev().collect(),
            dir_stack: Vec::new(),
            dir_sizes: HashMap::new()
        }
    }
    
    fn get_dir_sizes(mut self) -> HashMap<Path, usize> {
        while !self.lines.is_empty() {
            let line = self.lines.pop().unwrap();
            
            match line {
                CdDir(dir) => {
                    if dir.contains("/") {
                        self.dir_stack.push(Path::new(dir))
                    } else {
                        let mut path = self.dir_stack.last().unwrap().clone();
                        path.add(dir);
                        self.dir_stack.push(path)
                    }
                }
                CdUp => {
                    self.dir_stack.pop();
                }
                Ls => {}
                Dir => {}
                File(size) => {
                    self.dir_stack.iter().for_each(|dir| {
                        *self.dir_sizes.entry(dir.clone()).or_default() += size
                    });
                }
            }
        }
        
        self.dir_sizes
    }
}

// Required, as dir names might occur more than once
#[derive(Clone, Eq, PartialEq, Hash)]
struct Path {
    inner: Vec<String>
}

impl Path {
    fn new(dir: String) -> Self {
        Path {
            inner: vec![dir]
        }
    }
    
    fn add(&mut self, dir: String) {
        self.inner.push(dir)
    }
}

#[derive(Debug)]
enum Line {
    CdDir(String),
    CdUp,
    Ls,
    Dir,
    File(usize),
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        if let Some(capture) = CD_DIR_REG.captures(value) {
            return CdDir(capture.get(1).unwrap().as_str().into());
        }

        if CD_UP_REG.is_match(value) {
            return CdUp;
        }

        if LS_REG.is_match(value) {
            return Ls;
        }

        if DIR_REG.is_match(value) {
            return Dir;
        }

        if let Some(capture) = FILE_REG.captures(value) {
            return File(
                capture.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            );
        }

        panic!("Could not parse line '{value}'")
    }
}
