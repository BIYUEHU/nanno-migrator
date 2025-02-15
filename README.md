# Nanno Migrator

Hi, there! This is the Nanno Migrator project, which is a tool for migrating data from other visual-novel managers to [Galkeeper(Nanno)](https://github.com/biyuehu/gal-keeper). Whole project is written in Rust.

## Supports list

- **[WhiteCloud](https://whitecloud.xyyaya.com/)**: Base on sqlite and SeaORM
- And more...

Welcome to send issues to me if you want to add more supports or have any questions and suggestions.

## Usage

1. Download the latest release from [releases page](https://github.com/biyuehu/nanno-migrator/releases) or build from source.
2. Put executable file to same directory of `db.3.sqlite` file (Generally, it's in `whitecloud/resources/data/db.3.sqlite`).
3. Run executable file, then you will see a `export-from-whitecloud-xxxx.json` file in the same directory.
4. Import the `export-from-whitecloud-xxxx.json` file to Nanno.
