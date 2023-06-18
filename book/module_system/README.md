# Chapter 7

Here's a set of practice exercises based on managing projects with multiple files using modules and the concepts of structs, functions, and enums. These exercises are centered around creating a "Mage Guild" and managing the process of admitting new candidates into the guild.

## Exercise 1

Create a new Rust project called "mage_guild" with the necessary file structure.

```bash
Main file: main.rs
Module files: guild.rs, candidate.rs, tests.rs
```

## Exercise 2

Inside the guild.rs module:

- Define a struct called Member with fields name: String and level: u32.
- Implement a method on the Member struct called new that takes a name and level as parameters and returns a new instance of Member.
- Implement a method on the Member struct called promote that increases the level of the member by 1.

## Exercise 3

Inside the candidate.rs module:

- Define an enum called TestResult with variants Passed and Failed.
- Define a struct called Candidate with fields name: String and test_result: Option<TestResult>.
- Implement a method on the Candidate struct called new that takes a name as a parameter and returns a new instance of Candidate with test_result set to None.
- Implement a method on the Candidate struct called take_test that takes no parameters and randomly assigns a Passed or Failed test result to the candidate.

## Exercise 4

Inside the tests.rs module:

- Create a function called analyze_candidate that takes a Candidate as a parameter and returns a TestResult based on some custom logic (e.g., random chance).
- Use the rand crate to generate random test results.

## Exercise 5

In the main.rs file:

- Import the necessary modules (guild, candidate, and tests) using the mod keyword.
- Create a new instance of Candidate and call the take_test method to assign a test result.
- Use the analyze_candidate function from the tests module to analyze the candidate's test result.
- Print the test result.

## Exercise 6:

Inside the guild.rs module:

- Implement a method on the Member struct called invite_candidate that takes a Candidate as a parameter and returns a new instance of Member with the candidate's name and a default level of 1.
- Use the promote method to increase the level of the new member by 1.

## Exercise 7:

In the main.rs file:

- Call the invite_candidate method from the guild module and pass the analyzed candidate as a parameter.
- Print the details of the newly invited member.
