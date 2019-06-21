#!/usr/bin/env bash

WATCHFILE="/tmp/rusty_sword_arena.rebuild"

# For when things go horribly wrong
function die {
    echo
    echo "FATAL: $1"
    echo
    exit 2
}

function banner {
    echo "------------------------------------------------------------------------"
    echo "Waiting for new watchfile to appear to indicate it is time to rebuild..."
    echo "------------------------------------------------------------------------"
}

banner

while true ; do
    if [ -e ${WATCHFILE} ] ; then
        # Cleanup watchfile
        rm ${WATCHFILE}
        # Pull new source code
        git pull || die "Error pulling from GitHub"
        # Build the docs
        rm -rf target/doc || die "Failed cleaning previous docs"
        cargo doc --lib --no-deps || die "Failed generating documentation"
        # Build new server binary
        cargo build --release --bin server || die "Failed building the server"
        # Restart server
        systemctl daemon-reload || die "Failed to reload systemctl daemons"
        systemctl restart rusty_sword_arena || die "Failed restarting the server"
        echo "Started Rusty Sword Arena server version $(grep version Cargo.toml | cut -d '"' -f 2)"
        banner
        exec ./rebuild.sh
    fi
    sleep 1;
done

