# bf-3fcq3j: ArgoCD sync verification findings

## Issue
The WorkflowTemplate `gribtract-ci` synced to iad-ci cluster does NOT match the committed version in git.

**Verification Date:** 2026-07-28

## Evidence

### Cluster WorkflowTemplate Status
- **Exists:** Yes (AGE: 23h, created 2026-07-27T19:06:20Z)
- **Location:** `iad-ci` cluster, `argo-workflows` namespace

### Remote template (kubectl get) differences:
1. **Different artifact names**:
   - Remote: `bench-results-json`, `dashboard-html`
   - Local: `bench-results`, `dashboard`
2. **Different resource limits**:
   - Remote: `cpu: 2000m, memory: 4Gi`
   - Local: `cpu: 1500m, memory: 3Gi`
3. **Missing echo statement**: Remote lacks `echo "Building commit: ${COMMIT}"`

### What matches
- Basic structure (entrypoint, serviceAccountName)
- Container image: `debian:bookworm`
- Core CI checks: `cargo fmt --check`, `cargo check`, `cargo clippy`, `cargo test`
- Core benchmark command: `cargo run --bin xtask -- bench`
- Environment variables for Git credential helper

## Root Cause: ArgoCD Application BROKEN

**Application:** `argo-workflows-resources-iad-ci`
- **Responsible for:** Syncing `k8s/iad-ci/argo-workflows` from `jedarden/declarative-config`
- **Sync Status:** Unknown (cannot generate manifests)
- **Health Status:** Healthy (incorrect - app is actually broken)
- **Last Reconciled:** 2026-07-28T18:20:52Z

**Error:**
```
ComparisonError: Failed to load target state: failed to generate manifest for source 1 of 1:
`kustomize edit add annotation managed-by:argocd` failed exit status 1:
Error: annotation managed-by already in kustomization file. Use --force to override.
```

**Issue:** ArgoCD's kustomize build is failing due to duplicate `managed-by: argocd` annotations in the kustomization file. This prevents manifest generation, so no syncs can occur.

The cluster version is from a previous manual apply or earlier successful sync, but current committed changes cannot sync.

## Next steps required
To fix ArgoCD sync:
1. Check `k8s/iad-ci/argo-workflows/kustomization.yaml` in `jedarden/declarative-config` repo
2. Remove duplicate `managed-by: argocd` annotation from kustomization
3. Trigger ArgoCD refresh/sync once fixed
4. Re-verify gribtract-ci template matches

## Acceptance Criteria Status
- ✅ `kubectl get workflowtemplate gribtract-ci` returns successfully (template exists)
- ❌ Template spec matches committed YAML (significant discrepancies found)

## Status
❌ **ARGOCD SYNC FAILED** - Application broken due to kustomize annotation error
