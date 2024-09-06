Builtins I need:

# Array Related

 - [x] Array Wrap
 - [x] Array Unwrap
 - [x] Zip
 - [x] Sum
 - [x] Index
 - [x] Range
 - [ ] Reshape
 - [ ] Transpose
 - [x] Reverse
 - [x] Length

# Generic
 - [x] Comma - Yields the values from both it's arguments seperately

# Generator Related
 - [x] Reduce - Runs on every pair and outputs a single value
 - [x] Knot - Repeat but produce every intermediate value
 - [ ] First
 - [ ] Last
 - [x] Index - Finds the nth element of a list or generator
 - [ ] Until - Outputs the first value where the predicate is False
 - [ ] While - Outputs the last value where the predicate is True
 - [ ] Is empty? Returns true if there is at least one item
 - [x] Count

 # Math
 
 - [x] addition (on arrays concats)
 - [x] subtraction  (on arrays removes)
 - [x] multiplication (on arrays join)
 - [x] division (on arrays split)
 - [ ] modulo
 - [x] floor/flatten

 # String Related

 - [ ] To Number
 - [ ] To String
 - [ ] To JSON
 - [ ] To Upper
 - [ ] To Lower
 - [ ] Space Join

# Constants
- [x] 0-9
- [x] String Literals
- [ ] Empty
- [ ] Empty Array

# Control Flow
- [x] If/Then/Else - Needs special casing to handle multiple output values
- [x] Modify - Applies a transformation if condition, else return the value unchanged
- [x] Multiple - If a condituion is true, return a value unchanged. Else return None

# Stack Related
- [x] Dup
- [x] Swap
- [x] Over - Copies the second value from the stack
- [ ] Store - Push a value to the secondary stack
- [ ] Peek - Copies the top of the secondary stack
- [ ] Unstore - Pops the value of the secondary stack

# Thoughts on Special Cases
"," should be repeatable to take any number of arguments
"?" should be able to output any number of arguments
"!" should not pop arguments from the stack
"branch" should take any number of arguments.