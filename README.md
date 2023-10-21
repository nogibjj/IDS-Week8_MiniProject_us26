# Python script for data processing in Rust, highlighting the improvements in speed and resource usage

[![Runtime CI/CD](https://github.com/nogibjj/IDS-Week8_MiniProject_us26/actions/workflows/main.yml/badge.svg)](https://github.com/nogibjj/IDS-Week8_MiniProject_us26/actions/workflows/main.yml)

### [Rust Documentation](https://www.rust-lang.org/)

## Overview

The repo focuses towards comparing improvements in speed and resource usage for using rust against python.

## Code Description

I used criterion for Rust and pytest-benchmark for Python for benchmarking. ***Rust took 40ms and Python took 767ms to execute the same code. Rust is **19 times faster** to Calculate sum of squares for the first 10 million natural numbers as comapred to python.***

To execute **rust** file :
1. Install Rust (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)
2. Install the rust-analyzer extension 
3. A good way to create Rust program is to use Cargo to scaffold a new project by typing cargo new. This will create a simple Hello World program along with a default Cargo.toml dependency file. You pass cargo new the folder where you'd like to create the project.
    - cargo new rust_time
4. Cargo new creates a project with a main.rs source code file and Cargo.toml Cargo manifest file.
5. main.rs has the program's entry function main() and prints "Hello, world!" to the console using println!.
6. The output :

<p align="center">
  <img width="650" src="https://github.com/nogibjj/IDS-Week8_MiniProject_us26/blob/main/Image/rusttime.png" alt="My Image2">
</p>

To execute **python** file :
1. Install pytest-benchmark
2. run the command pytest python.py
3. The output : 

<p align="center">
  <img width="650" src="https://github.com/nogibjj/IDS-Week8_MiniProject_us26/blob/main/Image/pythontime.png" alt="My Image2">
</p>


## CI/CD Automation files

1. requirements.txt - Contains all the required python packages
2. Makfefile - Using make to automate different parts of developing a Python project, like -
   
       1. running tests
       2. cleaning builds
       3. installing dependencies
   
   Integrating it into my routine, so can save time and avoid errors.
   
5. .github/workflows - This directory in a Python project (or any GitHub repository) is used for creating and storing GitHub Actions workflows. GitHub Actions is a continuous integration and continuous delivery                           (CI/CD) platform provided by GitHub. The workflow is triggered on pushes to the main branch. It sets up :
   
       1. Python environment
       2. Installs project dependencies
       3. Install packages
       4. Linitng
       5. Runs tests
       6. Format
