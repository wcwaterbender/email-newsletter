key things so far:

use lld for faster linking

use cargo wach to monitor source code to trigger commands everytime a file changes

use cargo tarpaulin for code coverage testing

rustfmt for formatting crates, files, etc

turn on format on save

make format and linting checks part of CI

use cargo audit to check for cves on crates used in the project

use cargo add to quickly add deps to toml

use cargo expand to expand macros

rustup toolchain install nightly --allow-downgrade 
 - allows nightly for components that need it

use reqwest for integration tests against public apis
 - choose random ports when testing
 - handle binding with a tcp listener
 


