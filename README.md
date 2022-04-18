# sha_checker_example

Running the program often requires administrator rights on Windows. Requesting administrator rights can be embed into the PE `.exe` file.

## Examples of usage

### Relative path

Run the executable as usual

```
cargo run
```

It will type the instructions in the output

```
Current working directory is C:\Users\demen\Documents\GitHub\sha_checker_example
Please enter the path to the file whose hash you want to find out:
```

Follow the instructions by entering either an absolute or a relative file

```
./Cargo.toml
```

or

```
Cargo.toml
```

And you will see the result

```
Hash of "./Cargo.toml" is d6bffe915893827e89570151b9e3bd76f274c92c32653d60164c8162fc9fc0ea
```

### Absolute path

```
Current working directory is C:\Users\demen\Documents\GitHub\sha_checker_example
Please enter the path to the file whose hash you want to find out:
C:\\DumpStack.log
Hash of "C:\\DumpStack.log" is b1f93ac5ccc5cc51333d5d672275d1a7ecf7ee7c030b5561448f3bc9897d6edf
```
