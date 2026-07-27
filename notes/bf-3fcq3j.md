# bf-3fcq3j: ArgoCD sync verification findings

## Issue
The WorkflowTemplate `gribtract-ci` synced to iad-ci cluster does NOT match the committed version in git.

## Evidence

### Remote template (kubectl get) differences:
1. **Missing gh CLI installation** - The remote template omits the GitHub CLI setup steps
2. **Different artifact names**: 
   - Remote: `bench-results-json`, `dashboard-html`
   - Local: `bench-results`, `dashboard`
3. **Different resource limits**:
   - Remote: `cpu: 2000m, memory: 4Gi`
   - Local: `cpu: 1500m, memory: 3Gi`
4. **Different script messages**:
   - Remote: `echo "Building commit: ${COMMIT}"`, `echo "Benchmark artifacts generated successfully!"`, `ls -la bench-results.json dashboard.html`
   - Local: `echo "Benchmark results:"`, `cat bench-results.json`, `echo "Benchmark run completed successfully!"`
5. **Different error messages**:
   - Remote: `"Error: bench-results.json not generated"`
   - Local: `"bench-results.json not generated!"`

### What matches
- Basic structure (entrypoint, serviceAccountName)
- Container image: `debian:bookworm`
- Core CI checks: `cargo fmt --check`, `cargo check`, `cargo clippy`, `cargo test`
- Core benchmark command: `cargo run --bin xtask -- bench`
- File existence checks for artifacts
- Environment variables for Git credential helper

### ArgoCD tracking annotation present
```yaml
argocd.argoproj.io/tracking-id: argo-workflows-ns-iad-ci:argoproj.io/WorkflowTemplate:argo-workflows/gribtract-ci
```

## Root cause hypothesis
ArgoCD may have synced a stale version or there was a race condition during initial sync. The remote template's creation timestamp shows `2026-07-27T19:06:20Z` (approximately 61 minutes ago at time of verification).

## Next steps required
1. **Force ArgoCD resync** of the argo-workflows-ns-iad-ci application
2. **Re-verify** the remote template matches the committed version
3. **Consider** if there's a declarative-config drift issue

## Status
⚠️ **SYNC MISMATCH DETECTED** - ArgoCD sync verification failed
