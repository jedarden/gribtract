#!/usr/bin/env bash
# gribtract marathon launcher
#
# Starts an autonomous single-loop Claude session (via claude-print, which drives
# the interactive TUI over a PTY to preserve subscription billing) that does one
# bead of gribtract per iteration, re-reading .marathon/instruction.md each pass.
#
#   ./.marathon/start.sh            # preflight, then start the tmux loop
#   ./.marathon/start.sh --check    # preflight only, do not launch
#
# Env overrides:
#   MARATHON_MODEL   model passed to claude-print (default: sonnet)
#   MARATHON_DELAY   seconds between iterations (default: 8)
#   MARATHON_SESSION tmux session name (default: gribtract-marathon)
set -euo pipefail

REPO="/home/coding/gribtract"
SKILL="/home/coding/claude-config/skills/marathon-coding"
PROMPT="$REPO/.marathon/instruction.md"
LOG_DIR="$REPO/.marathon/logs"
MODEL="${MARATHON_MODEL:-sonnet}"
DELAY="${MARATHON_DELAY:-8}"
SESSION="${MARATHON_SESSION:-gribtract-marathon}"
CHECK_ONLY=false
[[ "${1:-}" == "--check" ]] && CHECK_ONLY=true

fail() { echo "PREFLIGHT FAIL: $*" >&2; exit 1; }
ok()   { echo "  ok: $*"; }

echo "== gribtract marathon preflight =="
cd "$REPO" || fail "repo not found: $REPO"

command -v claude-print >/dev/null      || fail "claude-print not on PATH"; ok "claude-print present"
command -v br >/dev/null                || fail "br not on PATH";           ok "br present"
[[ -x "$SKILL/launcher.sh" ]]           || fail "skill launcher missing: $SKILL/launcher.sh"; ok "skill launcher present"
[[ -f "$PROMPT" ]]                      || fail "instruction.md missing";   ok "instruction.md present"
[[ -f "docs/plan/plan.md" ]]            || fail "plan.md missing";          ok "plan.md present"
[[ -d ".beads" ]]                       || fail ".beads workspace missing"; ok ".beads present"
git rev-parse --git-dir >/dev/null 2>&1 || fail "not a git repo";           ok "git repo"
git remote get-url origin >/dev/null 2>&1 || fail "no origin remote";       ok "origin: $(git remote get-url origin)"

# A NEEDLE worker on this repo would fight the marathon for the shared worktree.
if pgrep -af 'needle' 2>/dev/null | grep -q "$REPO"; then
  fail "a NEEDLE worker appears to be running on $REPO — stop it first (shared worktree)"
fi
ok "no NEEDLE worker on this repo"

# Must have something to work on.
READY="$(br ready --limit 5000 2>/dev/null | grep -c '^\[' || true)"
[[ "${READY:-0}" -gt 0 ]] || fail "no ready beads — seed the queue first"
ok "$READY ready bead(s) — frontier:"
br ready --limit 5000 2>/dev/null | head -3 | sed 's/^/      /'

if tmux has-session -t "$SESSION" 2>/dev/null; then
  fail "tmux session '$SESSION' already running — 'tmux attach -t $SESSION' or kill it"
fi
ok "tmux session name '$SESSION' free"

echo "== preflight passed =="
if $CHECK_ONLY; then
  echo "(--check) not launching."
  exit 0
fi

mkdir -p "$LOG_DIR"
echo "== launching marathon: session=$SESSION model=$MODEL delay=${DELAY}s =="
# Launch the skill loop directly so cwd is pinned to the repo root (marathon.sh
# would otherwise set cwd to the prompt file's dir, i.e. .marathon/).
tmux new-session -d -s "$SESSION" -c "$REPO" \
  "$SKILL/launcher.sh --prompt '$PROMPT' --model $MODEL --delay $DELAY --log-dir '$LOG_DIR'"

echo "started. commands:"
echo "  attach: tmux attach -t $SESSION"
echo "  detach: Ctrl+B, D"
echo "  stop:   tmux kill-session -t $SESSION"
echo "  logs:   ls $LOG_DIR"
