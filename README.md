# Drosmokers API
[Drosmokers](https://github.com/cyobero/drosmokers) is an app that allows users to keep track of the different strains they've smoked.

## Install
You'll first need to download and install Rust's package manager [cargo](https://crates.io/) if you don't already have it. 

Once you have that installed, clone this repo and change into the directory
`git clone https://github.com/cyobero/drosmokers-api && cd drosmokers-api`

Then, run:
`cargo run`

Example request: 
  `$ curl localhost:8008/strains/?id=12`

Example response: 
  `$ {"200":[{"id":12, "name":"Headbang", "species":"Hybrid"}]}`

