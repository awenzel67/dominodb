# dominodb

Like any other database dominodb can be used to query a datastructure to get an answer data structure as result. What make it different is that we choosed:
* A single JSON document as its data structure.
* JavaScript as the query language.
* JSON Schema to describe the JSON document.
Not a good choice if a human writes the query but a rather good choice if an actual llm creates the query based on user input in natural language.

## Geting started

### Prerequsites

Install Rust for your operating system ([see here](https://rust-lang.org/tools/install/)).

For installation of git see [here](https://git-scm.com/install/windows)

### Installation


Clone the repository:

git clone 

Change working directory:
cd dominodb

Build the project:
cargo build

Run the cli interface:
cargo run --database test.json --schema test_schema.json "data.employess.length"









