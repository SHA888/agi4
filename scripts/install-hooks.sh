#!/bin/bash
# Install git hooks for the agi4 project

REPO_ROOT=$(git rev-parse --show-toplevel)
HOOKS_DIR="$REPO_ROOT/hooks"
GIT_HOOKS_DIR="$REPO_ROOT/.git/hooks"

echo "📦 Installing git hooks..."

# Install pre-commit hook
if [ -f "$HOOKS_DIR/pre-commit" ]; then
    cp "$HOOKS_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
    chmod +x "$GIT_HOOKS_DIR/pre-commit"
    echo "✅ pre-commit hook installed"
else
    echo "⚠️  pre-commit hook not found at $HOOKS_DIR/pre-commit"
fi

echo "🎉 Git hooks installation complete!"
echo ""
echo "To uninstall hooks, run:"
echo "  rm $GIT_HOOKS_DIR/pre-commit"
