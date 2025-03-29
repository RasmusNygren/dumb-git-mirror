# dumb-git-mirror
Very simple tool to mirror one git repository to another.
It purposefully does not do full mirrors and only 
mirrors the primary branch and tags.

## Installation
`cargo install --git https://github.com/RasmusNygren/dumb-git-mirror.git`

## Usage
If installed:
`dumb-git-mirror --filename mirrors.yaml`


## Config format
```yaml
mirrors:
  - from: "https://github.com/example/from.git"
    to: "https://github.com/example/to.git"
  - from: "https://github.com/example/from.git"
    to: "https://github.com/example/to.git"
```
