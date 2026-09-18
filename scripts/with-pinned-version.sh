#!/usr/bin/env bash
#
# Runs a command with the workspace version pinned, then restores it.
#
# Cargo derives rustc's `-C metadata` from each package's version, and that
# value seeds every symbol hash. Fat LTO merges the upstream modules in an
# order taken from those names, so bumping the workspace version reshuffles
# functions in the built wasms, and so in tests-expanded, without any code
# having changed. Pinning the version for these builds makes their output
# depend on the code alone.
#
# Only the workspace's own crates are pinned. Registry dependencies keep their
# real versions, so a dependency bump still changes the output, as it should.
#
# Cargo.toml and Cargo.lock are restored on exit, including when the command
# fails, and the backups live under target/ so that a hard kill cannot leave an
# untracked file in the working tree.

set -euo pipefail

if [ "$#" -eq 0 ]; then
	echo "usage: $0 <command> [args...]" >&2
	exit 2
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pinned_version="${PINNED_VERSION:-0.0.0}"
backup="$root/target/pinned-version"

mkdir -p "$backup"
cp "$root/Cargo.toml" "$backup/Cargo.toml"
cp "$root/Cargo.lock" "$backup/Cargo.lock"

restore() {
	mv -f "$backup/Cargo.toml" "$root/Cargo.toml"
	mv -f "$backup/Cargo.lock" "$root/Cargo.lock"
}
trap restore EXIT

# Take the version from [workspace.package] and rewrite the two shapes that
# carry it: the workspace's own version, and the requirements that the
# [workspace.dependencies] path entries place on sibling crates. Registry pins
# such as `version = "=28.0.2"` do not match either, and are left alone.
PINNED_VERSION="$pinned_version" perl -0pi -e '
	my ($v) = /^\[workspace\.package\]\s*\nversion = "([^"]+)"/m;
	die "could not find [workspace.package] version\n" unless defined $v;
	my $p = $ENV{PINNED_VERSION};
	s/^version = "\Q$v\E"$/version = "$p"/mg;
	s/\{ version = "\Q$v\E",/{ version = "$p",/g;
' "$root/Cargo.toml"

"$@"
