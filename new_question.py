#!/usr/bin/env python3

import argparse
import os
import requests
import rtoml
import sys

from word2number import w2n

CARGO_TOML = '''\
[package]
name = "<name>"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
advent = { path = "../advent" }
combine = "*"
'''

MAIN_RS = '''\
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;

#[part_one]
fn part_one(_: String) -> &'static str {
    "incomplete"
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();

'''

YEAR = 2021

def download_input(name: str, day: int):
    with open(os.path.join(os.getenv('HOME'), '.config/aocd/token')) as f:
        session_key = f.read().strip()

    headers = {'cookie': f'session={session_key}'}
    r = requests.get(
        f'https://adventofcode.com/{YEAR}/day/{day}/input', headers=headers)

    with open(os.path.join(name, 'input.txt'), 'w') as f:
        f.write(r.text)

def add_to_workspace(name: str):
    with open('Cargo.toml', 'a+') as f:
        f.seek(0)
        t = rtoml.load(f)

        members = set(t['workspace']['members'])
        members.add(name)
        members = list(members)
        members.sort()
        t['workspace']['members'] = members

        f.truncate(0)
        f.write(rtoml.dumps(t, pretty=True))

def add_new_question(name: str, day: int) -> int:
    if os.path.exists(name):
        print(f"ERROR: {name} already exists")
        return 1

    os.mkdir(name)
    os.mkdir(os.path.join(name, 'src'))

    with open(os.path.join(name, 'Cargo.toml'), 'w') as f:
        f.write(CARGO_TOML.replace('<name>', name))

    with open(os.path.join(name, 'src/main.rs'), 'w') as f:
        f.write(MAIN_RS)

    add_to_workspace(name)
    download_input(name, day)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("name")
    parser.add_argument("--day", type=int, required=False)
    args = parser.parse_args()
    return add_new_question(args.name, args.day or w2n.word_to_num(args.name))


if __name__ == "__main__":
    sys.exit(main())
