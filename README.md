# JSON-to-table

A Rust CLI to print an ASCII encoded table out of a JSON specification.
As input we are able to read from stdin or from a file.

To do it it forwards all the real work to [prettytable-rs](https://github.com/phsym/prettytable-rs)
and of course, serde_json.

Originally it was intended to be piped to some kind of Jira query, which would 
return a JSON table, possibly after being filtered with jq.
