# New project
To create a new directory (project) as an executable (binary)
    cargo new project --bin

Same as above but into an existing directory
    cargo init project --bin

Both of the above will create a src directory inside the project directory with a single file, main.rs,
containing a main function - the entry point for a binary program.
This is what `--bin` specifies in the cargo command.

To run the above project, change into the project directory and enter
    cargo run

This will compile the program, run the executable and then delete the executable.
You can just do the compile step to an executable with the command
    cargo build

This creates a target directory with a debug sub directory (the default build type)
with the executable and debug symbols file inside.
The name of executable is the name of the project.
You can change to a release build by appending `--release`.
