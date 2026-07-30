# bf-3fcq3j: ArgoCD sync verification findings

## Issue
**ORIGINAL ISSUE:** The WorkflowTemplate `gribtract-ci` synced to iad-ci cluster does NOT match the committed version in git.

**CORRECTED FINDING:** ArgoCD synced correctly - the local file in the gribtract repo was never the source of truth.

**Verification Date:** 2026-07-28

## Evidence

### Cluster WorkflowTemplate Status
- **Exists:** Yes (AGE: 24h, created 2026-07-27T19:06:20Z)
- **Location:** `iad-ci` cluster, `argo-workflows` namespace
- **ArgoCD Tracking ID:** `argo-workflows-ns-iad-ci:argoproj.io/WorkflowTemplate:argo-workflows/gribtract-ci`

### Corrected Analysis

**Sources checked:**
1. **Local gribtract repo:** `/home/coding/gribtract/gribtract-ci.yaml`
2. **declarative-config repo:** `/home/coding/declarative-config/k8s/iad-ci/argo-workflows/gribtract-workflowtemplate.yml`
3. **Cluster version:** kubectl get from iad-ci

**Key Finding:** The cluster version MATCHES the declarative-config version exactly. This confirms ArgoCD synced correctly.

### Source of Truth

**ArgoCD syncs from:** `jedarden/declarative-config` repository
- **Application:** `argo-workflows-ns-iad-ci`
- **Path:** `k8s/iad-ci/argo-workflows/gribtract-workflowtemplate.yml`

**NOT the gribtract repository:** The local `gribtract-ci.yaml` file in the gribtract repo is not synced by ArgoCD and contains an outdated/different version.

### Differences Between Repos

**declarative-config (deployed to cluster):**
- No gh CLI installation
- Artifact names: `bench-results-json`, `dashboard-html`
- Resource limits: 2000m CPU, 4Gi memory

**gribtract repo local file (outdated):**
- Includes gh CLI installation
- Artifact names: `bench-results`, `dashboard`
- Resource limits: 1500m CPU, 3Gi memory

### What matches (cluster vs declarative-config)
- ✅ Complete spec matches exactly
- ✅ Resource limits: 2000m CPU, 4Gi memory
- ✅ Artifact names: `bench-results-json`, `dashboard-html`
- ✅ All container commands, environment variables, and configuration

## Root Cause Analysis

**Original error diagnosis was incorrect.** The issue wasn't an ArgoCD sync failure - it was confusion about which repository is the source of truth.

**Correct understanding:**
1. ArgoCD successfully synced the WorkflowTemplate from declarative-config to iad-ci
2. The tracking annotation confirms ArgoCD management
3. The local file in the gribtract repo should be updated or removed to avoid confusion

## Recommendation

**For the gribtract repository:**
- Update `gribtract-ci.yaml` to match the deployed version
- OR add a README note explaining that declarative-config is the source of truth
- OR remove the local file entirely to prevent confusion

**For future reference:**
- All CI/CD resources are managed via `jedarden/declarative-config`
- ArgoCD syncs from declarative-config, not from individual application repos

## Acceptance Criteria Status
- ✅ `kubectl get workflowtemplate gribtract-ci` returns successfully (template exists)
- ✅ Template spec matches deployed YAML (cluster matches declarative-config)

## Status
✅ **ARGOCD SYNC VERIFIED** - WorkflowTemplate correctly synced from declarative-config to iad-ci cluster

**Note:** Original analysis incorrectly identified this as an ArgoCD sync failure. The actual issue was comparing against the wrong source file (gribtract repo instead of declarative-config).
