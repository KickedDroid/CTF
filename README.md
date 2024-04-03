# Pentest Automation with Rust
An extensible, multithreaded tool to gather recon for a Pentest or CTF! 
Quickly test a url against many tools such as:
- Subscout
- Rustscan
- Feroxbuster
- XSStrike (WIP) 
- Nuclei (WIP)
- Wordlist Generator

Todo:
- Obsidian/Markdown output
- SSL Scanning
- whatweb
- SQLi Scanning
- mtr
---
## Build
```cd pentest
   cargo build --release && cp target/release/pentest .
```
```
git clone https://github.com/dom-sec/subscout
cd subscout && cargo build --release && cp /target/release/subscout .
```
## Run 
```
./pentest example.com
```
Example output: 
```
██████╗░███████╗███╗░░██╗████████╗███████╗░██████╗████████╗
██╔══██╗██╔════╝████╗░██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
██████╔╝█████╗░░██╔██╗██║░░░██║░░░█████╗░░╚█████╗░░░░██║░░░
██╔═══╝░██╔══╝░░██║╚████║░░░██║░░░██╔══╝░░░╚═══██╗░░░██║░░░
██║░░░░░███████╗██║░╚███║░░░██║░░░███████╗██████╔╝░░░██║░░░
╚═╝░░░░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░╚══════╝╚═════╝░░░░╚═╝░░░

░█████╗░██╗░░░██╗████████╗░█████╗░███╗░░░███╗░█████╗░████████╗██╗░█████╗░███╗░░██╗  ░██╗░░░░░░░██╗██╗████████╗██╗░░██╗
██╔══██╗██║░░░██║╚══██╔══╝██╔══██╗████╗░████║██╔══██╗╚══██╔══╝██║██╔══██╗████╗░██║  ░██║░░██╗░░██║██║╚══██╔══╝██║░░██║
███████║██║░░░██║░░░██║░░░██║░░██║██╔████╔██║███████║░░░██║░░░██║██║░░██║██╔██╗██║  ░╚██╗████╗██╔╝██║░░░██║░░░███████║
██╔══██║██║░░░██║░░░██║░░░██║░░██║██║╚██╔╝██║██╔══██║░░░██║░░░██║██║░░██║██║╚████║  ░░████╔═████║░██║░░░██║░░░██╔══██║
██║░░██║╚██████╔╝░░░██║░░░╚█████╔╝██║░╚═╝░██║██║░░██║░░░██║░░░██║╚█████╔╝██║░╚███║  ░░╚██╔╝░╚██╔╝░██║░░░██║░░░██║░░██║
╚═╝░░╚═╝░╚═════╝░░░░╚═╝░░░░╚════╝░╚═╝░░░░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝░╚════╝░╚═╝░░╚══╝  ░░░╚═╝░░░╚═╝░░╚═╝░░░╚═╝░░░╚═╝░░╚═╝

██████╗░██╗░░░██╗░██████╗████████╗
██╔══██╗██║░░░██║██╔════╝╚══██╔══╝
██████╔╝██║░░░██║╚█████╗░░░░██║░░░
██╔══██╗██║░░░██║░╚═══██╗░░░██║░░░
██║░░██║╚██████╔╝██████╔╝░░░██║░░░
╚═╝░░╚═╝░╚═════╝░╚═════╝░░░░╚═╝░░░
Performing recon on "example.com"
Running rustscan on example.com
Running Feroxbuster on example.com
```

