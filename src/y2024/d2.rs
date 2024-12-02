use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split(" ").map(|split| parse::<usize>(split)).collect::<Vec<_>>())
        //.inspect(|(kind, nums)| println!("{:?} {:?}", kind, nums))
        .filter(|nums| is_save(nums))
        //.inspect(|(kind, nums)| println!("{:?} {:?}", kind, nums))
        .count()
}

pub fn solve_b(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split(" ").map(|split| parse::<usize>(split)).collect::<Vec<_>>())
        //.inspect(|(kind, nums)| println!("{:?} {:?}", kind, nums))
        .filter(|nums| (0..nums.len()).into_iter().any(|i| {
            let mut clone = nums.clone();
            clone.remove(i);
            is_save(&clone)
        }))
        //.inspect(|(kind, nums)| println!("{:?} {:?}", kind, nums))
        .count()
}

fn is_save(nums: &Vec<usize>) -> bool {
    if nums[0] < nums[1] {
        nums.iter().enumerate().skip(1).all(|(i, num)| nums[i - 1] < *num && nums[i - 1].abs_diff(*num) <= 3)
    } else if nums[0] > nums[1] {
        nums.iter().enumerate().skip(1).all(|(i, num)| nums[i - 1] > *num && nums[i - 1].abs_diff(*num) <= 3)
    } else {
        false
    }
}
