# MHDL

Modern HDL that transcopile to existing HDLs

## What it tries to achieve?

MHDL tries to implement a more syntactic and readable language for programming FPGAs.
While Verilog has very less structure, VHDL is way too expressive.
MHDL has a very precise and parseable syntax. The language is also more strict and specific about stuff that is for simulations.

## Why transcompiling?

Writing a own synthesizer and simulator is way to much effort.

## Basic concepts

### Seperate simulation and production code

VHDL and Verilog were not designed for coding, therefore those languages mixes testing and production code.
MHDL tries to solve this, by seperating this code with like similar to SW targeting languages like rust and zig.

### Readable syntax

While VHDL has waaaay too much unnessecary code, verilog tends to be way to unstructured.
MHDL tries to find a mix between those and a bit of modern C like syntax. This will make programming FPGAs very much simpler for new programmers.

### Strict syntax

While less strict syntax can be good for fast coding it is not something you really want int a FPGA project. Unconsise syntax could lead easily into hard to solve problems.
This is the reason, why MHDL has strict

### Fast transpiling

One of the main reasons for transpiling to verilog is its slightly faster syntesis and simulations. The transpiler is written in rust and therefore very fast.

### Rich Error

While most verilog/VHDL sythesisers and simulators tend to give bad errors. MHDL tries to give readable and easy understandable errors.
The final goal is to produce always either produce working verlog code or throw a transpiling error. This sadly won't be the case at this early stage of developemt, but will get better as development goes on.

### Modular

MHDL will provide easy file based modules. Declarations and instantiactions will be compatible with existing verilog/VHDL code or netlists.
However those will be signficantely more easily to use and less redundant than for example VHDL.

## Future features

Some long term features might be:

- Example projects
- Formatting tools
- Language server for big editors (Nvim, Vscode, etc.)
- Meta programming
- Static code analysis
- Own faster Simulation tools (Very very long term)
- Multiple codegen backends (Very very very long term)

## Disclaimer

This project is very early in development and more of a conecpt for now. DO NOT USE THIS IN PRODUCTION.
However feel free to open issues and make prs.
