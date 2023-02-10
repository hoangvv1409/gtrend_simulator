# gtrend_simulator

### Install Rust

Using command below or follow this link for more detail <https://www.rust-lang.org/tools/install>

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build and run

Build for dev

```bash
cargo build
```

Build for release

```bash
cargo build --target wasm32-unknown-unknown --release
```

Run the simulator (this version only support file path within this project)

```bash
cargo run <put your file path here>
```

Run the simulator with exists file in the project

```bash
cargo run ./sample_google_trends_data.json
```

Run the simulator and write the output to stdout

```bash
cargo run ./sample_google_trends_data.json > output
```

### Some thought about this task

#### How much time I spent on this task?

- 2 hours of reading, define and clarify the task's description. I also examine the sample file using python to see if there is any error
- 3 hours of implementing

#### What are the limitations of your code?

- Have no time to research and implement protobuf bonus task
- If the sample file gets large, this project will get slow down on the read file step, it might not run too
- The 'run' function in lib.rs can be split into smaller functions to reduce the indent
- Need to write unit test before implement
- The 'read_file' function in lib.rs should handle more file formats (for example file with line-by-line format)

#### What are the limitations of the source data? Are there any edge cases you don't handle?

- The structure of source data should be line-by-line for each timeframe. Why? When the file gets larger and larger, I can improve the code to read the file line-by-line rather than read the whole file. This could improve performance and use much smaller memory, I could save the latest line then continue from it if there are new lines appended to the file
- I don't handle the negative integer yet

#### What do you think would happen to the result if the sample data is infinitely long?

Check out the listing above

#### Are there any improvements you can make?

- Use line-by-line structure for the source data, with each line represent a time frame
- Read file line-by-line
- Re-organize the 'run' function to support read line-by-line way
- Provide more option for data source reader (from http rest, graphql, ...) using Trait
- Add unit tests
