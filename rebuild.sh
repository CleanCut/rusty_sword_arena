#!/usr/bin/env bash

WATCHFILE="/tmp/rusty_sword_arena.rebuild"

# For when things go horribly wrong
function die {
    echo
    echo $1
    echo
    exit 2
}

while true ; do
    if [ -e ${WATCHFILE} ] ; then
        # Cleanup watchfile
        rm ${WATCHFILE}
        # Pull new source code
        git pull origin master || die "Error pulling from GitHub"
        # Build the docs
        rm -rf target/doc || die "Failed cleaning previous docs"
        cargo doc --lib --no-deps || die "Failed generating documentation"
        # Build new server binary
        cargo build --release --bin server || die "Failed building the server"
        # Restart server
        systemctl restart rusty_sword_arena || die "Failed restarting the server"
        echo "Started Rusty Sword Arena server version $(grep version Cargo.toml | cut -d '"' -f 2)"
    fi
    sleep 1;
done

