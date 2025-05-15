# Design Project M11: High Performance Data Processing using Rust

Rosen company needs to deal with huge sets of sensor data stored in hdf5 data. This data needs to be processed by several algorithms that are configured with specific parameters. The decision about which algorithms, in which order and with which parameters are taken by their experienced experts. 

To achieve sufficient performance and solve these big data problems, they implement these algorithms in C++. They used a “pipes and filters” design pattern to “chain” these algorithms at runtime. Data is fed into this “chain” as a stream, to minimize file operations. The output consists of a modified data file and some result files. For this project, they want to experiment with an implementation of this framework in Rust. 

Rust is a modern and relatively new programming language which has gained popularity in the last couple of years. It features clean, modern syntax, a very helpful compiler and a rich ecosystem of third-party libraries. With its first-class support of parallelism, concurrency and a built-in async programming model, we believe it is a good choice for implementing a framework for Rosen’s signal and image processing and AI algorithm data pipeline.

[TOC]

## Installation
 - Clone the Git Repository to your local machine
 - Install Docker [Windows Guide](https://docs.docker.com/desktop/install/windows-install/), [Mac Guide](https://docs.docker.com/desktop/install/mac-install/), [Ubuntu Guide](https://docs.docker.com/engine/install/ubuntu/). Verify that Docker is installed by running the command `docker --version`
 - Login to the UTwente GitLab Registry by running the command `docker login registry.gitlab.utwente.nl` (you will have to use an [Access Token](https://gitlab.utwente.nl/-/profile/personal_access_tokens))
 - Install Python 3 [Download Page](https://www.python.org/downloads/). Verify that Python 3 is installed by running the command `python3 --version`. Note that other versions of Python may also work.

## Running the Docker Dev Container
 - Run the Docker Dev Container by running the Python start script `python3 rust-hdf5-env/start.py` 
 - Now in the Dev Container, Cargo can be used to:
 <br>1. Build the project `cargo build`
 <br>2. Run the project `cargo run`
 <br>3. Test the project `cargo test`

 ## Building documentation using Cargo
 - Run the command `cargo rustdoc --target-dir ./rust-doc --open`
