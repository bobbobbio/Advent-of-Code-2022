#![feature(type_alias_impl_trait)]

use advent::prelude::*;
use std::cmp;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(HasParser)]
struct ChangeDir {
    #[parse(before = "$ cd ", after = "\n")]
    path: CommandPath,
}

#[derive(HasParser)]
struct ListDir {
    #[parse(before = "$ ls\n")]
    entries: List<Entry, TermWith<NewLine>>,
}

struct CommandPath(PathBuf);

impl HasParser for CommandPath {
    #[into_parser]
    fn parser() -> _ {
        many1(alpha_num().or(char('/')).or(char('.'))).map(|s: String| Self(PathBuf::from(s)))
    }
}

enum Entry {
    Dir(CommandPath),
    File(u64, CommandPath),
}

impl HasParser for Entry {
    #[into_parser]
    fn parser() -> _ {
        let file = (u64::parser().skip(char(' ')), CommandPath::parser())
            .map(|(size, name)| Self::File(size, name));
        let dir = string("dir ").with(CommandPath::parser()).map(Self::Dir);
        file.or(dir)
    }
}

#[derive(HasParser)]
enum Command {
    Cd(ChangeDir),
    Ls(ListDir),
}

#[derive(Default, Debug)]
struct Node {
    space_used: u64,
}

struct Fs(BTreeMap<PathBuf, Node>);

impl Fs {
    fn build(commands: List<Command, Nil>) -> Self {
        let mut fs = Self(BTreeMap::new());
        let mut cwd = PathBuf::from("/");

        for cmd in commands {
            match cmd {
                Command::Cd(c) => {
                    if c.path.0 == Path::new("..") {
                        cwd.pop();
                    } else {
                        cwd.push(c.path.0);
                    }
                }
                Command::Ls(l) => {
                    for e in l.entries {
                        if let Entry::File(size, _) = e {
                            fs.add_size_to_dir(cwd.clone(), size);
                        }
                    }
                }
            }
        }

        fs
    }

    fn add_size_to_dir(&mut self, mut path: PathBuf, size: u64) {
        loop {
            let node = self.0.entry(path.to_owned()).or_insert(Node::default());
            node.space_used += size;

            if &path == Path::new("/") {
                break;
            }

            path.pop();
        }
    }

    fn dirs(&self) -> impl Iterator<Item = &Node> {
        self.0.values()
    }

    fn get_root(&self) -> &Node {
        self.0.get(Path::new("/")).unwrap()
    }
}

#[part_one]
fn part_one(commands: List<Command, Nil>) -> u64 {
    let fs = Fs::build(commands);

    let mut total_size = 0;
    for n in fs.dirs() {
        if n.space_used <= 100000 {
            total_size += n.space_used;
        }
    }

    total_size
}

#[part_two]
fn part_two(commands: List<Command, Nil>) -> u64 {
    let fs = Fs::build(commands);
    let used_space = fs.get_root().space_used;
    let free_space = 70_000_000 - used_space;
    assert!(free_space < 30_000_000, "{free_space}");

    let mut smallest_dir_to_delete = u64::MAX;
    for n in fs.dirs() {
        if n.space_used + free_space >= 30_000_000 {
            smallest_dir_to_delete = cmp::min(n.space_used, smallest_dir_to_delete);
        }
    }

    smallest_dir_to_delete
}

harness!(part_1: 1491614, part_2: 6400111);
