# ready... set... boole! 🏁

## An introduction project to boolean algebra and set theory

All the unit test for the project can be run with `cargo test`  
And you can run a little CLI programm to show test output with `cargo run`


### Summary:

<details>
<summary> Introduction </summary>

- [Exercise 00 - Adder](#ex00)
- [Exercise 01 - Multiplier](#ex01)
- [Exercise 02 - Gray code](#ex02)
- [Exercise 03 - Boolean evaluation](#ex03)
- [Exercise 04 - Truth table](#ex04)
</details>

<details>
<summary> Rewrite rules </summary>

- [Exercise 05 - Negation Normal Form](#ex05)
- [Exercise 06 - Conjunctive Normal Form](#ex06)
- [Exercise 07 - SAT](#ex07)
</details>

<details>
<summary> Set theory </summary>

- [Exercise 08 - Powerset](#ex08)
- [Exercise 09 - Set evaluation](#ex09)
</details>

<details>
<summary> Space filling curves </summary>

- [Exercise 10 - Curve](#ex10)
- [Exercise 11 - Inverse function](#ex11)
</details>


----

###  <a name="ex00">Exercise 00 - Adder</a>

Maximum time complexity: **O(1)**  
Maximum space complexity: **O(1)**  

You must write a function that takes as parameters two natural numbers a and b and re
turns one natural number that equals a + b. However the only operations you’re allowed
to use are:  

- & (bitwise AND)  
- | (bitwise OR)  
- ^ (bitwise XOR)  
- << (left shift)  
- \>\> (right shift)  
- = (assignment)  
- ==, !=, <, >, <=, >= (comparison operators)  

The incrementation operator (++ or += 1) is allowed only to increment the index of
a loop and must not be used to compute the result itself.

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.

###  <a name="ex01">Exercise 01 - Multiplier</a>

Maximum time complexity: **O(1)**  
Maximum space complexity: **O(1)**  

The goal is the same as the previous exercise, except the returned natural number equals a * b. The only operations you’re allowed to use are:
- & (bitwise AND)  
- | (bitwise OR)  
- ^ (bitwise XOR)  
- << (left shift)  
- \>\> (right shift)  
- = (assignment)  
- ==, !=, <, >, <=, >= (comparison operators)  

The incrementation operator (++ or += 1) is allowed only to increment the index of
a loop and must not be used to compute the result itself.

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.

###  <a name="ex02">Exercise 02 - Gray code</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **N/A**  

You must write a function that takes an integer n and returns its equivalent in Gray code.

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.

###  <a name="ex03">Exercise 03 - Boolean evaluation</a>

Maximum time complexity: **O(n)**  
Maximum space complexity: **N/A**  

You must write a function that takes as input a string that contains a propositional
formula in reverse polish notation, evaluates this formula, then returns the result.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex04">Exercise 04 - Truth table</a>

Maximum time complexity: **O(2n)**  
Maximum space complexity: **N/A**  

You must write a function that takes as input a string that contains a propositional
formula in reverse polish notation, and writes its truth table on the standard output.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex05">Exercise 05 - Negation Normal Form</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **N/A**  

You must write a function that takes as input a string that contains a propositional
formula in reverse polish notation, and returns an equivalent formula in **Negation Normal Form** (NNF), meaning that every negation operators must be located right after a
variable.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex06">Exercise 06 - Conjunctive Normal Form</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **N/A**  

You must write a function that takes as input a string that contains a propositional
formula in reverse polish notation, and returns an equivalent formula in **Conjunctive
Normal Form** (CNF). This means that in the output, every negation must be located
right after a variable and every conjunction must be located at the end of the formula.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex07">Exercise 07 - SAT</a>

Maximum time complexity: **O(2n)**  
Maximum space complexity: **N/A**  

You must write a function that takes as input a string that contains a propositional
formula in reverse polish notation and tells whether it is satisfiable.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex08">Exercise 08 - Powerset</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **O(2n)**  

You must write a function that takes as input a set of integers, and returns its powerset.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex09">Exercise 09 - Set evaluation</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **N/A**  

You must write a function that takes as input a string that contains a propositional
formula in reverse polish notation, and a list of sets (each containing numbers), then
evaluates this list and returns the resulting set.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  

###  <a name="ex10">Exercise 10 - Curve</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **N/A**  

You must write a function (the inverse of a space-filling curve, used to encode spatial
data into a line) that takes a pair of coordinates in two dimensions and assigns a unique
value in the closed interval [0; 1] ∈ R.  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  


###  <a name="ex11">Exercise 11 - Inverse function</a>

Maximum time complexity: **N/A**  
Maximum space complexity: **N/A**  

You must write the inverse function f
−1 of the function f from the previous exercise (so
this time, this is a space-filling curve, used to decode data from a line into a space).  

You must also turn in a main function in order to test your function, ready to be
compiled (if necessary) and run.  
