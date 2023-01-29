# A command line application in Rust
This is a tool where we can give a string and a path, and it will print only the lines containing the given string. 

## project setup
1. Clone the repo:
```
git clone https://github.com/JuliaJHL/week2-rust-mini-proj.git
```
2. cd into the repo:
```
cd week2-rust-mini-proj
```
3. compile the project:
```
cargo build --release
```
4. run the project:
```
cargo run --xxx(pattern) xxx(path)
```

## examples:
I created a test.txt with the following contents:
```
apple 1
bee 2
cat 3
dog 4
apple 5
```
When we apply `cargo run -- apple test.txt`, it would only return:
```
apple 1
apple 5
```


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
