#!/usr/bin/env bash
set -euo pipefail

# Find .js files that likely contain JSX and rename them to .jsx
# Also update any explicit imports that reference .js files to .jsx

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

echo "Scanning for .js files containing JSX..."
JSX_FILES=$(grep -RIl --include="*.js" -E "<[A-Z][A-Za-z0-9]*" src || true)

if [ -z "$JSX_FILES" ]; then
  echo "No .js files with JSX detected."
  exit 0
fi

echo "Found files:"
echo "$JSX_FILES"

echo ""
read -p "Proceed to rename these files to .jsx? [y/N] " confirm
if [ "${confirm:-N}" != "y" ] && [ "${confirm:-N}" != "Y" ]; then
  echo "Aborted by user."
  exit 0
fi

echo ""
for f in $JSX_FILES; do
  new="${f%.js}.jsx"
  echo "Renaming: $f -> $new"
  git mv "$f" "$new"
done

echo ""
echo "Updating explicit import paths that end with .js -> .jsx"
# Update imports that explicitly include .js extension
grep -RIl --include="*.{js,jsx,ts,tsx}" -e "\.js['\"]" src | while read -r file; do
  sed -Ei "s/((from|require)\s*\(['\"]([^'\"']+))\.js(['\"]\))/\1.jsx\4/g" "$file" || true
done

echo ""
echo "Done. Please run your dev server and verify."
echo "If you want to commit these changes, use: git status && git add -A && git commit -m \"Rename JSX .js -> .jsx\""

exit 0




