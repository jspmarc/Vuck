# Vuck

1. 1D, like brainfuck
1. The stack's elements are of `signed int` size (4 byte)
1. The stack's initially empty. Trying to access the stack/do operations on it will cause the
   interpreter to error.
1. Pointer can go sideways only
   - `l` to move to the right
   - `h` to move to the left
1. We can pop and push values to the stack
   - `k<number>` to push `<number>` (moving the stack pointer up), note the `.`
   - `j` to pop (moving the stack pointer down)
1. **End of program is denoted by `:q`**
1. Mathematical operation to the top of stack value is done by:

   - `+` add the first and the second stack value
   - `-` subtract the first by the second stack value (e.g. `first - second`)
   - `*` multiply the first by the second stack value
   - `/` divide the first by the second stack value (e.g. `first / second`)
   - `%` modulo the first with the second stack value (e.g. `first % second`)

   > - Result of the mathematical operation is pushed onto the stack, then the operands of
   >   the stack will be popped from the stack
   > - If the number of operands in the stack is not enough, the interpreter will error out

1. The program can **read a number** from the user and then push it to the stack
   with `i`
1. The program can **read an ASCII** and then store it as its integer representation
   from the user and then push it to the stack with `I`
1. The program can output a number as an **unsigned integera** with `p`
1. The program can output a number as its **ASCII value** with `P`

   Only the **first byte** will be taken out of the 4 byte to print as ASCII.

1. Loops:

   1. Mark the start of a loop with:
      ```
      ,
      ```
   2. The end of loop is:
      ```
      F
      ```
      - The loop will end if the top of the stack is 0

1. Conditionals:
   1. Mark the start of the conditional with:
      ```
      .
      ```
   1. Mark the end of the conditional with:
      ```
      T
      ```
   1. Mark the else branch with:
      ```
      |
      ```

## Used characters

| Character    | Usage |
| ------------ | ----- |
| `h`          |       |
| `j`          |       |
| `k<number>.` |       |
| `l`          |       |
| `:q`         |       |
| `+`          |       |
| `-`          |       |
| `*`          |       |
| `/`          |       |
| `%`          |       |
| `i`          |       |
| `I`          |       |
| `p`          |       |
| `P`          |       |
| `,<mark>`    |       |
| `F<mark>`    |       |
