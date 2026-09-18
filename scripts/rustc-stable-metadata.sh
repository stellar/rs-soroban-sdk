#!/usr/bin/env bash
# A RUSTC_WRAPPER that makes the test wasm builds independent of the
# workspace version.
#
# Cargo derives rustc's `-C metadata` from each package's id, which includes
# its version, and that value seeds rustc's crate disambiguator and so every
# symbol hash. Fat LTO merges the upstream modules in an order taken from
# those symbol names, so bumping the workspace version reshuffles functions in
# the built wasm even though no code changed. That churn then lands in
# tests-expanded, which embeds the built wasms.
#
# For workspace crates this replaces the version-derived `-C metadata` with one
# derived only from the crate's identity: name, crate types, features and
# target. Crates from the registry keep Cargo's value, so a dependency bump
# still changes the output, as it should.
#
# WORKSPACE_ROOT must be the absolute path of the workspace root.
# RUSTC_STABLE_METADATA_INNER_WRAPPER, if set, is invoked in place of rustc so
# an existing wrapper such as sccache still runs.

set -uo pipefail

rustc="$1"
shift

ws="${WORKSPACE_ROOT:-$PWD}"
crate="" ctype="" feats="" tgt="" in_workspace=0 prev=""

for arg in "$@"; do
	case "$prev" in
	--crate-name) crate="$arg" ;;
	--crate-type) ctype="$ctype,$arg" ;;
	--target) tgt="$arg" ;;
	--cfg) case "$arg" in feature=*) feats="$feats,$arg" ;; esac ;;
	esac
	# The input path tells us whether this is a workspace crate. Registry
	# sources live outside the workspace root and keep Cargo's metadata.
	case "$arg" in "$ws"/*) in_workspace=1 ;; esac
	prev="$arg"
done

args=("$@")
if [ "$in_workspace" = 1 ] && [ -n "$crate" ]; then
	# `-C metadata` takes an arbitrary string, so use a readable one rather
	# than a hash. It only has to be distinct per unit and free of the version.
	seed="stable:$crate:$ctype:$feats:$tgt"
	i=0
	for arg in "${args[@]}"; do
		case "$arg" in
		metadata=*) args[$i]="metadata=$seed" ;;
		-Cmetadata=*) args[$i]="-Cmetadata=$seed" ;;
		esac
		i=$((i + 1))
	done
fi

if [ -n "${RUSTC_STABLE_METADATA_INNER_WRAPPER:-}" ]; then
	exec "$RUSTC_STABLE_METADATA_INNER_WRAPPER" "$rustc" "${args[@]}"
fi
exec "$rustc" "${args[@]}"
