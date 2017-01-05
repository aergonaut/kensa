# kensa

Kensa is a service intended to analyze GitHub Pull Requests and report which of
a set of user-configured "features" match the PR. Features describe
characteristics of the PR such as LOC of patch, paths of changed files,
etc., but could also delegate to other external systems to perform more
intricate examination of the code, such as actually building artifacts and
running them.

## License

Licensed under either of

*   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
    <http://www.apache.org/licenses/LICENSE-2.0>)
*   MIT license ([LICENSE-MIT](LICENSE-MIT) or
    <http://opensource.org/licenses/MIT>)

at your option.
