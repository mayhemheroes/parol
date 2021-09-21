<!-- markdownlint-disable first-line-h1 -->
[![Build Status](https://travis-ci.org/jsinger67/parol.svg?branch=main)](https://travis-ci.org/jsinger/parol)
[![Docs.rs](https://docs.rs/parol/badge.svg)](https://docs.rs/parol)
[![Crates.io](https://img.shields.io/crates/v/parol.svg)](https://crates.io/crates/parol)
<!-- markdownlint-enable first-line-h1 -->

# About `parol`

`parol` is a LL(k) parser generator **for Rust written in Rust** with the following features

* Generated parsers are **table driven**. They are no bunch of recursive functions as often seen in practice. Instead they are true push down automata (PDAs).
* Generated parsers are predictive, i.e. they implement a **non-backtracking** parsing technique. This often results in much faster parsers.
* Generated parsers use a **flexible lookahead** for each non-terminal; they use only as much lookahead as needed.
* Generated parsers are clean and easy to read. Terminal names are automatically deduced from the grammar description.
* Rule selection is done by one deterministic finite **lookahead automaton** for each non-terminal.
* The lexer and the parser is generated from **one single grammar description** file. Semantic actions are strictly separated from the grammar definition.
* **Semantic actions** are generated for each production as stubs in a special trait with empty default implementations. The user can implement this trait for his grammar processing item and overwrite needed actions. This provides a loose coupling between your language definition and the language processing.
* The grammar description is provided in a **Bison/Yacc-like** style with additional features known from EBNF such as grouping, optional elements and repetitions.
* The grammar description supports definition of language comments via **%line_comment** and **%block_comment** declarations.
* The crate provides several tools for **grammar analysis**, **transformation** and **parse tree visualization** to support your grammar implementation.
* The parser generator **detects direct and indirect left recursions** in your grammar description.

This project contains some introductory grammar examples from entry level up to a more complex C-like expression language and an acceptor for Oberon-0 grammar.
Two of the examples describe the principles of language processing by using semantic actions in the way `parol` advocates it.

Last but not least **`parol`'s parser for its own input language is generated by `parol` itself**. So this application of language processing is an additional and very practical example.

It's worth mentioning that there exists another opportunity to process the parse result.
The parse tree the parser generates can be processed by user created tools too. So no one is tied to `parol`'s approach to semantic actions although this approach is easy and compelling.

## How `parol` works

`parol` first transforms the input grammar into an expanded form where optional expressions, groups and repetitions are substituted by equivalent production sets. Then it analyzes this pre-transformed input grammar for several properties that prevent a successful processing. Those properties are

* Left-recursions
* Non-productive non-terminals
* Unreachable non-terminals

If there are no objections against the input grammar the next step is to left-factor the grammar that was produced by the previous expansion. This step is crucial for decreasing the number of necessary lookahead symbols.

This finally transformed grammar is the basis for the parser generation and can or better should be written to file for later reference. By convention this expanded grammar is stored to files names \<original-name\>-exp.par. Thus it is often useful to use this expanded grammar with any tool, because it is checked and left-factored. Also because this processed grammar is the basis for parser generation, you have to use it in this form in your grammar processing backend.

The actual parser generation then starts witch generating the lookahead automata for the non-terminals. In this phase it determines if the grammar is LL(k) for *k* starting with 1 and increasing it by one until a solution is found or the maximum lookahead size is exceeded. If your grammar is more than LL(5) the needed amount of processing power and memory consumption makes it inefficient to work with. In such a case you should rework your grammar design thoroughly. Or you can use a super fast machine to generate your parser's sources and compile and run the generated parser on an ordinary one. Internally the maximum lookahead size is currently limited to 10 though.

To determine if your grammar is LL(k) `parol` generates equation systems for both FIRST(k) and FOLLOW(k) sets and tries to solve them iteratively until a fix point is reached which indicates the solution. This is the most expensive task for `parol`.

If a solution is found `parol` generates all necessary data to feed the scanner and parser with. Based on this data `parol` then generates two source files.

The first one contains all scanner and parser data. The second one provides two traits. The first of these traits is important for the user's grammar processing. It contains for each production an empty default implementations of the corresponding semantic action. The semantic actions of the user can be provided by implementing this trait and providing own implementations for any production needed. The trait's name can be defined per command line argument.

The second trait in this file provides bindings of semantic actions so that the parser can call them via production number during parse time. It's name is always `UserActionsTrait`.
  
## Dependencies

Parsers generated by `parol` have to add a dependency to the parol_runtime crate which provides the scanner and parser implementations needed. The parol_runtime crate is very lightweight.

## Further readings

* [Introduction to `parol`](docs/Introduction.md)
* [Tutorial](docs/Tutorial.md)
