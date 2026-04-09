# 🚀 GitOps CI/CD Implementation Guide - Step by Step

## Overview

This guide walks you through implementing an enterprise-grade GitOps CI/CD pipeline from scratch. No prior experience required!

**What you're building:**

- Automated testing and security scanning on every Pull Request
- Automated Docker image builds and pushes to ECR
- GitOps-based deployment with ArgoCD
- Multi-environment promotion (dev → staging → production)
- Full audit trail and approval gates

---

## Part 1: Understanding What You Have

You've created 2 workflow files:

1. **`.github/workflows/pr-checks.yml`** - Runs when PR is created
   - Tests your code
   - Scans for security issues
   - Gives fast feedback (3-5 minutes)
   - Blocks merge if tests fail

2. **`.github/workflows/deploy-microservices.yml`** (will rename to `build-and-push.yml`)
   - Runs when code is merged
   - Builds Docker images
   - Pushes to Amazon ECR
   - Does NOT deploy (ArgoCD does that)

---

## Part 2: Prerequisites - What You Need Before Starting

### ✅ Infrastructure State

**Current status:** Your infrastructure is RUNNING from Phase 4

**Option 1: Keep it running** (costs ~$150/month)

- You can test immediately
- Skip Part 3

**Option 2: Destroy it to save money** (recommended while setting up workflows)

```powershell
# In PowerShell
cd "C:\Users\chris\Documents\AWS Projects and Challenges\AWS DevOps Mastery\terraform\environments\dev"

# Destroy everything
terraform destroy -auto-approve

# This saves ~$150/month while you work on workflow code
```

**When to bring it back:**

- When you're ready to test the full pipeline
- Infrastructure recreates in ~10 minutes with `terraform apply`

### ✅ GitHub Repository Website

You'll need to do some configuration in GitHub's website. Open:

```
https://github.com/YOUR-USERNAME/aws-devops-mastery
```

---

## Part 3: Manual Steps You Need to Do

### Step 1: Rename `main` Branch to `staging`

**Why:** Makes it clear that the `staging` branch deploys to staging environment.

**In GitHub Website:**

1. Go to your repository
2. Click **Settings** (top menu)
3. Click **Branches** (left sidebar)
4. Find "Default branch" section
5. Click the swap icon ↔️ next to `main`
6. Type: `staging`
7. Click "Rename branch"
8. Click "I understand, update default branch"

**In Your Local Computer (PowerShell):**

```powershell
# Navigate to your project
cd "C:\Users\chris\Documents\AWS Projects and Challenges\AWS DevOps Mastery"

# Rename your local branch
git branch -m main staging

# Update the tracking branch
git fetch origin
git branch -u origin/staging staging

# Verify it worked
git branch -vv
# Should show: * staging [origin/staging]
```

### Step 2: Rename Workflow File

The file `deploy-microservices.yml` now only builds images (doesn't deploy). Let's rename it:

**In PowerShell:**

```powershell
cd "C:\Users\chris\Documents\AWS Projects and Challenges\AWS DevOps Mastery\.github\workflows"

# Rename the file
mv deploy-microservices.yml build-and-push.yml

# Stage the changes
git add .

# Commit
git commit -m "Rename workflow to reflect GitOps pattern - now only builds/pushes images"
```

### Step 3: Configure Branch Protection Rules

This prevents code from being merged without passing checks.

**In GitHub Website:**

1. Go to **Settings** → **Branches**
2. Click "Add rule" under "Branch protection rules"

**For `develop` branch:**

- Branch name pattern: `develop`
- ✅ Check: "Require a pull request before merging"
- ✅ Check: "Require status checks to pass before merging"
  - Search and add: `Check JavaScript API`, `Check Java Service`, `Check Rust Processor`
- ✅ Check: "Require branches to be up to date before merging"
- ✅ Check: "Do not allow bypassing the above settings"
- Click "Create"

**For `staging` branch:**

- Branch name pattern: `staging`
- ✅ Check: "Require a pull request before merging"
- ✅ Check: "Require approvals" → Set to 1
- ✅ Check: "Dismiss stale pull request approvals when new commits are pushed"
- ✅ Check: "Require status checks to pass before merging"
- ✅ Check: "Require linear history"
- ✅ Check: "Do not allow bypassing the above settings"
- Click "Create"

### Step 4: Create `develop` Branch

**In PowerShell:**

```powershell
# Make sure you're on staging branch
git checkout staging

# Create develop branch from staging
git checkout -b develop

# Push to GitHub
git push origin develop

# Now you have:
# - develop branch (for active development)
# - staging branch (for staging environment)
```

---

## Part 4: Testing the PR Checks Workflow

Let's test that automated tests/scans work on Pull Requests.

### Test Steps:

**Step 1: Make a Small Change**

```powershell
# Create a feature branch
git checkout develop
git checkout -b feature/test-pr-checks

# Make a small change (example: update JavaScript API)
code services/javascript-api/src/index.js

# Add a comment in the file:
// Testing PR checks workflow
```

**Step 2: Commit and Push**

```powershell
git add services/javascript-api/src/index.js
git commit -m "test: Add comment to test PR checks workflow"
git push origin feature/test-pr-checks
```

**Step 3: Create Pull Request**

1. Go to GitHub repository
2. You'll see a banner: "feature/test-pr-checks had recent pushes"
3. Click "Compare & pull request"
4. **Base:** `develop` (not staging!)
5. Title: "Test PR checks workflow"
6. Click "Create pull request"

**Step 4: Watch the Magic! ✨**

GitHub Actions will automatically:

1. Detect that JavaScript files changed
2. Build JavaScript API
3. Run tests (if you have any)
4. Scan with Trivy for vulnerabilities
5. Report results

**What you'll see:**

- Yellow circle ⭕ = Running (takes 3-5 minutes)
- Green checkmark ✅ = All checks passed (you can merge!)
- Red X ❌ = Something failed (need to fix before merging)

**If checks pass:**

1. Click "Merge pull request"
2. Click "Confirm merge"
3. Delete the feature branch

**What happens after merge:**

- The `build-and-push.yml` workflow triggers
- Builds production Docker image
- Pushes to ECR with tag `latest-dev`
- (ArgoCD would deploy it, but we haven't set that up yet)

---

## Part 5: Understanding Image Tags

Your workflow creates different tags based on which branch you merge to:

| Branch/Tag | When                     | Image Tags Created            | Deployed Where                    |
| ---------- | ------------------------ | ----------------------------- | --------------------------------- |
| `develop`  | Merge PR to develop      | `latest-dev`, `{git-sha}`     | Dev environment (auto)            |
| `staging`  | Merge develop to staging | `latest-staging`, `{git-sha}` | Staging environment (manual PR)   |
| `v1.2.3`   | Create version tag       | `v1.2.3`, `latest`            | Production (manual PR + approval) |

**Example:**

```powershell
# Merge to develop
# → Image: rust-processor:latest-dev
# → Image: rust-processor:a3f8d91c2e (git SHA)

# Merge to staging
# → Image: rust-processor:latest-staging
# → Image: rust-processor:b5e9f2341a (git SHA)

# Tag v1.0.0
git tag v1.0.0
git push origin v1.0.0
# → Image: rust-processor:v1.0.0
# → Image: rust-processor:latest
```

---

## Part 6: ArgoCD Installation (When Infrastructure is Ready)

**Prerequisites:**

- EKS cluster running (recreate with `terraform apply`)
- kubectl configured

### Step 1: Install ArgoCD

```powershell
# Create argocd namespace
kubectl create namespace argocd

# Install ArgoCD
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Wait for pods to be ready (takes 2-3 minutes)
kubectl get pods -n argocd -w

# Get admin password
$ARGOCD_PASSWORD = kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | ForEach-Object { [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String($_)) }
Write-Host "ArgoCD Password: $ARGOCD_PASSWORD"

# Port-forward to access UI
kubectl port-forward svc/argocd-server -n argocd 8080:443

# Open browser: https://localhost:8080
# Username: admin
# Password: (the password from above)
```

### Step 2: Install ArgoCD Image Updater

```powershell
# Install Image Updater
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj-labs/argocd-image-updater/stable/manifests/install.yaml

# Verify installation
kubectl get pods -n argocd -l app.kubernetes.io/name=argocd-image-updater
```

### Step 3: Configure ECR Access for Image Updater

```powershell
# Create secret for ECR access
kubectl create secret generic ecr-credentials \
  --from-literal=username=AWS \
  --from-literal=password="$(aws ecr get-login-password --region us-east-2)" \
  -n argocd

# Configure Image Updater to use secret
kubectl patch configmap argocd-image-updater-config -n argocd --patch '
data:
  registries.conf: |
    registries:
      - name: ecr
        api_url: https://737026300147.dkr.ecr.us-east-2.amazonaws.com
        prefix: 737026300147.dkr.ecr.us-east-2.amazonaws.com
        credentials: secret:argocd/ecr-credentials
        default: true
'
```

---

## Part 7: Creating a GitOps Repository (Later Phase)

This is a whole separate project for Phase 7. For now, you have:

✅ Automated PR checks (tests + security scans)
✅ Automated image builds and pushes to ECR
✅ Understanding of GitOps concepts
✅ Branch protection and approval gates

**Next steps (Phase 7):**

1. Create separate `aws-devops-mastery-gitops` repository
2. Add deployment manifests for dev/staging/production
3. Configure ArgoCD Applications
4. Set up CODEOWNERS for approval gates
5. Test promotion workflow (dev → staging → production)

---

## Part 8: Daily Development Workflow

Once everything is set up, here's your typical day:

**Morning: Start new feature**

```powershell
git checkout develop
git pull origin develop
git checkout -b feature/add-redis-cache
# ... make changes ...
git add .
git commit -m "feat: Add Redis caching to API endpoints"
git push origin feature/add-redis-cache
```

**Create PR on GitHub:**

- PR checks run automatically (3-5 min)
- If green ✅ → Request review from senior engineer
- If red ❌ → Fix issues, push again (checks re-run automatically)

**After approval:**

- Merge PR
- `build-and-push` workflow runs automatically
- Image pushed to ECR with `latest-dev` tag
- (ArgoCD Image Updater sees new image)
- (ArgoCD deploys to dev cluster automatically)
- Test in dev environment

**Promote to staging:**

```powershell
git checkout staging
git merge develop
git push origin staging
# → Builds images tagged `latest-staging`
# → Create PR to GitOps repo (staging folder)
# → Get 1 approval → Merge → ArgoCD deploys
```

**Promote to production:**

```powershell
git checkout staging
git tag v1.2.3
git push origin v1.2.3
# → Builds images tagged `v1.2.3` and `latest`
# → Create PR to GitOps repo (production folder)
# → Get 2 approvals → Merge → ArgoCD deploys
```

---

## Part 9: Troubleshooting Common Issues

### Issue: "Workflow didn't trigger"

**Check:**

1. Did you push to the right branch? (`develop` or `staging`)
2. Did you change files in `services/` folder?
3. Check Actions tab → See if workflow is queued

### Issue: "Tests failed"

**Check:**

1. Click on the failing check in PR
2. Expand the step that failed
3. Read the error message
4. Fix locally, push again

### Issue: "Trivy scan failed"

**This is good!** It found a vulnerability.

**Fix:**

1. Read the Trivy output (click Details)
2. Update vulnerable dependency
3. Push updated code

### Issue: "Can't merge PR"

**Check:**

1. Are there red X marks? Fix failing checks first
2. Did you get approval? (needed for staging branch)
3. Is branch up to date? Click "Update branch" button

---

## Part 10: What You've Achieved

✅ **Automated quality gates** - No broken code reaches production
✅ **Fast feedback** - Developers know in 3-5 min if code is good
✅ **Security scanning** - Catches vulnerabilities before deployment
✅ **Branch protection** - Prevents bypassing checks
✅ **Approval gates** - Senior engineers review critical changes
✅ **GitOps ready** - Foundation for ArgoCD deployment
✅ **Enterprise pattern** - Used by companies paying $250k+ salaries

**Interview talking points:**

- "I implemented a two-stage CI/CD pipeline separating PR checks from builds"
- "I configured branch protection rules that enforce code review and automated testing"
- "I set up language-specific security scanning with Trivy for container vulnerabilities"
- "I designed a GitOps workflow where CI builds artifacts and ArgoCD deploys them"

---

## Next Steps

1. Test PR checks workflow (Part 4)
2. Merge a PR and watch images build
3. Destroy infrastructure to save costs while learning ArgoCD concepts
4. When ready, recreate infrastructure and install ArgoCD (Part 6)
5. Proceed to Phase 7 for full GitOps implementation

**Questions? Check:**

- Workflow logs in Actions tab
- AWS CloudWatch for Kubernetes logs
- ArgoCD UI for deployment status

---

**You're now implementing senior-level CI/CD patterns! 🚀**
