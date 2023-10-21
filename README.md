# Python script for data processing in Rust, highlighting the improvements in speed and resource usage

### [Rust Documentation](https://www.rust-lang.org/)

## Overview

The repo focuses towards comparing improvements in speed and resource usage for using rust against python.

## Code Description

   The output of the visualization code is  :

<p align="center">
  <img width="650" src="https://github.com/nogibjj/IDS-Week3_MiniProject_us26/blob/main/output_graph/visualization.png" alt="My Image1">
</p>

### Action include the general CI/CD process in yml file, which automatically generate the graph and markdown

<p align="center">
  <img width="650" src="https://github.com/nogibjj/IDS-Week3_MiniProject_us26/blob/main/Image/yml_actions.png" alt="My Image2">
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
