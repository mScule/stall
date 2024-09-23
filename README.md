# Stall

This is my passtime project where I try to create a working stack machine

## Programs

Stall programs are folders that have `boot.toml` file in the root folder where the main function location is stated.

This folder can contain other folders and files.

`.funcs` files are the ones that contain the actual code in form of low levelish human readable format that is parsed before the program will be run.

See the *example_program* for reference.

## How to run program

To start a program call stall in terminal and give the path to the program folder to it as command line argument like so `./path/to/stall ./path/to/program_folder`

## Structrue

Stall consists of *booter*, *funcs_parser*, and *vm* thats abilities of doing stuff can be extended by creating "apis".

### Booter `boot`

Booter goes through a given folder and runs the funcs parser so the actual functions runnable by the vm is put into a hashmap where it can access them via function names.

### Funcs parser `funcs`

Funcs parser parses strings into actual function that the vm can understand. These are defined in their own syntax. See the [funcs syntax](#funcs-syntax-reference) for further details.

### VM `vm`

This is the actual stack machine that interpretes the bytecode created by the funcs parser.

It has three stacks that is uses, *Scope* stack, *Call* stack, and *Value* stack.

#### Scope stack `scopes`

Scope stack holds variables in vectors. Whenever new scope is pushed onto the stack, theres another vector where you can add variables. When scope ends, it will remove all the variables inside it.

#### Call stack `calls`

Call stack contains function calls. Function call is program counter for that specific function call and reference to the function itself. This way a function can return from where it was after function called inside it is popped from the call stack.

#### Value stack `vals`

Value stack is the "workbench" where all operations are being done and where constants and variables are being loaded. This is accessable by all scopes and function calls.

#### Values in VM

There are primary types and reference types. Primary types are ones that are owned by the variable and reference types are just referenced by it. Reference types are being reference counted. Stall can leak memory since [it is possible to leak memory through circular references](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html?highlight=Weak)

##### Primary types

- None   - Marks the absense of value
- Bool   - Boolean value
- Num    - Number
- String - Value that holds text

##### Reference types

- Vec  - Can hold dynamic amount of values inside itself
- Map  - Holds key value pairs. Key has to be a string values
- Func - Contains operations that are being interpreted by the vm

## Funcs syntax reference

### Defining functions

Funcs files consist of functions. Functions are defined by writing their name that is then followed by the operations inside brackets `{` `}` One `.funcs` file can contain as many functions as wanted.

```
say_hello {
    "Hello world!\n"
    call_sys "io/print"
    return
}
```

Funtions has to have return at some point, otherwise Stall will crash :) Return just means that the function will be popped from the call stack.

### Referencing functions

To refer certain function, you'll need to use the `get_const` operator, or get the function reference from variable. Then you call it with `call_func` operation. The route to desired function will be its path in the program folder from root.

See the example below:

```
| File path where the root is the root is the program folder:
| /libs/math.funcs

| This function presumes that there is two numeric values that share the
| same type pushed into the value stack
add_things {
  add
  return
}
```

If you wanted to call that function, you would use following string:
`/libs/math.funcs/add_things` to get the function pushed to value the stack.

### None value

If you want to mark that there isnt value, you can use `none` literal for it.

### Bool values

If you want to have boolean values, you can push them to stack with `true` and `false` literals.

### Numeric values

Num type uses [fraction package](https://docs.rs/fraction/latest/fraction/). Numbers are stored as fractions, but from the outside they can be handled as ints and floats. You can use them like this: `10` `10_300.34`. (You can add underscores for readiblity)

### Strings

To push string into the value stack, you can just write it like in C like languages. C like escape characters are supported.

### List of operation keywords

Pops column means how many values does the operation pop from the value stack at the start of the operation.
Pushes column means how many values does the operation push to the stack after the operation finishes.

If Pops or Pushes columns have * it means the value can be 0 or more.

| Keyword                             | Description                                                   | Pops | Pushes |
| ----------------------------------- | ------------------------------------------------------------- | ---- | ------ |
| `get_const <const_path>`            | Gets function                                                 | 0    | 1      |
| `new_scope`                         | Pushes new scope to the scope stack                           | 0    | 0      |
| `end_scope`                         | Pops latest scope from the scope stack                        | 0    | 0      |
| `new_var`                           | Pushes new variable to the current scope                      | 1    | 0      |
| `set_var <offset_from_top> <index>` | Sets variable                                                 | 1    | 0      |
| `get_var <offset_from_top> <index>` | Gets variable                                                 | 0    | 1      |
| `call_func`                         | Attempts to call a function                                   | *    | *      |
| `call_sys`                          | Attempts to call a system api                                 | *    | *      |
| `return`                            | Pops function latest call from call stack                     | 0    | 0      |
| `goto`                              | Sets the latest calls pc to given index                       | 0    | 0      |
| `if_true_goto`                      | Does the same as goto if the value in the value stack is true | 1    | 0      |
| `if_false_goto`                     | Does the same as above but in opposite manner                 | 1    | 0      |
| `gte`                               | Greater or equal to                                           | 2    | 1      |
| `lte`                               | Less or equal to                                              | 2    | 1      |
| `gt`                                | Greater than                                                  | 2    | 1      |
| `lt`                                | Less than                                                     | 2    | 1      |
| `eq`                                | Equal to                                                      | 2    | 1      |
| `not`                               | Not                                                           | 1    | 1      |
| `add`                               | Addition                                                      | 2    | 1      |
| `sub`                               | Subdivision                                                   | 2    | 1      |
| `mul`                               | Multiplication                                                | 2    | 1      |
| `div`                               | Division                                                      | 2    | 1      |
| `concat`                            | Concatenates two strings                                      | 2    | 1      |
| `to_num`                            | Changes value to num                                          | 1    | 1      |
| `to_string`                         | Changes to string                                             | 1    | 1      |
| `new_vec`                           | Create new empty vector                                       | 0    | 1      |
| `push_to_vec`                       | Pushes value to vector                                        | 1    | 0      |
| `get_vec_val`                       | Gets value from vector                                        | 3    | 1      |
| `set_vec_val`                       | Sets value to some index in vector                            | 3    | 0      |
| `new_map`                           | Create new empty map                                          | 0    | 1      |
| `get_map_val`                       | Gets value from map with key                                  | 2    | 1      |
| `set_map_val`                       | Sets value to map with key                                    | 3    | 0      |

### Comments

If you want to add comments to the `.funcs` file you can do it with pipes `|`.

If you want to have comment in middle of code you can do it by having two pipes like so `| This is comment |`. If you don't add the second pipe in the same line, the comment will be terminated when tokenizer hits new line character.
