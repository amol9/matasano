=================================================
Rust solutions for the matasano crypto challenges
=================================================

Challenges: http://cryptopals.com


Progress
========

* Set 1: complete (01-08)
* Set 2: complete (09-16)
* Set 3: 17-20


Platform
========

* Rust 1.6.0
* Linux / Windows


Usage
=====

* Build::

        cargo build

* Test all challenges for cryptopals cases::

        cargo test test_cryptopals

* Run individual challenges interactively: >cargo run [challenge_no], e.g.::

        cargo run 15

* List all solved challenges::

        cargo run list


Note
====

Some challenges require downloading data files from cryptopals. The tests expect these files to be present in directory named "data" at the project root.

