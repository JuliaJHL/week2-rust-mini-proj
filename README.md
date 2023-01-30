# A command line application in Rust
* This is a tool where we can give a string and a path, and it will print only the lines containing the given string.
* I distributed it with cargo in this [link](https://crates.io/crates/grrs_hj).
* We are able to run the tool like this:
```
$ cat test.txt
apple 1
bee 2
cat 3
dog 4
apple 5
$ grrs_hj apple test.txt
apple 1
apple 5
```

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
cargo run -- xxx(pattern) xxx(path)
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
![run_img](https://github.com/JuliaJHL/imgs_readme/blob/main/run.png)

## Distributing with cargo
I made package info updates in `Cargo.toml` and pushlished it via:
```
cargo login
cargo publish
```
Therefore, you can install `grrs_hj`  and run it without cloning the repo by implementing the following commands directly:
```
cargo install grrs_hj
grrs_hj xxx(pattern) xxx(path)
```
Here is an example:
![grrs_hj](https://github.com/JuliaJHL/imgs_readme/blob/main/grrs_hj_new.png)


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
