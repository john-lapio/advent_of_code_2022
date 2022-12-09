use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Default)]
struct Forest {
    forest: Vec<Vec<char>>,
    visible_trees: Vec<char>,
    scenic_scores: Vec<i32>,
}

struct Tree {
    up_count: i32,
    down_count: i32,
    left_count: i32,
    right_count: i32,
}

impl Forest {
    fn read_input(&self, fi: &str) -> Vec<String> {
        let file = File::open(fi).unwrap();
        let buf = BufReader::new(file);
        buf.lines()
            .map(|line| line.expect("Could not parse line"))
            .collect()
    }

    fn build_forest(&mut self, line: &String) {
        let tree_line = line.chars().collect::<Vec<char>>();
        self.forest.push(tree_line);
    }

    fn check_up(&self, mut tree: &mut Tree, tree_line: &Vec<char>, up: &[Vec<char>], tree_idx: usize) -> bool {
        let mut up_count = 0;
        let mut trees_compared = 0;
        for prev_tree_line in up.iter().rev(){
            trees_compared += 1;
            if tree_line[tree_idx] > prev_tree_line[tree_idx] {
                up_count += 1;
            } else {
                break;
            }
        }
        if up_count == trees_compared {
            tree.up_count = up_count;
            return true;
        } else {
            tree.up_count = up_count + 1;
            return false;
        }
    }

    fn check_down(&self, mut tree: &mut Tree, tree_line: &Vec<char>, down: &[Vec<char>], tree_idx: usize) -> bool {
        let mut down_count = 0;
        let mut trees_compared = 0;
        for post_tree_line in down{
            trees_compared += 1;
            if tree_line[tree_idx] > post_tree_line[tree_idx] {
                down_count += 1;
            } else {
                break;
            }
        }
        if down_count == trees_compared {
            tree.down_count = down_count;
            return true;
        } else {
            tree.down_count = down_count + 1;
            return false;
        }
    }

    fn check_left(&self, mut tree: &mut Tree, tree_line: &Vec<char>, tree_idx: usize) -> bool {
        let mut left_count = 0;
        let horizontal_trees = tree_line[..tree_idx].iter().rev();           

        let num_items = horizontal_trees.len() as i32;
        for tree in horizontal_trees {
            if tree_line[tree_idx] > *tree {
                left_count += 1;
            } else {
                break;
            }
        }
        if left_count == num_items {
            tree.left_count = left_count;
            return true;
        } else {
            tree.left_count = left_count + 1;
            return false;
        }
    }

    fn check_right(&self, mut tree: &mut Tree, tree_line: &Vec<char>, tree_idx: usize) -> bool {
        let mut right_count = 0;
        let horizontal_trees = tree_line[tree_idx+1..].iter();           

        let num_items = horizontal_trees.len() as i32;
        for tree in horizontal_trees {
            if tree_line[tree_idx] > *tree {
                right_count += 1;
            } else {
                break
            }
        } 
        if right_count == num_items {
            tree.right_count = right_count;
            return true;
        } else {
            tree.right_count = right_count + 1;
            return false;
        }
    }

    fn is_visible(&self, tree: &mut Tree, tree_line: &Vec<char>, up: &[Vec<char>], down: &[Vec<char>], tree_idx: usize) -> bool{
        let check = vec![
            self.check_up(tree, tree_line, up, tree_idx), 
            self.check_down(tree, tree_line, down, tree_idx),
            self.check_left(tree, tree_line, tree_idx),
            self.check_right(tree, tree_line, tree_idx),
        ];
        if check.contains(&true) {
            return true;
        } else {return false;}
    }

    fn find_visible_trees(&mut self){
        self.visible_trees.extend_from_slice(self.forest.first_mut().unwrap());
        self.visible_trees.extend_from_slice(self.forest.last_mut().unwrap());
        for tree_line_idx in 1..self.forest.len()-1 {
            self.visible_trees.push(*self.forest[tree_line_idx].first().unwrap());
            self.visible_trees.push(*self.forest[tree_line_idx].last().unwrap());
            let tree_line = &self.forest[tree_line_idx];
            let up = &self.forest[..tree_line_idx];
            let down = &self.forest[tree_line_idx+1..];

            for tree_idx in 1..self.forest[tree_line_idx].len()-1 {
                let mut tree = Tree {
                    up_count: 0,
                    down_count: 0,
                    left_count: 0,
                    right_count: 0
                };
                let visible = self.is_visible(&mut tree, tree_line, up, down, tree_idx);
                match visible {
                    true => {
                        self.visible_trees.push(tree_line[tree_idx]);
                    },
                    false => (),
                }
                self.scenic_scores.push(tree.up_count * tree.down_count * tree.left_count * tree.right_count);
            }
        }
    }

    pub fn process_part1(&mut self, input: &Vec<String>){
        for line in input{
            self.build_forest(line);
        }
        self.find_visible_trees();
        println!("{:?}",self.visible_trees.len());
    }

    pub fn process_part2(&mut self){
        println!("{:?}",self.scenic_scores.iter().max().unwrap());
    }
}

fn main() {
    let mut forest = Forest{ 
        forest: vec![],
        visible_trees: vec![],
        scenic_scores: vec![],
    };
    let input = forest.read_input("puzzle.txt");
    forest.process_part1(&input);
    forest.process_part2()
}