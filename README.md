# exercises

## 0. Introduction

You can run the tests either in vscode by clicking Run / Run Tests inline in the rs files. Alternatively you may use `cargo run` / `cargo test` to execute the assignments.

All exercises require you to create an implementation for them.
Some require you to add arguments to the function in order to match the tests.
Look for `todo!` statements or try to run the unit tests to see what is expected.

## 1. Variable

Example with one variable. You have to add another variable with a different type.
Try to modify the variable after declaration and print the new value

## 2. Primitive types

Introduction to slices, arrays and tuples. Try and continue with the knowledge from the previous exercise.

## 3. Strings

Introduction of strings, managed in heap and immutable references somewhere in memory.
You have to implement the string operations in the pre-defined functions.

## 4. Functions

Continues function syntax and implicit return line.
This exercise is split in 4 parts to help you run tests before all code is valid rust syntax. Note that tests have to be enabled to validate your implementation!

## 5. Can it go smaller
The Cargo exercise, can you trim a rust binary to a smaller size? Add compiler flags to strip information you do not need to run the binary.
In the solution of this exercise we have a creative work-around to trim it down, can you approach that?

## 6. Library
  
Larger exercise to put all learnings into action. 

  - Library implementation to manage books
  - Use rustdoc to view documentation on the code
  - Run clippy to check your code (`cargo clippy`)
  - Create a test that verifies the oldest book in the library