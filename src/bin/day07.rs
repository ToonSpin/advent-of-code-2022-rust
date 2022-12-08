use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::streaming::none_of;
use nom::combinator::{recognize, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Clone, Debug)]
enum CommandLineEntry {
    CdCommand(String),
    CdUpCommand,
    LsCommand,
    Dir(String),
    File(String, u32),
}

#[derive(Debug)]
enum FsNode {
    File(u32),
    Directory(Option<HashMap<String, FsNode>>),
}

impl FsNode {
    fn from_cli_entries(v: &Vec<CommandLineEntry>) -> FsNode {
        let (_, node) = Self::traverse_cli_entries(v, 2);
        node
    }

    fn traverse_cli_entries(v: &Vec<CommandLineEntry>, mut index: usize) -> (usize, FsNode) {
        let mut entries = HashMap::new();
        while index < v.len() {
            match &v[index] {
                CommandLineEntry::Dir(name) => {
                    entries.insert(name.clone(), FsNode::Directory(None));
                    index += 1;
                }
                CommandLineEntry::File(name, size) => {
                    entries.insert(name.clone(), FsNode::File(*size));
                    index += 1;
                }
                _ => {
                    break;
                }
            }
        }
        while index < v.len() {
            if let CommandLineEntry::CdUpCommand = &v[index] {
                return (index, FsNode::Directory(Some(entries)));
            }
            if let CommandLineEntry::CdCommand(name) = &v[index] {
                let (new_index, entry) = Self::traverse_cli_entries(v, index + 2);
                entries.insert(name.clone(), entry);
                index = new_index;
            }
            index += 1;
        }
        (index, FsNode::Directory(Some(entries)))
    }

    fn get_directories_with_sizes(&self) -> (u32, Vec<u32>) {
        match self {
            FsNode::File(size) => (*size, vec![]),
            FsNode::Directory(None) => unreachable!(),
            FsNode::Directory(Some(m)) => {
                let mut total_size = 0;
                let mut result = Vec::new();
                for (_, entry) in m.iter() {
                    let (subtotal, mut dirs) = entry.get_directories_with_sizes();
                    result.append(&mut dirs);
                    if let FsNode::Directory(_) = entry {
                        result.push(subtotal);
                    }
                    total_size += subtotal;
                }
                (total_size, result)
            }
        }
    }
}

fn parse_rest_of_line(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of("\n\r")))(input)
}

fn parse_cd_command(input: &str) -> IResult<&str, CommandLineEntry> {
    let (rest, dir) = preceded(tag("$ cd "), parse_rest_of_line)(input)?;
    Ok((rest, CommandLineEntry::CdCommand(String::from(dir))))
}

fn parse_ls_command(input: &str) -> IResult<&str, CommandLineEntry> {
    value(CommandLineEntry::LsCommand, tag("$ ls"))(input)
}

fn parse_cd_up_command(input: &str) -> IResult<&str, CommandLineEntry> {
    value(CommandLineEntry::CdUpCommand, tag("$ cd .."))(input)
}

fn parse_dir_entry(input: &str) -> IResult<&str, CommandLineEntry> {
    let (rest, dir) = preceded(tag("dir "), parse_rest_of_line)(input)?;
    Ok((rest, CommandLineEntry::Dir(String::from(dir))))
}

fn parse_file_entry(input: &str) -> IResult<&str, CommandLineEntry> {
    let (rest, (size, filename)) = separated_pair(digit1, tag(" "), parse_rest_of_line)(input)?;
    let size = size.parse().unwrap();
    Ok((rest, CommandLineEntry::File(String::from(filename), size)))
}

fn parse_entry(input: &str) -> IResult<&str, CommandLineEntry> {
    alt((
        parse_cd_up_command,
        parse_cd_command,
        parse_ls_command,
        parse_dir_entry,
        parse_file_entry,
    ))(input)
}

fn parse_entries(input: &str) -> IResult<&str, Vec<CommandLineEntry>> {
    separated_list1(tag("\n"), parse_entry)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let (_, entries) = parse_entries(input).unwrap();
    let fs = FsNode::from_cli_entries(&entries);

    let (total_size, mut subdirs) = fs.get_directories_with_sizes();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;
    subdirs.sort();

    let part1: u32 = subdirs.iter().filter(|&size| *size <= 100000).sum();
    println!("The total size of all smaller directories: {}", part1);

    for size in subdirs.iter() {
        if *size >= space_needed {
            println!("The size of the smallest directory to delete: {}", size);
            break;
        }
    }

    Ok(())
}
