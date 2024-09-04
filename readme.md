Builtins I need:

# Array Related

 - [x] Array Wrap
 - [x] Array Unwrap
 - [ ] Zip
 - [ ] Sum
 - [ ] Index
 - [ ] Range
 - [ ] Reshape
 - [ ] Transpose

# Generic
 - [x] Comma - Yields the values from both it's arguments seperately

# Generator Related
 - [ ] Reduce - Runs on every pair and outputs a single value
 - [ ] Scan - Like reduce but outputs every intmediate value
 - [ ] First
 - [ ] Last
 - [ ] Until - Outputs the first value where the predicate is False
 - [ ] While - Outputs the last value where the predicate is True
 - [ ] Is empty? Returns true if there is at least one item

 # Math
 
 - [x] addition (on arrays concats)
 - [x] subtraction  (on arrays removes)
 - [ ] multiplication (on arrays join)
 - [ ] division (on arrays split)
 - [ ] floor

 # String Related

 - [ ] To Number
 - [ ] To String
 - [ ] To JSON
 - [ ] To Upper
 - [ ] To Lower
 - [ ] Space Join

# Constants
- [x] 0-9
- [ ] String Literals
- [ ] Empty
- [ ] Empty Array

# Control Flow
- [x] If/Then/Else - Needs special casing to handle multiple output values
- [ ] If/Change - Applies a transformation if condition, else return the value unchanged
- [ ] Select - If a condituion is true, return a value unchanged. Else return None

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