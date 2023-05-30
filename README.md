# rping - Simple Network Scanner written in Rust

This is a simple (and blazingly fast) network scanner written in Rust that scans IP addresses by sending ping requests. It allows you to discover active hosts on a network by leveraging the ICMP protocol.

## Features

- Scans a range of IP addresses and checks for active hosts.
- Utilizes ICMP echo requests (ping) to determine if an IP address is active.
- Displays the results in a user-friendly format.

## Requirements

To compile and run this network scanner, you need to have Rust installed on your system. You can download and install Rust from the official website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Installation

1. Clone this repository to your local machine:

   ```shell
   git clone https://github.com/schneipp/rping.git; cd rping;
   
2. compile it 
      ```shell
   cargo build -r
   
3. copy the binary to a bin directory
      ```shell
   sudo cp target/release/rping /usr/local/bin

Keep in mind that you might need root privileges to run it!


## Usage
```shell
rping --help
Usage: rping [OPTIONS]

Options:
  -s, --startip <STARTIP>  [default: 192.168.0.1]
  -e, --endip <ENDIP>      [default: 192.168.0.254]
  -h, --help               Print help
  -V, --version            Print version
