#!/usr/bin/env bash

WATCHFILE="/tmp/rusty_sword_arena.rebuild"
DOCS_DIR="/web/static/rusty_file_arena"

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
        rm -rf ${DOCS_DIR} || die "Failed cleaning previous docs"
        cargo doc --lib --no-deps --target-dir ${DOCS_DIR} || die "Failed generating documentation"
        # Build new server binary
        cargo build --release --bin server || die "Failed building the server"
        # Restart server
        systemctl restart rusty_sword_arena || die "Failed restarting the server"
    fi
    sleep 1;
done

