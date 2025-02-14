#!/bin/sh

### How to publish the framework
#
# Prerequisites:
# - You need rights to publish on crates.io
# - You need an API access token (you obtain it from https://crates.io/me)
# - You need to call `cargo login <token>` in a console, follow the instructions on crates.io for this.
#
# Steps:
#
# 1. Have a look at commits on GitHub, everything that changed since the last release must be published.
# Be mindful that hotfixes need to be backwards compatible, minor releases do not.
# We always publish all `dharitri-wasm-*` crates together.
# We always publish `dharitri-codec` and `dharitri-codec-derive` together.
# `dharitri-wasm-*` depend on both `dharitri-codec` and `denali`, so if you have a minor release on the latter,
# you also need a minor release on `dharitri-wasm-*`.
#
# 2. Mass replace previous version -> new version (dharitri-wasm, dharitri-codec, denali - different numbers).
# Be careful to not accidentally replace some of the other dependencies we have.
#
# 3. Write release name, date and description in `CHANGELOG.md`.
#
# 4. Run `cargo test`, to make sure nothing was broken and all dependencies still work fine.
#
# 5. Commit changes. The name of the commit should be the released crates and versions, same as the changelog title,
# e.g. `dharitri-wasm 0.21.1, dharitri-codec 0.8.1, denali 0.11.1`.
# The branch doesn't need to be published for the following steps to work.
# 
# 5. Run this script, `./publish.sh`.
# You can comment out the crates you are not publishing. The script will stop otherwise when it cannot publish them.
# 
# 6. Search for `dharitri` on `crates.io` and check that the new versions appear for all crates.
# If any of the crates was not published, check what went wrong and try again.
#
# 7. Create tag.
# `git tag -s -a vX.X.X -m 'very short description of the release'`
# `git push origin vX.X.X`
#
# 8. Go to https://github.com/DharitriNetwork/dharitri-wasm-rs/tags
# Click on the new tag.
# Click `Create release from tag`.
# The title should be the released crates and versions, same as in the changelog and the commit message.
# The description should be copied from CHANGELOG.md, as is.
#
# 9. Create pull request on GitHub. The faster it gets merged in master, the better.
#
# 10. (optional) Test the new framework on one of the contracts that are not in the same repo, e.g. DNS, DEX, etc.
#
# 11. (optional) Announce on Telegram.
# Skip this step if you feel the new release is a bit too experimental, or if it doesn't work with the latest VM.
#

cd dharitri-codec-derive
cargo publish || return 1
cd ..

cd dharitri-codec
cargo publish || return 1
cd ..

cd dharitri-wasm-derive
cargo publish || return 1
cd ..

cd dharitri-wasm
cargo publish || return 1
cd ..

cd denali
cargo publish || return 1
cd ..

cd dharitri-wasm-node
cargo publish || return 1
cd ..

cd dharitri-wasm-debug
cargo publish || return 1
cd ..

cd dharitri-wasm-output
cargo publish || return 1
cd ..

cd dharitri-wasm-modules
cargo publish || return 1
cd ..

cd contracts/core/price-aggregator
cargo publish || return 1
cd ../../..

cd contracts/core/wmoa-swap
cargo publish || return 1
cd ../../..

cd dharitri-interact-snippets
cargo publish || return 1
cd ..
