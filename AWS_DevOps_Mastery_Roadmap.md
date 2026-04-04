# 🚀 Enterprise AWS, Kubernetes & Multi-Language CI/CD - Complete Platform Engineering Mastery

**Version:** 2.0  
**Target Role:** Senior/Lead Platform Engineer, Staff SRE, Principal Kubernetes Engineer, Senior DevOps Architect  
**Salary Range:** $180k-$300k (US markets)  
**Learning Time:** 60-80 hours for complete mastery  
**AWS Cost:** ~$50-200/mo (with cost controls and terraform destroy)  
**Focus:** Production-grade AWS + Kubernetes + Multi-language CI/CD

---

## 📋 Table of Contents

1. [Project Overview](#project-overview)
2. [Why This Project Will Get You Hired](#why-this-project-will-get-you-hired)
3. [Learning Objectives](#learning-objectives)
4. [Architecture Overview](#architecture-overview)
5. [Phase 0: AWS OIDC Setup (No Access Keys!)](#phase-0-aws-oidc-setup)
6. [Phase 1: Enterprise Terraform Isolation with AWS](#phase-1-enterprise-terraform-isolation-with-aws)
7. [Phase 2: EKS Cluster with ECR](#phase-2-eks-cluster-with-ecr)
8. [Phase 3: Advanced Kubernetes & Helm Mastery](#phase-3-advanced-kubernetes--helm-mastery)
9. [Phase 4: Multi-Language CI/CD Pipeline](#phase-4-multi-language-cicd-pipeline)
10. [Phase 5: Environment-Specific Security Scanning](#phase-5-environment-specific-security-scanning)
11. [Phase 6: Integration & GitOps](#phase-6-integration--gitops)
12. [Cost Management & Terraform Destroy](#cost-management--terraform-destroy)
13. [Kubernetes Troubleshooting Scenarios](#kubernetes-troubleshooting-scenarios)
14. [Interview Questions You'll Master](#interview-questions-youll-master)
15. [Implementation Roadmap](#implementation-roadmap)
16. [Real Company Patterns](#real-company-patterns)

---

## 🎯 Project Overview

### **What You're Building**

A production-grade, enterprise-level infrastructure platform featuring:

1. **Complete Terraform Environment Isolation**
   - Three fully isolated environments (dev/staging/production)
   - Remote state with locking (S3 + DynamoDB OR Terraform Cloud)
   - Environment-specific configurations
   - CI/CD deployment gates
   - Cost optimization per environment

2. **Multi-Language Microservices CI/CD**
   - **JavaScript (Node.js)** - REST API service
   - **Java (Spring Boot)** - Business logic service
   - **Rust** - High-performance data processor
   - Matrix build strategy (parallel execution)
   - Language-specific security scanning
   - Container image optimization per language
   - Kubernetes deployment with Helm

3. **GitOps Deployment Pattern**
   - ArgoCD for automated synchronization
   - Environment promotion workflow
   - Rollback capabilities
   - Full audit trail

### **Tech Stack**

| Category                    | Technologies                                                            |
| --------------------------- | ----------------------------------------------------------------------- |
| **Infrastructure as Code**  | Terraform, AWS (EKS, VPC, RDS, S3, ECR)                                 |
| **Container Orchestration** | Kubernetes (EKS), Helm Charts, Docker                                   |
| **Container Registry**      | Amazon ECR (Elastic Container Registry)                                 |
| **Authentication**          | GitHub OIDC → AWS (No Access Keys!)                                     |
| **CI/CD**                   | GitHub Actions, Matrix Builds, Environment-Specific Scanning            |
| **Languages**               | JavaScript/Node.js, Java/Spring Boot, Rust                              |
| **GitOps**                  | ArgoCD, Helm Values Updates                                             |
| **Security**                | Trivy, Semgrep, gitleaks, OWASP, tfsec, environment-aware               |
| **Monitoring**              | Prometheus, Grafana, CloudWatch, Container Insights                     |
| **State Management**        | S3 + DynamoDB (per environment)                                         |
| **Kubernetes Advanced**     | HPA, Cluster Autoscaler, Network Policies, RBAC, Pod Disruption Budgets |

---

## � What You'll Learn & Career Impact

This project mirrors real-world enterprise infrastructure used by Fortune 500 companies. You'll master production-grade AWS and Kubernetes—from OIDC authentication (eliminating risky access keys) to building EKS clusters with advanced features like Horizontal Pod Autoscaling, Pod Disruption Budgets, and zero-trust network policies. Upon completion, you'll have a portfolio demonstrating architectural decision-making, security consciousness, and operational excellence, positioning you for: Senior Platform Engineer ($160k-$200k), Lead DevOps Engineer ($180k-$230k), Staff SRE ($220k-$280k), or Principal Kubernetes Engineer ($250k-$320k). These are current market rates for engineers who confidently discuss production system design, explain trade-offs, troubleshoot distributed systems, and articulate how infrastructure impacts business outcomes.

---

## �💼 Why This Project Will Get You Hired

### **What Makes This "Senior-Level"**

| Junior Engineer            | Mid-Level Engineer       | **Senior/Lead Engineer (You)**                                           |
| -------------------------- | ------------------------ | ------------------------------------------------------------------------ |
| Deploys to one environment | Has dev and prod configs | **Complete environment isolation with state locking**                    |
| Uses one language          | Works with 2 languages   | **Orchestrates polyglot microservices architecture**                     |
| Manual deployments         | Basic CI/CD              | **GitOps with approval gates and audit trails**                          |
| Terraform in one folder    | Separate tfvars files    | **Backend isolation, CODEOWNERS, cost-optimized configs**                |
| Basic Docker builds        | Multi-stage builds       | **Language-optimized builds with layer caching**                         |
| Runs tests                 | Runs tests + linting     | **6-layer security scanning (SAST, SCA, secrets, container, IaC, DAST)** |
| Can explain their work     | Documents their work     | **Backs decisions with industry patterns from FAANG companies**          |

### **Interview Impact**

When you complete this, you can confidently say:

> "I've implemented enterprise Terraform isolation patterns used by companies like HashiCorp and Grammarly, with complete state separation across environments and DynamoDB locking to prevent concurrent applies. I've also built a polyglot CI/CD pipeline handling JavaScript, Java, and Rust microservices with matrix builds optimized for each language's ecosystem, achieving sub-5-minute build times through intelligent caching."

**Hiring managers think:** "This person has done the hard work. They understand production systems."

---

## 🎓 Learning Objectives

By completing this project, you will master:

### **Terraform Mastery**

- ✅ Remote state management (S3 + DynamoDB locking)
- ✅ Terraform Cloud backends (no AWS required alternative)
- ✅ Environment-specific variable files
- ✅ Backend configurations per environment
- ✅ Module composition and reusability
- ✅ State migration strategies
- ✅ Cost tagging and optimization
- ✅ Preventing accidental production destruction

### **Multi-Language CI/CD**

- ✅ Matrix build strategies for parallel execution
- ✅ Language-specific dependency caching (npm, Maven, Cargo)
- ✅ Container image optimization per language
- ✅ Multi-stage Docker builds
- ✅ Security scanning for each language ecosystem
- ✅ Build artifact management
- ✅ Deployment orchestration across services
- ✅ Handling polyglot monorepo structures

### **GitOps & Deployment**

- ✅ ArgoCD application configurations
- ✅ Environment promotion workflows (dev → staging → prod)
- ✅ Approval gates and manual intervention
- ✅ Rollback strategies
- ✅ Audit logging
- ✅ Drift detection and remediation

### **Security Engineering**

- ✅ SAST (Static Application Security Testing)
- ✅ SCA (Software Composition Analysis)
- ✅ Container vulnerability scanning
- ✅ Infrastructure as Code security
- ✅ Secret management across environments
- ✅ RBAC and least privilege

### **Production Operations**

- ✅ Blue/green deployments
- ✅ Canary releases
- ✅ Health checks and readiness probes
- ✅ Resource limits and requests
- ✅ Horizontal Pod Autoscaling
- ✅ Cost monitoring and optimization

---

## 🏗️ Architecture Overview

### **High-Level Architecture**

```
┌─────────────────────────────────────────────────────────────────┐
│                     GITHUB REPOSITORY                            │
│                                                                  │
│  ┌────────────────────┐  ┌────────────────────────────────────┐│
│  │ terraform/         │  │ services/                          ││
│  │ ├── environments/  │  │ ├── javascript-api/                ││
│  │ │   ├── dev/       │  │ │   ├── src/                      ││
│  │ │   ├── staging/   │  │ │   ├── Dockerfile                ││
│  │ │   └── production/│  │ │   └── package.json              ││
│  │ └── modules/       │  │ ├── java-service/                 ││
│  │                    │  │ │   ├── src/                      ││
│  │                    │  │ │   ├── Dockerfile                ││
│  │                    │  │ │   └── pom.xml                   ││
│  │                    │  │ └── rust-processor/               ││
│  │                    │  │     ├── src/                      ││
│  │                    │  │     ├── Dockerfile                ││
│  │                    │  │     └── Cargo.toml                ││
│  └────────────────────┘  └────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                               │
            ┌──────────────────┼──────────────────┐
            ▼                  ▼                  ▼
    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
    │ DEV          │  │ STAGING      │  │ PRODUCTION   │
    │ Environment  │  │ Environment  │  │ Environment  │
    ├──────────────┤  ├──────────────┤  ├──────────────┤
    │ Terraform    │  │ Terraform    │  │ Terraform    │
    │ State: S3    │  │ State: S3    │  │ State: S3    │
    │ dev bucket   │  │ stg bucket   │  │ prod bucket  │
    ├──────────────┤  ├──────────────┤  ├──────────────┤
    │ EKS Cluster  │  │ EKS Cluster  │  │ EKS Cluster  │
    │ - t3.medium  │  │ - m5.large   │  │ - m5.2xlarge │
    │ - 2 nodes    │  │ - 3 nodes    │  │ - 6 nodes    │
    │ - Single AZ  │  │ - Multi-AZ   │  │ - Multi-AZ   │
    ├──────────────┤  ├──────────────┤  ├──────────────┤
    │ Services:    │  │ Services:    │  │ Services:    │
    │ - JS API     │  │ - JS API     │  │ - JS API     │
    │ - Java Svc   │  │ - Java Svc   │  │ - Java Svc   │
    │ - Rust Proc  │  │ - Rust Proc  │  │ - Rust Proc  │
    └──────────────┘  └──────────────┘  └──────────────┘
         ~$100/mo         ~$400/mo        ~$1200/mo
```

### **CI/CD Pipeline Flow**

```
Developer pushes code to feature branch
            ↓
┌───────────────────────────────────────────────────────────┐
│ STAGE 1: BUILD & TEST (Parallel Matrix)                  │
├───────────────────────────────────────────────────────────┤
│                                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ JavaScript  │  │   Java      │  │    Rust     │     │
│  │ - npm ci    │  │ - mvn test  │  │ - cargo tst │     │
│  │ - npm test  │  │ - build jar │  │ - cargo bld │     │
│  │ - build img │  │ - build img │  │ - build img │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
│       2 min            4 min           3 min            │
│                                                           │
│  Total time: 4 minutes (parallel, not 9!)               │
└───────────────────────────────────────────────────────────┘
            ↓
┌───────────────────────────────────────────────────────────┐
│ STAGE 2: SECURITY SCANNING (Parallel)                    │
├───────────────────────────────────────────────────────────┤
│  - SAST (Semgrep)         - Secret Scanning (gitleaks)   │
│  - SCA (Snyk/OWASP)       - Container (Trivy)            │
│  - IaC (tfsec)            Total: 2 minutes               │
└───────────────────────────────────────────────────────────┘
            ↓
┌───────────────────────────────────────────────────────────┐
│ STAGE 3: DEPLOY TO DEV (Auto)                            │
├───────────────────────────────────────────────────────────┤
│  - Update Helm values.yaml                                │
│  - Commit to Git                                          │
│  - ArgoCD detects change → deploys                        │
└───────────────────────────────────────────────────────────┘
            ↓
┌───────────────────────────────────────────────────────────┐
│ STAGE 4: PROMOTE TO STAGING (Manual Approval)            │
├───────────────────────────────────────────────────────────┤
│  - Run integration tests                                  │
│  - Require 1 approval                                     │
│  - Deploy via ArgoCD                                      │
└───────────────────────────────────────────────────────────┘
            ↓
┌───────────────────────────────────────────────────────────┐
│ STAGE 5: PROMOTE TO PRODUCTION (Manual Approval)         │
├───────────────────────────────────────────────────────────┤
│  - Require 2 approvals (senior engineers)                │
│  - Blue/green deployment                                  │
│  - Smoke tests                                            │
│  - Monitor for 10 minutes                                 │
└───────────────────────────────────────────────────────────┘
```

---

## � Phase 0: AWS OIDC Setup (No Access Keys!)

### **Learning Outcomes**

After this phase, you can answer:

- "How do you securely authenticate GitHub Actions to AWS without access keys?"
- "Explain OpenID Connect (OIDC) and why it's better than IAM users."
- "How do you implement least-privilege access for CI/CD?"
- "Walk me through your GitHub → AWS authentication flow."

### **0.1: Why NO ACCESS KEYS?**

**The Problem with Access Keys:**

```yaml
# ❌ DANGEROUS: Access keys in GitHub Secrets
env:
  AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
  AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}

# Problems:
# 1. Keys never expire (unless manually rotated)
# 2. If leaked, attacker has permanent access
# 3. Can't restrict to specific repos/branches
# 4. 90-day rotation burden
# 5. Hard to audit which workflow used which key
```

**The Solution: OIDC (OpenID Connect)**

```yaml
# ✅ SECURE: Temporary credentials via OIDC
permissions:
  id-token: write  # Request OIDC token
  contents: read

- name: Configure AWS credentials
  uses: aws-actions/configure-aws-credentials@v4
  with:
    role-to-assume: arn:aws:iam::123456789012:role/GitHubActionsRole
    aws-region: us-east-1

# Benefits:
# ✅ Credentials expire in 1 hour (auto-renewed)
# ✅ Can restrict to specific repos/branches
# ✅ No secrets to rotate
# ✅ Full CloudTrail audit trail
# ✅ Can't be leaked (temporary tokens)
```

### **0.2: How OIDC Works**

```
┌──────────────────────────────────────────────────────────────┐
│ Step 1: GitHub Actions workflow starts                       │
├──────────────────────────────────────────────────────────────┤
│ Workflow: "I need to deploy to AWS"                         │
│ GitHub: "Here's a signed OIDC token proving you're          │
│          running from repo 'mycompany/infra' on branch      │
│          'main' with commit sha 'abc123'"                   │
└──────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌──────────────────────────────────────────────────────────────┐
│ Step 2: Workflow sends token to AWS STS                     │
├──────────────────────────────────────────────────────────────┤
│ Workflow → AWS STS: "Here's my GitHub OIDC token"           │
│ AWS STS: "Let me verify this with GitHub's public keys..."  │
│ AWS STS: "Token is valid! Repo matches! Branch matches!"    │
│ AWS STS: "Here are temporary credentials (1 hour)"          │
└──────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌──────────────────────────────────────────────────────────────┐
│ Step 3: Workflow uses temporary credentials                 │
├──────────────────────────────────────────────────────────────┤
│ Workflow: terraform apply (with temp creds)                 │
│ AWS: "Credentials valid, IAM role allows this action ✅"     │
│                                                              │
│ After 1 hour: Credentials expire automatically              │
└──────────────────────────────────────────────────────────────┘
```

**Key Concepts:**

- **OIDC Token**: Signed JWT containing claims (repo, branch, actor)
- **Trust Policy**: AWS IAM role says "I trust GitHub's OIDC tokens"
- **Audience**: Prevents token reuse in other contexts
- **Subject (sub)**: Identifies exact repo/branch/environment

### **0.3: Setting Up AWS OIDC**

**Step 1: Create OIDC Provider in AWS**

```bash
# Run this ONCE per AWS account
aws iam create-open-id-connect-provider \
  --url https://token.actions.githubusercontent.com \
  --client-id-list sts.amazonaws.com \
  --thumbprint-list 6938fd4d98bab03faadb97b34396831e3780aea1
```

Or with Terraform (BETTER - infrastructure as code):

```hcl
# terraform/modules/github-oidc/main.tf
resource "aws_iam_openid_connect_provider" "github" {
  url = "https://token.actions.githubusercontent.com"

  client_id_list = [
    "sts.amazonaws.com"
  ]

  thumbprint_list = [
    "6938fd4d98bab03faadb97b34396831e3780aea1"
  ]

  tags = {
    Name = "GitHub Actions OIDC Provider"
  }
}
```

**Step 2: Create IAM Role with Trust Policy**

```hcl
# terraform/modules/github-oidc/main.tf (continued)
resource "aws_iam_role" "github_actions" {
  name = "GitHubActionsRole"

  # Trust policy: Allow GitHub OIDC tokens
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = aws_iam_openid_connect_provider.github.arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
          }
          StringLike = {
            # Only allow specific repos and branches
            "token.actions.githubusercontent.com:sub" = [
              "repo:yourcompany/infrastructure:ref:refs/heads/main",
              "repo:yourcompany/infrastructure:ref:refs/heads/develop",
              "repo:yourcompany/infrastructure:ref:refs/heads/staging"
            ]
          }
        }
      }
    ]
  })
}

# Attach policies for what the role can do
resource "aws_iam_role_policy_attachment" "github_actions_admin" {
  role       = aws_iam_role.github_actions.name
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"  # For demo
  # In production, use least-privilege custom policies!
}

output "role_arn" {
  value = aws_iam_role.github_actions.arn
  description = "Use this in GitHub Actions: role-to-assume"
}
```

**Step 3: Use in GitHub Actions**

```yaml
# .github/workflows/deploy.yml
name: Deploy Infrastructure

on:
  push:
    branches: [main, develop, staging]

permissions:
  id-token: write # CRITICAL: Request OIDC token
  contents: read

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Configure AWS credentials via OIDC
        uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: arn:aws:iam::123456789012:role/GitHubActionsRole
          aws-region: us-east-1
          # Optional: Restrict what this workflow can do
          role-session-name: GitHubActions-${{ github.run_id }}

      - name: Verify AWS identity
        run: aws sts get-caller-identity

      - name: Deploy with Terraform
        working-directory: terraform/environments/dev
        run: |
          terraform init
          terraform plan
          terraform apply -auto-approve
```

### **0.4: Environment-Specific OIDC Roles**

**Best Practice:** Separate IAM roles per environment

```hcl
# terraform/modules/github-oidc/main.tf

# Dev role: More permissive, allow feature branches
resource "aws_iam_role" "github_actions_dev" {
  name = "GitHubActionsRole-Dev"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        Federated = aws_iam_openid_connect_provider.github.arn
      }
      Action = "sts:AssumeRoleWithWebIdentity"
      Condition = {
        StringEquals = {
          "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
        }
        StringLike = {
          # Allow all branches to deploy to dev
          "token.actions.githubusercontent.com:sub" = "repo:yourcompany/infrastructure:*"
        }
      }
    }]
  })
}

# Production role: Highly restrictive, only main branch
resource "aws_iam_role" "github_actions_production" {
  name = "GitHubActionsRole-Production"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        Federated = aws_iam_openid_connect_provider.github.arn
      }
      Action = "sts:AssumeRoleWithWebIdentity"
      Condition = {
        StringEquals = {
          "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
          # ONLY main branch can deploy to production
          "token.actions.githubusercontent.com:sub" = "repo:yourcompany/infrastructure:ref:refs/heads/main"
        }
      }
    }]
  })
}
```

**GitHub Actions workflow uses different roles:**

```yaml
jobs:
  deploy-dev:
    if: github.ref == 'refs/heads/develop'
    steps:
      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: arn:aws:iam::123456789012:role/GitHubActionsRole-Dev

  deploy-production:
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: arn:aws:iam::123456789012:role/GitHubActionsRole-Production
```

### **0.5: Implementation Steps**

**Step 1: Create Terraform module**

```bash
mkdir -p terraform/modules/github-oidc
# Copy the Terraform code above
```

**Step 2: Deploy OIDC provider**

```bash
cd terraform/
terraform init
terraform apply -target=module.github_oidc
```

**Step 3: Get role ARN**

```bash
terraform output github_actions_role_arn
# Output: arn:aws:iam::123456789012:role/GitHubActionsRole
```

**Step 4: Test in GitHub Actions**

```yaml
# Create a test workflow that just prints AWS identity
- name: Test OIDC
  run: aws sts get-caller-identity
```

**Step 5: Celebrate** \ud83c\udf89  
No more access keys to rotate!

---

## 📚 Phase 1: Enterprise Terraform Isolation with AWS

### **Learning Outcomes**

After this phase, you can answer:

- "How do you prevent dev Terraform from affecting production?"
- "Explain your state management strategy with S3 and DynamoDB."
- "How do you handle concurrent Terraform applies?"
- "Walk me through your environment separation approach on AWS."
- "How do you manage Terraform state cleanup and cost control?"

### **1.1: Understanding State Isolation**

**The Problem:**

```bash
# Bad: Single statefile
terraform/
└── terraform.tfstate  ← Contains ALL environments

# Engineer runs: terraform destroy
# Question: "Does this destroy dev or prod?"
# Answer: "We don't know until it's too late!"
```

**The Solution:**

```bash
# Good: Isolated state per environment
terraform/
├── environments/
│   ├── dev/
│   │   └── .terraform/
│   │       └── terraform.tfstate  ← Only dev resources
│   ├── staging/
│   │   └── .terraform/
│   │       └── terraform.tfstate  ← Only staging resources
│   └── production/
│       └── .terraform/
│           └── terraform.tfstate  ← Only production resources
```

**Even Better: Remote State**

```hcl
# environments/production/backend.tf
terraform {
  backend "s3" {
    bucket         = "mycompany-terraform-state-prod"  # Production-only bucket
    key            = "infrastructure/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "terraform-lock-prod"  # Prevents concurrent applies
  }
}
```

**Why This Matters:**

- ✅ Destroying dev cannot touch production (different state files)
- ✅ DynamoDB locking prevents race conditions
- ✅ S3 versioning allows state rollback
- ✅ Encryption protects sensitive data in state

### **1.2: Directory Structure**

```
terraform/
├── environments/
│   ├── dev/
│   │   ├── main.tf              # Calls modules with dev params
│   │   ├── variables.tf         # Input variables
│   │   ├── terraform.tfvars     # Dev values (small instances)
│   │   ├── backend.tf           # Remote state config
│   │   └── outputs.tf           # Useful info after apply
│   │
│   ├── staging/
│   │   ├── main.tf              # Calls modules with staging params
│   │   ├── variables.tf
│   │   ├── terraform.tfvars     # Staging values (medium instances)
│   │   ├── backend.tf
│   │   └── outputs.tf
│   │
│   └── production/
│       ├── main.tf              # Calls modules with prod params
│       ├── variables.tf
│       ├── terraform.tfvars     # Production values (large instances, HA)
│       ├── backend.tf
│       └── outputs.tf
│
└── modules/                     # Reusable infrastructure components
    ├── vpc/
    │   ├── main.tf
    │   ├── variables.tf
    │   └── outputs.tf
    ├── eks/
    │   ├── main.tf
    │   ├── variables.tf
    │   └── outputs.tf
    └── rds/
        ├── main.tf
        ├── variables.tf
        └── outputs.tf
```

### **1.3: Environment-Specific Configurations**

**environments/dev/terraform.tfvars:**

```hcl
environment = "dev"

# Small, cheap resources
instance_type = "t3.medium"
node_count = 2
enable_multi_az = false
enable_deletion_protection = false  # Can destroy easily

# Development-friendly settings
enable_detailed_monitoring = false
backup_retention_days = 1

# Cost optimization
nat_gateway_count = 1  # Single NAT, not per-AZ
```

**environments/production/terraform.tfvars:**

```hcl
environment = "production"

# Production-grade resources
instance_type = "m5.2xlarge"
node_count = 6
enable_multi_az = true
enable_deletion_protection = true  # Cannot accidentally destroy!

# Production monitoring
enable_detailed_monitoring = true
backup_retention_days = 30

# High availability
nat_gateway_count = 2  # NAT per AZ for redundancy
```

### **1.4: Remote State Setup with AWS S3**

**Step 1: Create state bucket and lock table (Per Environment)**

Create this Terraform module to bootstrap state infrastructure:

```hcl
# terraform/bootstrap/main.tf
# Run this FIRST to create state buckets

terraform {
  # This uses LOCAL state to create remote state infrastructure
  # It's the "chicken and egg" problem - we need somewhere to start
}

provider "aws" {
  region = var.region
}

variable "region" {
  default = "us-east-1"
}

variable "environment" {
  description = "dev, staging, or production"
}

variable "project_name" {
  default = "mycompany"
}

# S3 bucket for Terraform state
resource "aws_s3_bucket" "terraform_state" {
  bucket = "${var.project_name}-terraform-state-${var.environment}"

  lifecycle {
    prevent_destroy = true  # Don't accidentally delete state!
  }

  tags = {
    Name        = "Terraform State ${title(var.environment)}"
    Environment = var.environment
  }
}

# Enable versioning (can rollback state if needed)
resource "aws_s3_bucket_versioning" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  versioning_configuration {
    status = "Enabled"
  }
}

# Encrypt state at rest
resource "aws_s3_bucket_server_side_encryption_configuration" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

# Block public access (state files contain secrets!)
resource "aws_s3_bucket_public_access_block" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# DynamoDB table for state locking
resource "aws_dynamodb_table" "terraform_lock" {
  name         = "terraform-lock-${var.environment}"
  billing_mode = "PAY_PER_REQUEST"  # Cost: ~$0.01/month
  hash_key     = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }

  tags = {
    Name        = "Terraform Lock ${title(var.environment)}"
    Environment = var.environment
  }
}

output "s3_bucket_name" {
  value = aws_s3_bucket.terraform_state.id
}

output "dynamodb_table_name" {
  value = aws_dynamodb_table.terraform_lock.name
}
```

**Deploy state infrastructure:**

```bash
# Create state infrastructure for each environment
cd terraform/bootstrap

# Dev
terraform apply -var="environment=dev"

# Staging
terraform apply -var="environment=staging"

# Production
terraform apply -var="environment=production"

# You'll now have 3 S3 buckets and 3 DynamoDB tables
```

**Step 2: Configure backend in each environment**

```hcl
# terraform/environments/dev/backend.tf
terraform {
  backend "s3" {
    bucket         = "mycompany-terraform-state-dev"
    key            = "infrastructure/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "terraform-lock-dev"
  }
}
```

```hcl
# terraform/environments/production/backend.tf
terraform {
  backend "s3" {
    bucket         = "mycompany-terraform-state-production"
    key            = "infrastructure/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "terraform-lock-production"
  }
}
```

**Benefits:**

- ✅ Complete isolation (dev state cannot touch prod)
- ✅ State locking prevents concurrent applies
- ✅ Versioning allows rollback
- ✅ Encryption protects sensitive data
- ✅ Each environment costs ~$0.01/month for DynamoDB

### **1.5: IMPORTANT - Terraform Destroy & Cost Management**

**⚠️ CRITICAL: Always Clean Up to Avoid Charges!**

Terraform-managed AWS resources cost money even when idle. **Always destroy** when not actively using:

**Daily Workflow:**

```bash
# Morning: Start working
cd terraform/environments/dev
terraform apply

# Work on your project...

# Evening: Clean up to avoid charges
terraform destroy -auto-approve

# Cost savings: ~$50-100/month by destroying dev overnight
```

**Automated Cleanup (Advanced):**

```yaml
# .github/workflows/nightly-cleanup.yml
name: Nightly Cleanup

on:
  schedule:
    - cron: "0 2 * * *" # 2 AM daily
  workflow_dispatch: # Manual trigger

jobs:
  destroy-dev:
    runs-on: ubuntu-latest
    permissions:
      id-token: write
      contents: read
    steps:
      - uses: actions/checkout@v3

      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: ${{ secrets.AWS_ROLE_ARN_DEV }}
          aws-region: us-east-1

      - name: Destroy dev environment
        working-directory: terraform/environments/dev
        run: |
          terraform init
          terraform destroy -auto-approve

      - name: Notify team
        run: |
          echo "Dev environment destroyed to save costs"
          # Could send Slack notification here
```

**What to Destroy:**

```bash
# Order matters! Destroy in reverse order of creation

# 1. Application resources (fast, cheap to recreate)
cd terraform/environments/dev
terraform destroy -target=module.app -auto-approve

# 2. EKS cluster (takes 10-15 minutes to recreate)
terraform destroy -target=module.eks -auto-approve

# 3. RDS database (CAREFUL: Contains data!)
# Only destroy if you have backups or using test data
terraform destroy -target=module.rds -auto-approve

# 4. VPC and networking (fast to recreate)
terraform destroy -auto-approve
```

**What NOT to Destroy:**

```bash
# DON'T destroy state infrastructure
# - S3 bucket (contains all your state history)
# - DynamoDB table (needed for locking)

# These cost ~$0.01/month, not worth destroying
```

**Cost Monitoring:**

```bash
# Check current AWS spend
aws ce get-cost-and-usage \
  --time-period Start=2026-04-01,End=2026-04-04 \
  --granularity DAILY \
  --metrics BlendedCost \
  --group-by Type=TAG,Key=Environment

# Expected costs:
# Dev (destroyed nightly): ~$20-30/month
# Dev (always running): ~$100-150/month
# Production: ~$800-1500/month (don't destroy this!)
```

**Production Safety:**

```hcl
# terraform/environments/production/main.tf
resource "aws_eks_cluster" "main" {
  # ... config ...

  lifecycle {
    prevent_destroy = true  # Terraform will refuse to destroy
  }
}
```

### **1.6: Preventing Production Accidents**

**CODEOWNERS file:**

```
# .github/CODEOWNERS

# Production requires 2 senior engineer approvals
/terraform/environments/production/ @senior-team @tech-leads

# Staging requires 1 platform engineer approval
/terraform/environments/staging/ @platform-team

# Dev has no restrictions
/terraform/environments/dev/ @all-engineers
```

**GitHub Branch Protection:**

```yaml
# Settings → Branches → main branch protection
- Require pull request reviews: 2 (for production changes)
- Require status checks: terraform-plan
- Require conversation resolution
- Require signed commits (optional but recommended)
```

**Deletion Protection in Code:**

```hcl
# environments/production/main.tf
resource "aws_db_instance" "main" {
  # ... other config ...

  deletion_protection = var.enable_deletion_protection  # true for prod

  lifecycle {
    prevent_destroy = true  # Terraform will refuse to destroy
  }
}
```

### **1.6: Cost Optimization Per Environment**

| Resource        | Dev          | Staging     | Production     | Savings           |
| --------------- | ------------ | ----------- | -------------- | ----------------- |
| **EKS Nodes**   | 2x t3.medium | 3x m5.large | 6x m5.2xlarge  |                   |
| **RDS**         | db.t3.micro  | db.t3.large | db.r6g.2xlarge |                   |
| **NAT Gateway** | 1 ($32/mo)   | 2 ($64/mo)  | 2 ($64/mo)     | Dev: -50%         |
| **Backups**     | 1 day        | 7 days      | 30 days        | Dev: -97% storage |
| **Monitoring**  | Basic        | Standard    | Detailed       | Dev: -60%         |
| **Multi-AZ**    | No           | Yes         | Yes            | Dev: -50% compute |
| **Total Est.**  | ~$100/mo     | ~$400/mo    | ~$1200/mo      | **Dev: -92%**     |

**Implementation:**

```hcl
# modules/eks/main.tf
resource "aws_eks_node_group" "main" {
  instance_types = var.environment == "dev" ? ["t3.medium"] :
                   var.environment == "staging" ? ["m5.large"] :
                   ["m5.2xlarge"]

  scaling_config {
    desired_size = var.environment == "dev" ? 2 :
                   var.environment == "staging" ? 3 : 6
    min_size     = var.environment == "dev" ? 1 : 2
    max_size     = var.environment == "dev" ? 3 : 10
  }
}
```

### **1.7: Implementation Steps for Phase 1**

**Step 1: Create Directory Structure**

```bash
cd terraform/
mkdir -p environments/{dev,staging,production}
```

**Step 2: Create Module Structure**

```bash
# Your modules/ already exist from current project
# Just reference them from each environment
```

**Step 3: Create Dev Environment**

```bash
cd environments/dev/

# Create main.tf that calls modules
# Create variables.tf with input variables
# Create terraform.tfvars with dev values
# Create backend.tf with state config
# Create outputs.tf with useful info
```

**Step 4: Initialize and Apply Dev**

```bash
terraform init    # Downloads providers, configures backend
terraform plan    # Preview changes
terraform apply   # Create dev infrastructure
```

**Step 5: Repeat for Staging and Production**

```bash
# Copy structure, adjust values, different backends
```

**Step 6: Test Isolation**

```bash
# In dev directory
terraform destroy  # Should only affect dev

# In production directory
terraform plan     # Should show production is untouched
```

---

## ☸️ Phase 2: EKS Cluster with ECR

### **Learning Outcomes**

After this phase, you can answer:

- "How do you provision a production-grade EKS cluster with Terraform?"
- "Explain your ECR repository strategy and image lifecycle policies."
- "How do you configure EKS authentication and RBAC?"
- "Walk me through your EKS security hardening approach."
- "How do you manage EKS cluster upgrades with zero downtime?"

### **2.1: EKS Module Architecture**

```hcl
# terraform/modules/eks/main.tf
resource "aws_eks_cluster" "main" {
  name     = "${var.project_name}-${var.environment}"
  role_arn = aws_iam_role.cluster.arn
  version  = var.kubernetes_version  # 1.28, 1.29, etc.

  vpc_config {
    subnet_ids              = var.private_subnet_ids
    endpoint_private_access = true
    endpoint_public_access  = var.environment == "production" ? false : true

    # Production: Only allow access from office/VPN
    public_access_cidrs = var.environment == "production" ? var.allowed_cidrs : ["0.0.0.0/0"]
  }

  # Encryption at rest for secrets
  encryption_config {
    provider {
      key_arn = aws_kms_key.eks.arn
    }
    resources = ["secrets"]
  }

  # Enable control plane logging
  enabled_cluster_log_types = [
    "api",
    "audit",
    "authenticator",
    "controllerManager",
    "scheduler"
  ]

  depends_on = [
    aws_iam_role_policy_attachment.cluster_policy,
    aws_cloudwatch_log_group.eks
  ]

  tags = merge(var.tags, {
    Name = "${var.project_name}-eks-${var.environment}"
  })
}

# EKS Node Group (worker nodes)
resource "aws_eks_node_group" "main" {
  cluster_name    = aws_eks_cluster.main.name
  node_group_name = "${var.project_name}-node-group"
  node_role_arn   = aws_iam_role.node.arn
  subnet_ids      = var.private_subnet_ids

  # Instance types based on environment
  instance_types = var.environment == "dev" ? ["t3.medium"] :
                   var.environment == "staging" ? ["m5.large"] :
                   ["m5.2xlarge"]

  # Scaling configuration
  scaling_config {
    desired_size = var.environment == "dev" ? 2 :
                   var.environment == "staging" ? 3 : 6
    min_size     = var.environment == "dev" ? 1 : 2
    max_size     = var.environment == "dev" ? 3 : 10
  }

  # Use latest AL2 EKS-optimized AMI
  ami_type = "AL2_x86_64"

  # Update strategy
  update_config {
    max_unavailable_percentage = 33  # Rolling updates
  }

  # Apply labels for workload scheduling
  labels = {
    Environment = var.environment
    ManagedBy   = "terraform"
  }

  # Apply taints for production (optional)
  dynamic "taint" {
    for_each = var.environment == "production" ? [1] : []
    content {
      key    = "production"
      value  = "true"
      effect = "NO_SCHEDULE"  # Require toleration
    }
  }

  tags = merge(var.tags, {
    Name = "${var.project_name}-eks-nodes-${var.environment}"
  })

  depends_on = [
    aws_iam_role_policy_attachment.node_policy,
    aws_iam_role_policy_attachment.cni_policy,
    aws_iam_role_policy_attachment.registry_policy
  ]
}

# Cluster Autoscaler setup
resource "aws_iam_policy" "cluster_autoscaler" {
  name = "${var.project_name}-cluster-autoscaler-${var.environment}"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "autoscaling:DescribeAutoScalingGroups",
          "autoscaling:DescribeAutoScalingInstances",
          "autoscaling:DescribeLaunchConfigurations",
          "autoscaling:DescribeScalingActivities",
          "autoscaling:DescribeTags",
          "ec2:DescribeInstanceTypes",
          "ec2:DescribeLaunchTemplateVersions"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "autoscaling:SetDesiredCapacity",
          "autoscaling:TerminateInstanceInAutoScalingGroup"
        ]
        Resource = "*"
        Condition = {
          StringEquals = {
            "autoscaling:ResourceTag/k8s.io/cluster-autoscaler/${aws_eks_cluster.main.name}" = "owned"
          }
        }
      }
    ]
  })
}
```

### **2.2: ECR (Elastic Container Registry)**

**Why ECR?**

- ✅ Integrated with IAM (no Docker Hub credentials)
- ✅ Encrypted at rest
- ✅ Image scanning (CVE detection)
- ✅ Lifecycle policies (auto-delete old images)
- ✅ Cross-region replication
- ✅ Private by default

```hcl
# terraform/modules/ecr/main.tf
resource "aws_ecr_repository" "app" {
  for_each = toset(var.service_names)  # ["javascript-api", "java-service", "rust-processor"]

  name                 = "${var.project_name}/${each.value}"
  image_tag_mutability = "MUTABLE"  # Allow overwriting tags like "latest"

  # Encryption
  encryption_configuration {
    encryption_type = "AES256"
  }

  # Image scanning on push
  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Name        = each.value
    Environment = var.environment
  }
}

# Lifecycle policy: Delete untagged images > 7 days old
resource "aws_ecr_lifecycle_policy" "app" {
  for_each   = aws_ecr_repository.app
  repository = each.value.name

  policy = jsonencode({
    rules = [
      {
        rulePriority = 1
        description  = "Remove untagged images older than 7 days"
        selection = {
          tagStatus   = "untagged"
          countType   = "sinceImagePushed"
          countUnit   = "days"
          countNumber = 7
        }
        action = {
          type = "expire"
        }
      },
      {
        rulePriority = 2
        description  = "Keep only last 10 images"
        selection = {
          tagStatus     = "any"
          countType     = "imageCountMoreThan"
          countNumber   = 10
        }
        action = {
          type = "expire"
        }
      }
    ]
  })
}

# Output repository URLs
output "repository_urls" {
  value = {
    for k, v in aws_ecr_repository.app :
    k => v.repository_url
  }
}
```

**Push image to ECR from GitHub Actions:**

```yaml
- name: Login to Amazon ECR
  uses: aws-actions/amazon-ecr-login@v2

- name: Build and push Docker image
  env:
    ECR_REGISTRY: ${{ steps.login-ecr.outputs.registry }}
    ECR_REPOSITORY: myproject/javascript-api
    IMAGE_TAG: ${{ github.sha }}
  run: |
    docker build -t $ECR_REGISTRY/$ECR_REPOSITORY:$IMAGE_TAG .
    docker push $ECR_REGISTRY/$ECR_REPOSITORY:$IMAGE_TAG
    docker tag $ECR_REGISTRY/$ECR_REPOSITORY:$IMAGE_TAG $ECR_REGISTRY/$ECR_REPOSITORY:latest
    docker push $ECR_REGISTRY/$ECR_REPOSITORY:latest
```

### **2.3: EKS Authentication & kubectl Setup**

**Configure kubectl to talk to EKS:**

```bash
# Update kubeconfig
aws eks update-kubeconfig \
  --region us-east-1 \
  --name myproject-dev \
  --alias dev

# Verify connection
kubectl get nodes
kubectl get pods --all-namespaces

# Output:
# NAME                          STATUS   ROLES    AGE   VERSION
# ip-10-0-1-123.ec2.internal    Ready    <none>   5m    v1.28.5-eks-abcdef
# ip-10-0-2-456.ec2.internal    Ready    <none>   5m    v1.28.5-eks-abcdef
```

**RBAC Configuration:**

```yaml
# k8s/rbac/developer-role.yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: developer
  namespace: dev
rules:
  - apiGroups: ["", "apps", "batch"]
    resources: ["pods", "deployments", "jobs", "services", "configmaps"]
    verbs: ["get", "list", "create", "update", "delete"]
  - apiGroups: [""]
    resources: ["pods/log"]
    verbs: ["get"]

---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: developer-binding
  namespace: dev
subjects:
  - kind: User
    name: developer@company.com
    apiGroup: rbac.authorization.k8s.io
roleRef:
  kind: Role
  name: developer
  apiGroup: rbac.authorization.k8s.io
```

### **2.4: Cost Optimization for EKS**

| Component                  | Dev          | Staging      | Production   | Notes                  |
| -------------------------- | ------------ | ------------ | ------------ | ---------------------- |
| **EKS Control Plane**      | $73/mo       | $73/mo       | $73/mo       | Fixed cost per cluster |
| **EC2 Nodes (t3.medium)**  | ~$30/mo      | -            | -            | 2 nodes, on-demand     |
| **EC2 Nodes (m5.large)**   | -            | ~$150/mo     | -            | 3 nodes, on-demand     |
| **EC2 Nodes (m5.2xlarge)** | -            | -            | ~$500/mo     | 6 nodes, savings plan  |
| **ALB**                    | ~$20/mo      | ~$20/mo      | ~$40/mo      | Load balancer          |
| **NAT Gateway**            | ~$32/mo      | ~$64/mo      | ~$64/mo      | Single vs multi-AZ     |
| **ECR Storage**            | <$1/mo       | <$5/mo       | ~$10/mo      | Depends on image count |
| **Total**                  | **~$155/mo** | **~$312/mo** | **~$687/mo** |                        |

**Savings Strategies:**

```bash
# 1. Use Spot Instances for non-production (70% cheaper)
terraform apply -var="use_spot_instances=true"

# 2. Destroy dev at night (50% savings)
terraform destroy  # Before logging off

# 3. Savings Plans for production (40% discount)
# AWS Console → Savings Plans → 1-year no-upfront

# 4. Fargate for intermittent workloads
# Pay only when pods run, no idle node costs
```

---

## 🎯 Phase 3: Advanced Kubernetes & Helm Mastery

### **Learning Outcomes**

After this phase, you can answer:

- "How do you troubleshoot a CrashLoopBackOff at a senior level?"
- "Explain Horizontal Pod Autoscaling with custom metrics."
- "How do you handle stateful applications in Kubernetes?"
- "Walk me through your Helm chart best practices."
- "How do you debug networking issues between pods?"
- "Explain pod disruption budgets and when you'd use them."

### **3.1: Advanced Helm Chart Structure**

```
helm/microservices/
├── Chart.yaml
├── values.yaml              # Default values
├── values-dev.yaml          # Dev overrides
├── values-staging.yaml
├── values-production.yaml
├── charts/                  # Subcharts (optional)
├── templates/
│   ├── _helpers.tpl         # Template functions
│   ├── NOTES.txt            # Post-install instructions
│   ├── deployment.yaml
│   ├── service.yaml
│   ├── ingress.yaml
│   ├── hpa.yaml             # Horizontal Pod Autoscaler
│   ├── pdb.yaml             # Pod Disruption Budget
│   ├── configmap.yaml
│   ├── secret.yaml
│   ├── serviceaccount.yaml
│   ├── networkpolicy.yaml   # Network isolation
│   └── tests/
│       └── test-connection.yaml
└── README.md
```

**Production-Grade Helm Deployment:**

```yaml
# helm/microservices/templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "microservices.fullname". }}
  labels:
    {{- include "microservices.labels" . | nindent 4 }}
spec:
  {{- if not .Values.autoscaling.enabled }}
  replicas: {{ .Values.replicaCount }}
  {{- end }}

  # Rolling update strategy
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0  # Zero downtime

  selector:
    matchLabels:
      {{- include "microservices.selectorLabels" . | nindent 6 }}

  template:
    metadata:
      annotations:
        # Force pod restart when ConfigMap changes
        checksum/config: {{ include (print $.Template.BasePath "/configmap.yaml") . | sha256sum }}
        # Prometheus scraping
        prometheus.io/scrape: "true"
        prometheus.io/port: "{{ .Values.service.port }}"
        prometheus.io/path: "/metrics"
      labels:
        {{- include "microservices.selectorLabels" . | nindent 8 }}

    spec:
      # Security: Don't run as root
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000

      # Service account for IRSA (IAM Roles for Service Accounts)
      serviceAccountName: {{ include "microservices.serviceAccountName" . }}

      # Pod anti-affinity (don't schedule on same node)
      {{- if eq .Values.environment "production" }}
      affinity:
        podAntiAffinity:
          requiredDuringScheduling IgnoredDuringExecution:
            - labelSelector:
                matchExpressions:
                  - key: app
                    operator: In
                    values:
                      - {{ include "microservices.name" . }}
              topologyKey: kubernetes.io/hostname
      {{- end }}

      # Init containers (database migrations, etc.)
      initContainers:
        - name: migration
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
          command: ["npm", "run", "migrate"]
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: {{ include "microservices.fullname" . }}
                  key: database-url

      containers:
        - name: {{ .Chart.Name }}
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
          imagePullPolicy: {{ .Values.image.pullPolicy }}

          ports:
            - name: http
              containerPort: {{ .Values.service.targetPort }}
              protocol: TCP

          # Liveness probe (restart if unhealthy)
          livenessProbe:
            httpGet:
              path: /health
              port: http
            initialDelaySeconds: 30
            periodSeconds: 10
            timeoutSeconds: 5
            failureThreshold: 3

          # Readiness probe (don't send traffic if not ready)
          readinessProbe:
            httpGet:
              path: /ready
              port: http
            initialDelaySeconds: 5
            periodSeconds: 5
            timeoutSeconds: 3
            failureThreshold: 3

          # Startup probe (for slow-starting apps)
          startupProbe:
            httpGet:
              path: /health
              port: http
            initialDelaySeconds: 0
            periodSeconds: 10
            timeoutSeconds: 3
            failureThreshold: 30  # Allow 5 minutes to start

          # Resource limits (CRITICAL for production)
          resources:
            requests:
              memory: {{ .Values.resources.requests.memory }}
              cpu: {{ .Values.resources.requests.cpu }}
            limits:
              memory: {{ .Values.resources.limits.memory }}
              cpu: {{ .Values.resources.limits.cpu }}

          # Environment variables
          env:
            - name: NODE_ENV
              value: {{ .Values.environment }}
            - name: PORT
              value: "{{ .Values.service.targetPort }}"
            - name: POD_NAME
              valueFrom:
                fieldRef:
                  fieldPath: metadata.name
            - name: POD_NAMESPACE
              valueFrom:
                fieldRef:
                  fieldPath: metadata.namespace
            - name: POD_IP
              valueFrom:
                fieldRef:
                  fieldPath: status.podIP

          # Mount ConfigMaps and Secrets
          volumeMounts:
            - name: config
              mountPath: /app/config
              readOnly: true
            - name: secrets
              mountPath: /app/secrets
              readOnly: true

      volumes:
        - name: config
          configMap:
            name: {{ include "microservices.fullname" . }}
        - name: secrets
          secret:
            secretName: {{ include "microservices.fullname" . }}
```

### **3.2: Horizontal Pod Autoscaling (HPA)**

```yaml
# helm/microservices/templates/hpa.yaml
{{- if .Values.autoscaling.enabled }}
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: {{ include "microservices.fullname" . }}
  labels:
    {{- include "microservices.labels" . | nindent 4 }}
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: {{ include "microservices.fullname" . }}

  minReplicas: {{ .Values.autoscaling.minReplicas }}
  maxReplicas: {{ .Values.autoscaling.maxReplicas }}

  metrics:
    # Scale based on CPU
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: {{ .Values.autoscaling.targetCPUUtilizationPercentage }}

    # Scale based on memory
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: {{ .Values.autoscaling.targetMemoryUtilizationPercentage }}

    # Custom metric: HTTP requests per second
    {{- if .Values.autoscaling.customMetrics.enabled }}
    - type: Pods
      pods:
        metric:
          name: http_requests_per_second
        target:
          type: AverageValue
          averageValue: "1000"  # Scale up above 1000 req/s
    {{- end }}

  # Scale down slowly to avoid flapping
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300  # Wait 5 min before scaling down
      policies:
        - type: Percent
          value: 50  # Scale down max 50% at a time
          periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0  # Scale up immediately
      policies:
        - type: Percent
          value: 100  # Can double capacity
          periodSeconds: 15
        - type: Pods
          value: 4  # Or add 4 pods
          periodSeconds: 15
      selectPolicy: Max  # Use whichever adds more pods
{{- end }}
```

**values.yaml configuration:**

```yaml
autoscaling:
  enabled: true
  minReplicas: 2
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80
  customMetrics:
    enabled: true
```

### **3.3: Pod Disruption Budgets (PDB)**

**Prevents too many pods from being evicted during node maintenance:**

```yaml
# helm/microservices/templates/pdb.yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: {{include "microservices.fullname" . }}
  labels:
    {{- include "microservices.labels" . | nindent 4 }}
spec:
  {{- if eq .Values.environment "production" }}
  minAvailable: 2  # Always keep 2 pods running
  {{- else }}
  maxUnavailable: 1  # Allow 1 pod to be down
  {{- end }}

  selector:
    matchLabels:
      {{- include "microservices.selectorLabels" . | nindent 6 }}
```

**Why This Matters:**

```bash
# Without PDB: Node drain kills all pods
kubectl drain node-1  # All 3 pods terminated → Service down!

# With PDB: Kubernetes respects minAvailable
kubectl drain node-1  # Waits for pods to reschedule before draining
# Result: Zero downtime during maintenance
```

### **3.4: StatefulSets for Databases**

**When you need stable network identity and persistent storage:**

```yaml
# helm/microservices/templates/statefulset.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: {{ include "microservices.fullname" . }}-postgres
spec:
  serviceName: "postgres"
  replicas: {{ .Values.postgres.replicas }}

  selector:
    matchLabels:
      app: postgres

  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
        - name: postgres
          image: postgres:15-alpine
          ports:
            - containerPort: 5432
              name: postgres

          volumeMounts:
            - name: data
              mountPath: /var/lib/postgresql/data

          env:
            - name: POSTGRES_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: postgres-secret
                  key: password
            - name: PGDATA
              value: /var/lib/postgresql/data/pgdata

  # Persistent volume claim per pod
  volumeClaimTemplates:
    - metadata:
        name: data
      spec:
        accessModes: ["ReadWriteOnce"]
        storageClassName: gp3  # AWS EBS gp3
        resources:
          requests:
            storage: {{ .Values.postgres.storage }}
```

**Key Differences from Deployment:**

- ✅ Stable pod names: postgres-0, postgres-1, postgres-2
- ✅ Ordered deployment/scaling
- ✅ Persistent volume per pod
- ✅ Stable DNS: postgres-0.postgres.default.svc.cluster.local

### **3.5: Network Policies (Zero Trust)**

```yaml
# helm/microservices/templates/networkpolicy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: { { include "microservices.fullname" . } }
  namespace: { { .Values.namespace } }
spec:
  podSelector:
    matchLabels:
      app: { { include "microservices.name" . } }

  policyTypes:
    - Ingress
    - Egress

  # Ingress: Who can talk TO this service
  ingress:
    # Allow traffic from ingress controller
    - from:
        - namespaceSelector:
            matchLabels:
              name: ingress-nginx
        - podSelector:
            matchLabels:
              app: ingress-nginx
      ports:
        - protocol: TCP
          port: 3000

    # Allow traffic from other microservices
    - from:
        - podSelector:
            matchLabels:
              app: java-service
      ports:
        - protocol: TCP
          port: 3000

  # Egress: Where THIS service can talk TO
  egress:
    # Allow DNS
    - to:
        - namespaceSelector:
            matchLabels:
              name: kube-system
      ports:
        - protocol: UDP
          port: 53

    # Allow database
    - to:
        - podSelector:
            matchLabels:
              app: postgres
      ports:
        - protocol: TCP
          port: 5432

    # Allow external HTTPS
    - to:
        - namespaceSelector: {}
      ports:
        - protocol: TCP
          port: 443
```

**Test network policy:**

```bash
# Should work (allowed)
kubectl exec -it javascript-api-pod -- curl http://postgres:5432

# Should fail (blocked)
kubectl exec -it javascript-api-pod -- curl http://redis:6379
# Output: Connection timed out
```

---

## 🔧 Phase 4: Multi-Language CI/CD Pipeline

### **Learning Outcomes**

After this phase, you can answer:

- "How do you handle CI/CD for polyglot microservices?"
- "Explain your build caching strategy."
- "How do you optimize Docker images per language?"
- "Walk me through your matrix build approach."

### **2.1: Project Structure**

```
polyglot-microservices/
├── services/
│   ├── javascript-api/
│   │   ├── src/
│   │   │   ├── index.js
│   │   │   ├── routes/
│   │   │   └── controllers/
│   │   ├── tests/
│   │   ├── package.json
│   │   ├── package-lock.json
│   │   ├── Dockerfile
│   │   ├── .dockerignore
│   │   └── README.md
│   │
│   ├── java-service/
│   │   ├── src/
│   │   │   ├── main/java/com/company/service/
│   │   │   └── test/java/com/company/service/
│   │   ├── pom.xml
│   │   ├── Dockerfile
│   │   ├── .dockerignore
│   │   └── README.md
│   │
│   └── rust-processor/
│       ├── src/
│       │   ├── main.rs
│       │   └── lib.rs
│       ├── tests/
│       ├── Cargo.toml
│       ├── Cargo.lock
│       ├── Dockerfile
│       ├── .dockerignore
│       └── README.md
│
├── .github/
│   └── workflows/
│       ├── javascript-ci.yml
│       ├── java-ci.yml
│       ├── rust-ci.yml
│       └── deploy.yml
│
├── helm/
│   └── microservices/
│       ├── Chart.yaml
│       ├── values.yaml
│       ├── values-dev.yaml
│       ├── values-staging.yaml
│       ├── values-production.yaml
│       └── templates/
│           ├── javascript-deployment.yaml
│           ├── java-deployment.yaml
│           └── rust-deployment.yaml
│
└── README.md
```

### **2.2: Language-Specific Optimizations**

#### **JavaScript/Node.js Best Practices**

**Optimized Dockerfile:**

```dockerfile
# Multi-stage build for Node.js
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files first (layer caching)
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY . .

# Production image
FROM node:20-alpine

WORKDIR /app

# Copy only production dependencies and code
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist
COPY package.json ./

# Security: non-root user
USER node

EXPOSE 3000

CMD ["node", "dist/index.js"]
```

**Why This Works:**

- ✅ `npm ci` is faster than `npm install` (uses lock file)
- ✅ Layer caching: package.json rarely changes
- ✅ Multi-stage reduces image size (no build tools in final image)
- ✅ Alpine Linux: 5MB vs 900MB for full node image

**GitHub Actions Workflow:**

```yaml
name: JavaScript CI

on:
  push:
    paths:
      - "services/javascript-api/**"

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: "20"
          cache: "npm" # ← Caches node_modules
          cache-dependency-path: services/javascript-api/package-lock.json

      - name: Install dependencies
        working-directory: services/javascript-api
        run: npm ci

      - name: Run tests
        run: npm test

      - name: Build
        run: npm run build

      - name: Build Docker image
        run: docker build -t js-api:${{ github.sha }} .
```

**Build Time:**

- First run: ~2 minutes
- Subsequent runs with cache: ~30 seconds

---

#### **Java/Spring Boot Best Practices**

**Optimized Dockerfile:**

```dockerfile
# Multi-stage build for Java
FROM maven:3.9-eclipse-temurin-17 AS builder

WORKDIR /app

# Copy pom.xml first (dependency caching)
COPY pom.xml .

# Download dependencies (cached layer)
RUN mvn dependency:go-offline

# Copy source and build
COPY src ./src
RUN mvn package -DskipTests

# Production image
FROM eclipse-temurin:17-jre-alpine

WORKDIR /app

# Copy only the JAR
COPY --from=builder /app/target/*.jar app.jar

# Security
USER nobody

EXPOSE 8080

ENTRYPOINT ["java", "-jar", "app.jar"]
```

**Why This Works:**

- ✅ JRE image (not JDK): 200MB vs 400MB
- ✅ Maven dependency cache: Most builds don't change dependencies
- ✅ `-DskipTests`: Tests run in CI, not in Docker build
- ✅ Single JAR deployment (Spring Boot embedded server)

**GitHub Actions Workflow:**

```yaml
name: Java CI

on:
  push:
    paths:
      - "services/java-service/**"

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Java
        uses: actions/setup-java@v3
        with:
          distribution: "temurin"
          java-version: "17"
          cache: "maven" # ← Caches .m2 directory

      - name: Build with Maven
        working-directory: services/java-service
        run: mvn clean verify

      - name: Run security scan
        run: mvn dependency-check:check

      - name: Build Docker image
        run: docker build -t java-service:${{ github.sha }} .
```

**Build Time:**

- First run: ~4 minutes (Maven downloads everything)
- Subsequent runs with cache: ~1 minute

---

#### **Rust Best Practices**

**Optimized Dockerfile:**

```dockerfile
# cargo-chef for dependency caching
FROM rust:1.75 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Compute recipe file
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies (cached layer)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release

# Production image
FROM debian:bookworm-slim

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/rust-processor .

# Security
USER nobody

CMD ["./rust-processor"]
```

**Why This Works:**

- ✅ `cargo-chef`: Caches Rust dependencies (game changer!)
- ✅ Release build: Optimized, no debug symbols
- ✅ Slim image: 70MB vs 1.5GB with build tools
- ✅ Fast subsequent builds: Dependencies rarely change

**GitHub Actions Workflow:**

```yaml
name: Rust CI

on:
  push:
    paths:
      - "services/rust-processor/**"

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Cache Cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/
            ~/.cargo/git/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        working-directory: services/rust-processor
        run: cargo test

      - name: Build release
        run: cargo build --release

      - name: Run Clippy (linting)
        run: cargo clippy -- -D warnings

      - name: Build Docker image
        run: docker build -t rust-processor:${{ github.sha }} .
```

**Build Time:**

- First run: ~3 minutes (Cargo downloads crates)
- Subsequent runs with cache: ~30 seconds

---

### **2.3: Matrix Build Strategy**

**The Problem:**
If you build services sequentially, you wait for each:

```
JavaScript (2 min) → Java (4 min) → Rust (3 min) = 9 minutes total
```

**The Solution: Matrix Builds (Parallel)**

```yaml
name: Multi-Language CI

on:
  push:
    branches: [main, develop]

jobs:
  # Detect which services changed
  changes:
    runs-on: ubuntu-latest
    outputs:
      javascript: ${{ steps.filter.outputs.javascript }}
      java: ${{ steps.filter.outputs.java }}
      rust: ${{ steps.filter.outputs.rust }}
    steps:
      - uses: actions/checkout@v3
      - uses: dorny/paths-filter@v2
        id: filter
        with:
          filters: |
            javascript:
              - 'services/javascript-api/**'
            java:
              - 'services/java-service/**'
            rust:
              - 'services/rust-processor/**'

  # Build all services in parallel
  build:
    needs: changes
    runs-on: ubuntu-latest
    strategy:
      matrix:
        service:
          - name: javascript-api
            path: services/javascript-api
            dockerfile: services/javascript-api/Dockerfile
            enabled: ${{ needs.changes.outputs.javascript }}
          - name: java-service
            path: services/java-service
            dockerfile: services/java-service/Dockerfile
            enabled: ${{ needs.changes.outputs.java }}
          - name: rust-processor
            path: services/rust-processor
            dockerfile: services/rust-processor/Dockerfile
            enabled: ${{ needs.changes.outputs.rust }}

    # Skip if service didn't change
    if: matrix.service.enabled == 'true'

    steps:
      - uses: actions/checkout@v3

      - name: Build ${{ matrix.service.name }}
        run: |
          docker build -t ${{ matrix.service.name }}:${{ github.sha }} \
            -f ${{ matrix.service.dockerfile }} \
            ${{ matrix.service.path }}

      - name: Run security scan
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: ${{ matrix.service.name }}:${{ github.sha }}
          severity: CRITICAL,HIGH

      - name: Push to registry
        run: |
          docker tag ${{ matrix.service.name }}:${{ github.sha }} \
            myregistry/${{ matrix.service.name }}:${{ github.sha }}
          docker push myregistry/${{ matrix.service.name }}:${{ github.sha }}
```

**Result:**

- ✅ All 3 services build in parallel
- ✅ Total time: ~4 minutes (longest service)
- ✅ Only changed services build
- ✅ 55% faster than sequential

---

### **2.4: Language-Specific Security Scanning**

| Language       | Tool                   | What It Catches                           |
| -------------- | ---------------------- | ----------------------------------------- |
| **JavaScript** | npm audit, Snyk        | Vulnerable dependencies (lodash, express) |
| **JavaScript** | Semgrep                | Code vulnerabilities (SQL injection, XSS) |
| **Java**       | OWASP Dependency-Check | CVEs in Maven dependencies                |
| **Java**       | SpotBugs               | Bug patterns, security issues             |
| **Rust**       | cargo audit            | Vulnerable crates                         |
| **Rust**       | cargo clippy           | Code quality, potential bugs              |
| **All**        | Trivy                  | Container image vulnerabilities           |
| **All**        | gitleaks               | Accidentally committed secrets            |

**Implementation:**

```yaml
jobs:
  security-scan:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        service: [javascript-api, java-service, rust-processor]
    steps:
      - uses: actions/checkout@v3

      # JavaScript-specific
      - name: npm audit (JS)
        if: matrix.service == 'javascript-api'
        working-directory: services/${{ matrix.service }}
        run: npm audit --audit-level=high

      # Java-specific
      - name: OWASP Dependency Check (Java)
        if: matrix.service == 'java-service'
        working-directory: services/${{ matrix.service }}
        run: mvn dependency-check:check

      # Rust-specific
      - name: Cargo Audit (Rust)
        if: matrix.service == 'rust-processor'
        working-directory: services/${{ matrix.service }}
        run: cargo audit

      # Common scans
      - name: Run Trivy (All)
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: ${{ matrix.service }}:${{ github.sha }}
```

---

### **2.5: Deployment Orchestration**

**Challenge:** You have 3 services that need to deploy together.

**Solution: Helm Chart with Multiple Deployments**

**helm/microservices/values.yaml:**

```yaml
# Global values
global:
  imageRegistry: myregistry.io
  imageTag: latest
  environment: production

# JavaScript API
javascriptApi:
  enabled: true
  replicaCount: 3
  image:
    repository: javascript-api
    tag: abc123f
  resources:
    requests:
      memory: "256Mi"
      cpu: "250m"
    limits:
      memory: "512Mi"
      cpu: "500m"

# Java Service
javaService:
  enabled: true
  replicaCount: 2
  image:
    repository: java-service
    tag: def456g
  resources:
    requests:
      memory: "512Mi"
      cpu: "500m"
    limits:
      memory: "1Gi"
      cpu: "1000m"

# Rust Processor
rustProcessor:
  enabled: true
  replicaCount: 2
  image:
    repository: rust-processor
    tag: ghi789h
  resources:
    requests:
      memory: "128Mi"
      cpu: "100m"
    limits:
      memory: "256Mi"
      cpu: "200m"
```

**Single Helm Command Deploys All:**

```bash
helm upgrade --install microservices ./helm/microservices \
  --values helm/microservices/values-production.yaml \
  --set javascriptApi.image.tag=$JS_TAG \
  --set javaService.image.tag=$JAVA_TAG \
  --set rustProcessor.image.tag=$RUST_TAG
```

---

### **2.6: Implementation Steps for Phase 2**

**Step 1: Create Service Structure**

```bash
mkdir -p services/{javascript-api,java-service,rust-processor}
```

**Step 2: Build Each Service**

```bash
# JavaScript
cd services/javascript-api/
npm init -y
npm install express

# Java
cd services/java-service/
spring init --dependencies=web

# Rust
cd services/rust-processor/
cargo new rust-processor
```

**Step 3: Create Dockerfiles**

```bash
# Use the optimized Dockerfiles shown above for each language
```

**Step 4: Create GitHub Actions Workflows**

```bash
mkdir -p .github/workflows/
# Create the matrix build workflow
```

**Step 5: Create Helm Chart**

```bash
helm create microservices
# Customize with multiple deployments
```

**Step 6: Test Locally**

```bash
docker build -t js-api:test ./services/javascript-api
docker build -t java-svc:test ./services/java-service
docker build -t rust-proc:test ./services/rust-processor
```

---

## �️ Phase 5: Environment-Specific Security Scanning

### **Learning Outcomes**

After this phase, you can answer:

- "How do you balance security with developer velocity?"
- "Explain your branch-aware security scanning strategy."
- "Why don't you run all security scans in every environment?"
- "How do you integrate security gates into CI/CD without slowing down development?"

### **5.1: The Problem with One-Size-Fits-All Scanning**

**Scenario:**

```yaml
# ❌ Bad: Same scans for all branches
jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - run: npm audit # 30 seconds
      - run: semgrep scan # 2 minutes
      - run: trivy scan # 3 minutes
      - run: tfsec . # 1 minute
      - run: gitleaks detect # 30 seconds
# Total: 7 minutes EVERY commit

# Problems:
# - Developers wait 7+ minutes for feature branch feedback
# - Slows down iteration
# - Developers bypass CI or batch commits
# - Security becomes "that slow thing we hate"
```

**Solution: Environment-Aware Scanning**

```
Feature Branch (develop):
  ├── Fast scans only (< 2 minutes total)
  ├── npm audit (critical vulns only)
  ├── gitleaks (secrets)
  └── Basic lint checks
  → GOAL: Fast feedback for developers

Staging Branch:
  ├── Comprehensive scans (5-7 minutes)
  ├── All SAST rules
  ├── Full dependency scanning
  ├── Container scanning
  └── IaC scanning
  → GOAL: Catch issues before production

Production Branch (main):
  ├── FULL security suite (10-15 minutes)
  ├── All scans in parallel
  ├── Manual security approval
  ├── Compliance checks
  └── Generate audit report
  → GOAL: Maximum security, deployment is infrequent
```

### **5.2: Implementation Pattern**

```yaml
# .github/workflows/security-scanning.yml
name: Security Scanning

on:
  workflow_call: # Reusable workflow
    inputs:
      scan-level:
        required: true
        type: string # 'fast', 'comprehensive', or 'full'

jobs:
  # Job 1: Detect environment from branch
  detect-environment:
    runs-on: ubuntu-latest
    outputs:
      scan-level: ${{ steps.determine.outputs.level }}
      run-fast: ${{ steps.determine.outputs.run-fast }}
      run-comprehensive: ${{ steps.determine.outputs.run-comprehensive }}
      run-full: ${{ steps.determine.outputs.run-full }}
    steps:
      - name: Determine scan level
        id: determine
        run: |
          if [[ "${{ github.ref }}" == "refs/heads/main"  ]]; then
            echo "level=full" >> $GITHUB_OUTPUT
            echo "run-fast=true" >> $GITHUB_OUTPUT
            echo "run-comprehensive=true" >> $GITHUB_OUTPUT
            echo "run-full=true" >> $GITHUB_OUTPUT
          elif [[ "${{ github.ref }}" == "refs/heads/staging" ]]; then
            echo "level=comprehensive" >> $GITHUB_OUTPUT
            echo "run-fast=true" >> $GITHUB_OUTPUT
            echo "run-comprehensive=true" >> $GITHUB_OUTPUT
            echo "run-full=false" >> $GITHUB_OUTPUT
          else
            echo "level=fast" >> $GITHUB_OUTPUT
            echo "run-fast=true" >> $GITHUB_OUTPUT
            echo "run-comprehensive=false" >> $GITHUB_OUTPUT
            echo "run-full=false" >> $GITHUB_OUTPUT
          fi

  # Job 2: Fast scans (ALWAYS run)
  fast-scans:
    needs: detect-environment
    if: needs.detect-environment.outputs.run-fast == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Secret Scanning (gitleaks)
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: npm audit (CRITICAL only)
        working-directory: services/javascript-api
        run: npm audit --audit-level=critical

      - name: Quick lint checks
        run: |
          npm run lint
          cargo clippy --all-targets -- -D warnings

  # Job 3: Comprehensive scans (staging + production)
  comprehensive-scans:
    needs: detect-environment
    if: needs.detect-environment.outputs.run-comprehensive == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: SAST with Semgrep
        uses: returntocorp/semgrep-action@v1
        with:
          config: p/security-audit p/owasp-top-ten

      - name: Full npm audit
        working-directory: services/javascript-api
        run: npm audit

      - name: OWASP Dependency Check (Java)
        working-directory: services/java-service
        run: mvn org.owasp:dependency-check-maven:check

      - name: cargo audit (Rust)
        working-directory: services/rust-processor
        run: cargo audit

      - name: IaC Scanning (Terraform)
        run: |
          tfsec terraform/ --format json --out tfsec-results.json
          checkov -d terraform/ --output json > checkov-results.json

  # Job 4: Full scans (production ONLY)
  full-scans:
    needs: detect-environment
    if: needs.detect-environment.outputs.run-full == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Container Scanning (Trivy)
        run: |
          docker build -t app:${{ github.sha }} .
          trivy image --severity HIGH,CRITICAL app:${{ github.sha }}

      - name: DAST (Dynamic scanning)
        run: |
          # Start app in test mode
          docker run -d -p 3000:3000 app:${{ github.sha }}
          # Run OWASP ZAP
          docker run -t owasp/zap2docker-stable zap-baseline.py \
            -t http://host.docker.internal:3000

      - name: Compliance Checks
        run: |
          # Check for required security headers
          # Verify encryption configs
          # Validate RBAC policies

      - name: Generate Security Report
        run: |
          # Aggregate all scan results
          # Generate PDF report for compliance
          # Upload to S3 for audit trail

  # Job 5: Security Summary
  security-summary:
    needs: [detect-environment, fast-scans, comprehensive-scans, full-scans]
    if: always()
    runs-on: ubuntu-latest
    steps:
      - name: Report Results
        run: |
          echo "Scan Level: ${{ needs.detect-environment.outputs.scan-level }}"
          echo "Fast Scans: ${{ needs.fast-scans.result }}"
          echo "Comprehensive: ${{ needs.comprehensive-scans.result }}"
          echo "Full Scans: ${{ needs.full-scans.result }}"

          if [[ "${{ needs.fast-scans.result }}" == "failure" ]]; then
            echo "❌ Critical security issues found!"
            exit 1
          fi
```

### **5.3: Real-World Timing**

| Scan Level        | Branch              | Duration      | Scans Included                             |
| ----------------- | ------------------- | ------------- | ------------------------------------------ |
| **Fast**          | develop, feature/\* | **1-2 min**   | gitleaks, npm audit (critical), linting    |
| **Comprehensive** | staging             | **5-7 min**   | + Semgrep, full dependency scan, IaC scan  |
| **Full**          | main (production)   | **10-15 min** | + Container scan, DAST, compliance, report |

**Impact on Developer Experience:**

```
Without environment-aware scanning:
  - Every feature commit: 15 minutes wait
  - Developers per day: 10 commits
  - Total CI time wasted: 150 minutes
  - Result: Developers batch commits, slower velocity

With environment-aware scanning:
  - Feature commits: 2 minutes
  - Staging: 7 minutes (less frequent)
  - Production: 15 minutes (infrequent)
  - Total daily CI: ~30 minutes
  - Result: Fast feedback, security when it matters
```

### **5.4: Inline Suppressions for Accepted Risks**

**When security tools flag intentional decisions:**

```javascript
// services/javascript-api/src/server.js

// nosemgrep: javascript.express.security.audit.express-check-csurf-middleware-usage.express-check-csurf-middleware-usage
// JUSTIFICATION: CSRF protection handled by API Gateway
// RISK: Medium - API is backend-only, no browser clients
// MITIGATION: API Gateway validates Origin headers
app.post("/api/data", (req, res) => {
  // ...
});
```

```hcl
# terraform/modules/vpc/main.tf

# nosemgrep: terraform.aws.security.aws-vpc-public-subnet.aws-vpc-public-subnet
# JUSTIFICATION: NAT Gateway requires public subnet with internet gateway
# RISK: Low - Only NAT Gateway resides in public subnet, strict security groups
# MITIGATION: Network ACLs restrict traffic, no EC2 instances allowed
resource "aws_subnet" "public" {
  map_public_ip_on_launch = true
  # ...
}
```

### **5.5: Security Gates with Manual Approval**

```yaml
# .github/workflows/deploy.yml
jobs:
  security-gate:
    uses: ./.github/workflows/security-scanning.yml

  deploy-staging:
    needs: security-gate
    if: github.ref == 'refs/heads/staging'
    # Automatic deployment after security passes

  deploy-production:
    needs: security-gate
    if: github.ref == 'refs/heads/main'
    environment:
      name: production
      url: https://app.company.com
    # Manual approval required (configured in GitHub Settings → Environments)
    steps:
      - name: Deploy after security approval
        run: |
          # Security team has reviewed scan results
          # Two approvals required for production
          kubectl apply -f k8s/production/
```

**GitHub Environment Protection Rules:**

1. Go to Settings → Environments → production
2. Check "Required reviewers"
3. Add security team members
4. Require 2 approvals
5. Result: Production deploys only after security review

---

## 🔄 Phase 6: Integration & GitOps

### **Learning Outcomes**

After this phase, you can answer:

- "How do you coordinate deployments across microservices?"
- "Explain your environment promotion strategy."
- "How do you handle rollbacks?"
- "Walk me through ArgoCD configuration."

### **3.1: GitOps with ArgoCD**

**ArgoCD Application for Dev:**

```yaml
# argocd/applications/microservices-dev.yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: microservices-dev
  namespace: argocd
spec:
  project: default

  source:
    repoURL: https://github.com/yourcompany/infrastructure
    targetRevision: develop
    path: helm/microservices
    helm:
      valueFiles:
        - values-dev.yaml

  destination:
    server: https://kubernetes.default.svc
    namespace: dev

  syncPolicy:
    automated:
      prune: true
      selfHeal: true
    syncOptions:
      - CreateNamespace=true
```

**What This Does:**

- 🔄 Watches `develop` branch
- 🔄 Syncs every 3 minutes
- 🔄 Auto-heals if someone makes manual changes
- 🔄 Auto-prunes deleted resources

### **3.2: Environment Promotion Workflow**

```
Feature Branch
    ↓ (PR + Tests)
Merge to develop → Deploy to DEV (automatic)
    ↓ (Smoke tests pass)
Merge to staging → Deploy to STAGING (1 approval)
    ↓ (Integration tests pass)
Merge to main → Deploy to PRODUCTION (2 approvals + manual gate)
```

**GitHub Actions Workflow:**

```yaml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment:
      name: production
      url: https://production.yourcompany.com
    steps:
      - uses: actions/checkout@v3

      - name: Wait for approval
        uses: trstringer/manual-approval@v1
        with:
          approvers: senior-team,tech-leads
          minimum-approvals: 2

      - name: Update Helm values
        run: |
          # Update image tags in values-production.yaml
          ...

      - name: Commit changes
        run: |
          git config user.name "GitHub Actions"
          git commit -am "Deploy to production: ${{ github.sha }}"
          git push

      - name: Wait for ArgoCD sync
        run: |
          # Poll ArgoCD API until sync completes
          ...

      - name: Run smoke tests
        run: |
          # Test critical paths
          ...
```

### **3.3: Rollback Strategy**

**Option 1: Git Revert (Recommended)**

```bash
# Revert the last deployment commit
git revert HEAD
git push

# ArgoCD detects change and automatically rolls back
```

**Option 2: ArgoCD Rollback**

```bash
# Rollback to previous version via ArgoCD
argocd app rollback microservices-production

# Or via UI: Click "History" → Select previous version → "Rollback"
```

**Option 3: Helm Rollback**

```bash
# If ArgoCD is down, use Helm directly
helm rollback microservices 1
```

---

## 🔧 Kubernetes Troubleshooting Scenarios (Senior/Lead Level)

### **Scenario 1: CrashLoopBackOff - Deep Dive**

**Symptom:**

```bash
$ kubectl get pods
NAME                    READY   STATUS              RESTARTS   AGE
api-deployment-abc123   0/1     CrashLoopBackOff    5          3m
```

**Senior-Level Troubleshooting Process:**

**Step 1: Get pod events**

```bash
kubectl describe pod api-deployment-abc123

# Look for:
Events:
  Type     Reason     Age                From               Message
  ----     ------     ----               ----               -------
  Warning  BackOff    2m (x10 over 5m)   kubelet            Back-off restarting failed container
```

**Step 2: Check logs (current AND previous)**

```bash
# Current container logs (may be empty if it crashed instantly)
kubectl logs api-deployment-abc123

# Previous container logs (the crashed one)
kubectl logs api-deployment-abc123 --previous

# Common issues found:
# - Missing environment variables
# - Database connection failed
# - Port already in use
# - Out of memory killed (OOMKilled)
```

**Step 3: Check container exit code**

```bash
kubectl get pod api-deployment-abc123 -o jsonpath='{.status.containerStatuses[0].lastState.terminated.exitCode}'

# Exit codes:
# 0: Successful (shouldn't be crashing)
# 1: Application error
# 137: OOMKilled (128 + 9 SIGKILL)
# 139: Segmentation fault
# 143: Graceful termination (SIGTERM)
```

**Step 4: Check resource limits**

```bash
kubectl describe pod api-deployment-abc123 | grep -A 5 "Limits"

# If OOMKilled (exit 137):
Limits:
  memory: 128Mi
Requests:
  memory: 64Mi

# Solution: Increase memory limits
# Or: App has memory leak (profile with heap dump)
```

**Step 5: Exec into a working container (if one exists)**

```bash
# If CrashLoopBackOff, container won't stay up
# Use debug container (Kubernetes 1.23+)
kubectl debug api-deployment-abc123 -it --image=busybox --share-processes

# Or change deployment command temporarily
kubectl set env deployment/api DEBUG=true
kubectl set image deployment/api api=api:debug
```

**Real-World Fix Examples:**

```yaml
# Problem: Database connection timeout
# Fix: Add init container to wait for DB
initContainers:
  - name: wait-for-db
    image: busybox:1.36
    command:
      [
        "sh",
        "-c",
        "until nc -z postgres 5432; do echo waiting for db; sleep 2; done",
      ]

# Problem: OOMKilled
# Fix: Increase memory based on actual usage
resources:
  requests:
    memory: "512Mi" # Was 128Mi
  limits:
    memory: "1Gi" # Was 256Mi

# Problem: Missing secret
# Fix: Ensure secret exists before deployment
env:
  - name: DATABASE_URL
    valueFrom:
      secretKeyRef:
        name: db-secret # Must exist!
        key: url
```

---

### **Scenario 2: Pod Stuck in Pending**

**Symptom:**

```bash
$ kubectl get pods
NAME                    READY   STATUS    RESTARTS   AGE
api-deployment-xyz789   0/1     Pending   0          10m
```

**Troubleshooting:**

**Step 1: Check events**

```bash
kubectl describe pod api-deployment-xyz789

# Common reasons:
# 1. Insufficient CPU/memory
Events:
  Warning  FailedScheduling  0/3 nodes are available: 3 Insufficient memory.

# 2. Node selector mismatch
Events:
  Warning  FailedScheduling  0/3 nodes matched node selector.

# 3. Pod affinity rules not satisfied
Events:
  Warning  FailedScheduling  0/3 nodes matched pod affinity/anti-affinity rules.

# 4. Volume mount issues
Events:
  Warning  FailedMount  MountVolume.SetUp failed for volume "data" : PVC "data-claim" not found
```

**Solutions:**

```bash
# Solution 1: Insufficient resources
# Check node capacity
kubectl describe nodes | grep -A 5 "Allocated resources"

# Options:
# A) Scale down other pods
# B) Add more nodes (cluster autoscaler)
# C) Reduce pod resource requests

# Solution 2: Node selector mismatch
kubectl get nodes --show-labels

# Fix deployment
spec:
  nodeSelector:
    disktype: ssd  # Ensure nodes have this label

# Solution 3: Taint/toleration issue
kubectl describe nodes | grep Taints

Taints:  node-role.kubernetes.io/control-plane:NoSchedule

# Fix: Add toleration
tolerations:
  - key: "node-role.kubernetes.io/control-plane"
    operator: "Exists"
    effect: "NoSchedule"

# Solution 4: PVC missing
kubectl get pvc  # Verify exists
kubectl describe pvc data-claim  # Check status
```

---

### **Scenario 3: Service Not Reachable (Networking Debug)**

**Symptom:**

```bash
curl: (7) Failed to connect to api-service port 3000: Connection refused
```

**Senior-Level Network Debugging:**

**Step 1: Verify service exists and has endpoints**

```bash
kubectl get svc api-service

NAME          TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)    AGE
api-service   ClusterIP   10.100.200.50   <none>        3000/TCP   5m

kubectl get endpoints api-service

NAME          ENDPOINTS                                   AGE
api-service   10.244.1.5:3000,10.244.2.8:3000            5m
# ^^^^^^ If empty, pods aren't matching service selector!
```

**Step 2: Check service selector matches pod labels**

```bash
kubectl get svc api-service -o yaml | grep -A 3 selector

selector:
  app: api  # Must match pod labels

kubectl get pods -l app=api  # Should show pods

# If no pods match: Fix service selector or pod labels
```

**Step 3: Test from another pod (inside cluster)**

```bash
# Start debug pod
kubectl run debug --image=nicolaka/netshoot -it --rm -- /bin/bash

# Inside debug pod:
curl http://api-service:3000/health

# Works from inside cluster but not outside?
# Check ingress/LoadBalancer configuration

# Doesn't work from inside cluster?
# Check network policies
```

**Step 4: Check Network Policies**

```bash
kubectl get networkpolicies

# If exists, verify rules allow traffic
kubectl describe networkpolicy api-policy

# Test: Temporarily delete network policy
kubectl delete networkpolicy api-policy

# Works now? Network policy was blocking traffic
# Re-create with correct rules
```

**Step 5: DNS debugging**

```bash
# From debug pod:
nslookup api-service

# Should return:
Server:    10.100.0.10
Address:   10.100.0.10:53

Name:   api-service.default.svc.cluster.local
Address: 10.100.200.50

# DNS not working? Check CoreDNS
kubectl get pods -n kube-system -l k8s-app=kube-dns
kubectl logs -n kube-system -l k8s-app=kube-dns
```

**Step 6: Port mismatch debugging**

```bash
# Service config
kubectl get svc api-service -o yaml

spec:
  ports:
    - port: 3000         # Service port (what you curl)
      targetPort: 8080   # Container port (where app listens)

# Check pod container port
kubectl get pod api-pod -o jsonpath='{.spec.containers[0].ports[0].containerPort}'

# Output: 3000
# ^^^ MISMATCH! Service targetPort is 8080 but container is 3000

# Fix: Update service targetPort to 3000
```

---

### **Scenario 4: Persistent Volume Claim Stuck in Pending**

**Symptom:**

```bash
$ kubectl get pvc
NAME         STATUS    VOLUME   CAPACITY   ACCESS MODES   STORAGECLASS   AGE
data-claim   Pending                                      gp3            10m
```

**Troubleshooting:**

```bash
# Step 1: Describe PVC
kubectl describe pvc data-claim

Events:
  Warning  ProvisioningFailed  waiting for a volume to be created

# Common issues:

# Issue 1: StorageClass doesn't exist
kubectl get storageclass

# No 'gp3' found? Create it:
kubectl apply -f - <<EOF
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: gp3
provisioner: ebs.csi.aws.com
parameters:
  type: gp3
  encrypted: "true"
volumeBindingMode: WaitForFirstConsumer
allowVolumeExpansion: true
EOF

# Issue 2: IAM permissions missing
# Check EBS CSI driver pod logs
kubectl logs -n kube-system -l app=ebs-csi-controller

# Error: UnauthorizedOperation: Not authorized to perform: ec2:CreateVolume
# Fix: Add IAM policy to node role or use IRSA

# Issue 3: Availability zone mismatch
# PVC requests volume in us-east-1a
# Pod scheduled on node in us-east-1b
# EBS volumes are AZ-specific!

# Solution: Use volumeBindingMode: WaitForFirstConsumer
# This delays volume creation until pod is scheduled
```

---

### **Scenario 5: High Memory/CPU Usage (Performance Debug)**

**Symptom:**

```bash
$ kubectl top pods
NAME                     CPU(cores)   MEMORY(bytes)
api-deployment-abc123    950m         1800Mi
api-deployment-def456    200m         400Mi

# One pod using 4x more resources!
```

**Senior-Level Performance Debugging:**

**Step 1: Check if it's a known pattern**

```bash
# Get pod restart count
kubectl get pod api-deployment-abc123 -o jsonpath='{.status.containerStatuses[0].restartCount}'

# Output: 0 (no restarts)
# This isn't OOMKilled, just high usage

# Check age
kubectl get pod api-deployment-abc123 -o jsonpath='{.status.startTime}'

# If old: Potential memory leak (heap grows over time)
```

**Step 2: Get heap dump (Java apps)**

```bash
# Exec into pod
kubectl exec -it api-deployment-abc123 -- /bin/bash

# Generate heap dump
jmap -dump:live,format=b,file=/tmp/heapdump.hprof <PID>

# Copy out of pod
kubectl cp api-deployment-abc123:/tmp/heapdump.hprof ./heapdump.hprof

# Analyze with MAT (Eclipse Memory Analyzer)
# Look for memory leaks, large object retention
```

**Step 3: Profile CPU (Node.js apps)**

```bash
# Enable profiling
kubectl exec api-deployment-abc123 -- kill -SIGUSR1 <PID>

# Generates CPU profile
kubectl cp api-deployment-abc123:/tmp/cpu-profile.log ./profile.log

# Analyze with clinic.js or speedscope
```

**Step 4: Check for external factors**

```bash
# Is this pod getting more traffic?
kubectl logs api-deployment-abc123 --tail=100 | wc -l

# Compare with other pod
kubectl logs api-deployment-def456 --tail=100 | wc -l

# Uneven load distribution?
# Check service loadbalancing

# Or: This pod is handling long-running requests
# Check active connections
kubectl exec api-deployment-abc123 -- netstat -an | grep ESTABLISHED | wc -l
```

**Step 5: Set up proper resource limits**

```yaml
# Based on profiling, set appropriate limits
resources:
  requests:
    memory: "512Mi" # Average usage
    cpu: "250m"
  limits:
    memory: "1Gi" # Max allowed (will OOMKill if exceeded)
    cpu: "1000m" # Throttled if exceeded


# Enable HPA to scale horizontally instead of vertically
```

---

### **Scenario 6: Helm Upgrade Failed - Rollback**

**Symptom:**

```bash
$ helm upgrade myapp ./chart
Error: UPGRADE FAILED: failed to create resource: admission webhook "validator" denied the request

$ helm list
NAME   NAMESPACE   REVISION   STATUS   CHART        APP VERSION
myapp  default     5          failed   myapp-1.2.0  1.2.0
```

**Troubleshooting:**

```bash
# Step 1: Check what changed
helm diff revision myapp 4 5

# Shows ConfigMap changed, deployment updated

# Step 2: Get detailed error
kubectl get events --sort-by='.lastTimestamp' | tail -10

# Shows: ValidatingWebhookConfiguration blocked deployment

# Step 3: Rollback to last working version
helm rollback myapp 4

# Confirm rollback
helm list
NAME   NAMESPACE   REVISION   STATUS     CHART        APP VERSION
myapp  default     6          deployed   myapp-1.1.0  1.1.0
# ^^^^ Revision increments even on rollback

# Step 4: Investigate webhook issue
kubectl get validatingwebhookconfigurations

# Step 5: Fix chart and retry
helm upgrade myapp ./chart --dry-run --debug
# Validate changes before applying
```

**Prevention:**

```bash
# Always use dry-run first
helm upgrade myapp ./chart --dry-run --debug

# Use helm diff plugin
helm plugin install https://github.com/databus23/helm-diff
helm diff upgrade myapp ./chart

# Set timeout
helm upgrade myapp ./chart --timeout 10m --wait

# Enable atomic rollback
helm upgrade myapp ./chart --atomic
# If upgrade fails, automatically rolls back
```

---

### **Scenario 7: ImagePullBackOff - ECR Authentication**

**Symptom:**

```bash
$ kubectl get pods
NAME                    READY   STATUS             RESTARTS   AGE
api-deployment-abc123   0/1     ImagePullBackOff   0          2m
```

**Troubleshooting:**

```bash
# Step 1: Describe pod
kubectl describe pod api-deployment-abc123

Events:
  Warning  Failed     Failed to pull image "123456789012.dkr.ecr.us-east-1.amazonaws.com/api:latest":
           rpc error: code = Unknown desc = Error response from daemon:
           pull access denied for 123456789012.dkr.ecr.us-east-1.amazonaws.com/api

# Step 2: Check if image exists
aws ecr describe-images \
  --repository-name api \
  --region us-east-1 \
  --image-ids imageTag=latest

# Image not found? Wrong tag or repo name

# Step 3: Check IAM permissions
# Nodes need ecr:GetAuthorizationToken, ecr:BatchGetImage, ecr:GetDownloadUrlForLayer

# For IRSA (IAM Roles for Service Accounts):
kubectl describe sa default

Annotations:
  eks.amazonaws.com/role-arn: arn:aws:iam::123456789012:role/EKSPodRole

# Check role has ECR permissions
aws iam get-role-policy --role-name EKSPodRole --policy-name ECRAccess

# Step 4: Verify region
# Image in us-west-2 but cluster in us-east-1?
# ECR is regional!

# Step 5: Check image pull secrets (if using)
kubectl get secrets
kubectl describe secret ecr-secret

# If using imagePullSecrets, ensure it's valid
```

**Fix:**

```yaml
# Option 1: Use IRSA (recommended)
apiVersion: v1
kind: ServiceAccount
metadata:
  name: api-sa
  annotations:
    eks.amazonaws.com/role-arn: arn:aws:iam::123456789012:role/EKSPodECRRole

# Then in deployment:
spec:
  serviceAccountName: api-sa

# Option 2: Add ECR policy to node IAM role (less secure)
# Attach AmazonEC2ContainerRegistryReadOnly to node role
```

---

### **Scenario 8: Readiness Probe Failing - Traffic Not Reaching Pod**

**Symptom:**

```bash
$ kubectl get pods
NAME                    READY   STATUS    RESTARTS   AGE
api-deployment-abc123   0/1     Running   0          5m

$ kubectl describe pod api-deployment-abc123
Readiness probe failed: HTTP probe failed with statuscode: 500
```

**Troubleshooting:**

```bash
# Step 1: Test endpoint manually
kubectl exec api-deployment-abc123 -- curl -v http://localhost:3000/ready

# If works: Readiness probe config wrong
# If fails: App actually not ready

# Step 2: Check app logs
kubectl logs api-deployment-abc123

# Common issues:
# - Database connection not ready
# - Required env var missing
# - App listening on wrong port

# Step 3: Adjust readiness probe timings
spec:
  readinessProbe:
    httpGet:
      path: /ready
      port: 3000
    initialDelaySeconds: 10  # Was 5, app needs more time
    periodSeconds: 5
    timeoutSeconds: 3
    failureThreshold: 3

# Step 4: Check dependency services
kubectl get pods  # Is database pod ready?
kubectl exec api-deployment-abc123 -- nc -zv postgres 5432  # Can pod reach DB?

# Step 5: Differentiate liveness vs readiness
livenessProbe:
  # Checks if app is alive (restart if fails)
  httpGet:
    path: /health  # Simple check: is process running?
  failureThreshold: 3

readinessProbe:
  # Checks if app can serve traffic (remove from service if fails)
  httpGet:
    path: /ready  # Complex check: DB connected, cache ready, etc.
  failureThreshold: 1  # Remove from LB immediately
```

---

## 🎤 Interview Questions You'll Master

### **Terraform Questions**

**Q: "How do you prevent a developer from accidentally destroying production infrastructure?"**

**Your Answer:**
"I use multiple layers of protection:

1. **State Isolation** - Each environment has completely separate state files in different S3 buckets. Destroying dev state cannot touch production.

2. **Backend Configuration** - Production backend is in a separate bucket with stricter IAM policies. Developers don't have write access.

3. **Code Protection** - I use `lifecycle { prevent_destroy = true }` on critical resources and `deletion_protection = true` on databases.

4. **GitHub Protection** - CODEOWNERS file requires 2 senior engineer approvals for production Terraform changes. Branch protection enforces this.

5. **CI/CD Only** - Developers cannot run `terraform apply` in production. Only the CI/CD pipeline has credentials, and it requires manual approval gates.

6. **DynamoDB Locking** - Prevents concurrent applies that could cause race conditions.

This layered approach means even if one protection fails, others catch it. It's the same pattern used by companies like HashiCorp and Grammarly."

---

**Q: "Explain your Terraform state management strategy."**

**Your Answer:**
"I use remote state with S3 backend and DynamoDB locking:

1. **Separate State Per Environment** - Dev, staging, and production each have their own state bucket. This ensures complete isolation.

2. **Encryption** - All state files are encrypted at rest using S3 server-side encryption. State files contain sensitive data like database passwords.

3. **Versioning** - S3 bucket versioning is enabled. If state gets corrupted, I can rollback to a previous version.

4. **Locking** - DynamoDB table provides state locking. If two engineers run `terraform apply` simultaneously, one is blocked until the other finishes.

5. **Access Control** - IAM policies restrict who can read/write state. Only CI/CD service accounts have write access to production state.

For teams without AWS, I also recommend Terraform Cloud which provides all of this out-of-the-box, plus a web UI and collaboration features."

---

### **CI/CD Questions**

**Q: "How do you optimize CI/CD pipelines for polyglot microservices?"**

**Your Answer:**
"I use a multi-layered optimization strategy:

1. **Matrix Builds** - Build all services in parallel instead of sequentially. Three services building in parallel (max 4 min) vs sequential (9 min) saves 55%.

2. **Path-Based Triggers** - Only build services that changed. If Java service changes, don't rebuild JavaScript or Rust.

3. **Language-Specific Caching**:
   - JavaScript: Cache node_modules based on package-lock.json hash
   - Java: Cache Maven .m2 directory
   - Rust: Cache Cargo registry and compiled dependencies

4. **Docker Layer Caching** - Each Dockerfile is optimized to leverage layer caching. Dependencies rarely change, so they're in cached layers.

5. **Dependency Pre-Download** - For Java and Rust, I use multi-stage builds that download dependencies in a separate layer before copying source code.

6. **Build-Time Optimizations**:
   - Rust: Use `cargo-chef` for dependency caching (game changer)
   - Java: Use `mvn dependency:go-offline` to cache dependencies
   - JavaScript: Use `npm ci` instead of `npm install`

7. **Conditional Steps** - Security scans only run on changed services. Integration tests only run when APIs change.

Result: First build takes ~10 minutes. Subsequent builds with cache: ~2 minutes. This is critical for developer experience - fast feedback encourages frequent commits."

---

**Q: "Walk me through your deployment strategy from feature to production."**

**Your Answer:**
"I use GitOps with progressive promotion:

**Step 1: Feature Development**

- Developer works on feature branch
- On push, CI runs: tests, linting, security scans
- No deployment yet

**Step 2: Deploy to Dev (Automatic)**

- PR merged to `develop` branch
- CI builds all changed services
- Updates Helm values.yaml with new image tags
- Commits to Git
- ArgoCD detects change, deploys to dev cluster
- Smoke tests run automatically
- Total time: ~5 minutes

**Step 3: Promote to Staging (Manual)**

- Engineer creates PR from `develop` to `staging`
- Requires 1 platform engineer approval
- Integration tests run (5-10 minutes)
- On merge, ArgoCD deploys to staging
- QA team validates
- Total time: ~30 minutes (includes human validation)

**Step 4: Promote to Production (Gated)**

- Engineer creates PR from `staging` to `main`
- Requires 2 senior engineer approvals
- Manual approval gate in GitHub Actions
- Blue/green deployment to production
- Canary analysis for 10 minutes
- If metrics look good, complete rollout
- If metrics degrade, automatic rollback
- Total time: ~45 minutes (includes approvals and monitoring)

**Rollback:** Simple Git revert. ArgoCD detects and auto-rolls back.

**Audit Trail:** Every deployment is a Git commit with author, timestamp, and approval history.

This pattern ensures safety while maintaining velocity. We deploy to dev 20+ times per day, staging daily, production 2-3 times per week."

---

### **Architecture Questions**

**Q: "How do you design for multiple environments with different requirements?"**

**Your Answer:**
"I use configuration-driven differentiation with shared modules:

**Shared Modules:** Infrastructure components (VPC, EKS, RDS) are written once as Terraform modules. This ensures consistency across environments.

**Environment-Specific Config:**

- **Dev**: Small instances (t3.medium), single AZ, no multi-region, deletion protection off. Goal: Fast iteration, low cost (~$100/mo)
- **Staging**: Medium instances (m5.large), multi-AZ, production-like but cheaper. Goal: Realistic testing (~$400/mo)
- **Production**: Large instances (m5.2xlarge), multi-AZ, multi-region DR, deletion protection on. Goal: High availability, performance (~$1200/mo)

**Configuration Files:**

```
terraform/environments/dev/terraform.tfvars → Small, cheap
terraform/environments/staging/terraform.tfvars → Medium
terraform/environments/production/terraform.tfvars → Large, HA
```

**Benefits:**

1. Modules enforce consistency (same security group rules across all envs)
2. tfvars allow differentiation (dev uses 2 nodes, prod uses 6)
3. Cost optimization (dev is 92% cheaper than prod but architecturally equivalent)
4. Testing confidence (staging matches prod architecture)

**Example:** Dev has 1 NAT Gateway ($32/mo), prod has 2 for redundancy ($64/mo). Same module, different variable."

---

## 📚 Real Company Patterns

### **Companies Using These Patterns**

| Company       | Pattern                                            | Source                         |
| ------------- | -------------------------------------------------- | ------------------------------ |
| **HashiCorp** | Terraform workspace isolation per environment      | Terraform docs, HashiCorp blog |
| **Grammarly** | S3 backend with DynamoDB locking, 3 environments   | Engineering blog               |
| **Uber**      | Environment-specific tfvars, CODEOWNERS for prod   | Uber Engineering blog          |
| **Netflix**   | Polyglot microservices (Java, Node, Python)        | Netflix Tech Blog              |
| **Spotify**   | GitOps with ArgoCD, automated canary deployments   | Spotify Engineering            |
| **Google**    | Matrix builds for mono-repo, Bazel for caching     | Google Cloud blog              |
| **Airbnb**    | Kubernetes multi-tenant, namespace per environment | Airbnb Engineering             |
| **Stripe**    | Terraform modules with env-specific overrides      | Stripe blog                    |

### **Key Takeaways from FAANG**

1. **Isolation is Non-Negotiable** - No company lets dev changes touch production
2. **Automation with Gates** - Deploy often, but require approvals for prod
3. **Caching Saves Time** - All companies optimize build times aggressively
4. **GitOps for Audit** - Compliance requires every change in Git
5. **Cost Optimization** - Dev environments are 80-90% cheaper than prod

---

## 💰 Cost Optimization

### **Monthly Cost Breakdown**

| Component            | Dev       | Staging   | Production |
| -------------------- | --------- | --------- | ---------- |
| EKS Control Plane    | $72       | $72       | $72        |
| EC2 Nodes            | $30       | $150      | $500       |
| NAT Gateway          | $32       | $64       | $64        |
| RDS                  | $15       | $80       | $300       |
| Load Balancer        | $18       | $18       | $36        |
| S3 (Terraform State) | $1        | $1        | $1         |
| CloudWatch           | $5        | $20       | $80        |
| Data Transfer        | $10       | $30       | $100       |
| **Total**            | **~$183** | **~$435** | **~$1153** |

### **Cost Optimization Strategies**

1. **Spot Instances for Dev** - Save 70% on compute
2. **Shutdown Dev at Night** - Save 50% on off-hours
3. **Reserved Instances for Prod** - Save 40% on 1-year commit
4. **Right-Sizing** - Dev doesn't need m5.2xlarge
5. **Single NAT for Dev** - Save $32/mo vs multi-AZ
6. **Shorter Backup Retention** - Dev: 1 day, Prod: 30 days

**Potential Savings:** ~$500/mo (30% reduction)

---

## 🔧 Troubleshooting Guide

### **Common Issues & Solutions**

#### **Issue: "Backend initialization failed"**

**Symptom:**

```
Error: Failed to get existing workspaces: S3 bucket does not exist
```

**Solution:**

```bash
# Create S3 bucket and DynamoDB table first
aws s3 mb s3://mycompany-terraform-state-dev
aws dynamodb create-table \
  --table-name terraform-lock-dev \
  --attribute-definitions AttributeName=LockID,AttributeType=S3 \
  --key-schema AttributeName=LockID,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST
```

**Prevention:** Create backend infrastructure before running `terraform init`

---

#### **Issue: "Error acquiring state lock"**

**Symptom:**

```
Error: Error locking state: ConditionalCheckFailedException
Lock Info:
  ID: abc-123-def-456
  Operation: OperationTypeApply
  Who: john@laptop
  Created: 2026-04-03 14:32:15
```

**Cause:** Another process is running terraform, or previous run crashed without releasing lock

**Solution:**

```bash
# Check if the terraform process is actually running
ps aux | grep terraform

# If not, force unlock (USE WITH CAUTION!)
terraform force-unlock abc-123-def-456
```

**Prevention:** Use DynamoDB locking, don't kill terraform processes

---

#### **Issue: "Build cache not working"**

**Symptom:** Every build downloads all dependencies from scratch

**Solution:**

**For npm:**

```yaml
- uses: actions/setup-node@v3
  with:
    cache: "npm"
    cache-dependency-path: package-lock.json # ← Must be exact
```

**For Maven:**

```yaml
- uses: actions/setup-java@v3
  with:
    cache: "maven" # ← Caches ~/.m2
```

**For Rust:**

```yaml
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry/
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('Cargo.lock') }}
```

**Prevention:** Always specify cache in GitHub Actions

---

#### **Issue: "Docker image too large"**

**Symptom:** Docker image is 2GB, takes 10 minutes to push

**Solutions:**

1. **Multi-stage builds** - Only copy runtime dependencies to final image
2. **Alpine base images** - Use `node:20-alpine` not `node:20`
3. **.dockerignore** - Exclude node_modules, .git, tests
4. **Layer optimization** - Put rarely-changing files first

**Example:**

```dockerfile
# Bad: 900MB
FROM node:20
COPY . .
RUN npm install

# Good: 80MB
FROM node:20-alpine AS builder
COPY package*.json ./
RUN npm ci --only=production
COPY src ./src

FROM node:20-alpine
COPY --from=builder /app /app
```

---

## 🎯 Implementation Roadmap

### **Week 1-2: AWS Foundation & OIDC**

**Goals:**

- [ ] Phase 0: Set up GitHub OIDC for AWS
  - [ ] Create OIDC provider in AWS
  - [ ] Create IAM roles (dev, staging, production)
  - [ ] Configure trust policies
  - [ ] Test OIDC authentication in GitHub Actions
- [ ] Phase 1: Terraform environment isolation
  - [ ] Create bootstrap module for S3/DynamoDB
  - [ ] Set up remote state per environment
  - [ ] Configure backend configurations
  - [ ] Test `terraform destroy` workflow
  - [ ] Set up CODEOWNERS for production protection
  - [ ] Document cost management strategy

**Key Milestone:** Can deploy infrastructure with OIDC, no access keys

**Time:** 12-15 hours

**Cost:** ~$50-80/month (with nightly destroy of dev)

---

### **Week 3-4: EKS Cluster & Container Registry**

**Goals:**

- [ ] Phase 2: EKS with ECR
  - [ ] Create EKS Terraform module
  - [ ] Deploy EKS cluster to dev
  - [ ] Configure kubectl access
  - [ ] Set up ECR repositories
  - [ ] Configure IAM roles for pods (IRSA)
  - [ ] Deploy Cluster Autoscaler
  - [ ] Configure CloudWatch Container Insights
  - [ ] Test pod→ECR image pull
  - [ ] Document EKS authentication flow

**Key Milestone:** Running EKS cluster with ECR integration

**Time:** 10-12 hours

**Cost:** ~$155/month for dev (EKS $73 + nodes $82)

---

### **Week 5-6: Advanced Kubernetes & Helm**

**Goals:**

- [ ] Phase 3: Kubernetes mastery
  - [ ] Create production-grade Helm chart
  - [ ] Implement HPA (Horizontal Pod Autoscaler)
  - [ ] Configure Pod Disruption Budgets
  - [ ] Set up Network Policies
  - [ ] Configure RBAC roles
  - [ ] Implement StatefulSet for database
  - [ ] Practice all 8 troubleshooting scenarios
  - [ ] Document kubectl commands reference

**Key Milestone:** Deep Kubernetes expertise, can troubleshoot production issues

**Time:** 15-20 hours (includes hands-on troubleshooting practice)

**Portfolio Value:** This alone qualifies you for Senior K8s Engineer roles

---

### **Week 7: Multi-Language CI/CD**

**Goals:**

- [ ] Phase 4: Polyglot microservices
  - [ ] Create JavaScript/Node.js API
  - [ ] Create Java/Spring Boot service
  - [ ] Create Rust processor
  - [ ] Optimize Dockerfiles per language
  - [ ] Build matrix CI/CD workflow
  - [ ] Push images to ECR
  - [ ] Deploy all services to EKS

**Key Milestone:** Three microservices running on EKS

**Time:** 12-15 hours

---

### **Week 8: Security Scanning**

**Goals:**

- [ ] Phase 5: Environment-specific scanning
  - [ ] Implement branch detection workflow
  - [ ] Configure fast scans for dev
  - [ ] Configure comprehensive scans for staging
  - [ ] Configure full scans for production
  - [ ] Add inline security suppressions
  - [ ] Set up manual approval gates
  - [ ] Document security strategy

**Key Milestone:** Multi-tier security that doesn't slow down development

**Time:** 8-10 hours

---

### **Week 9: GitOps & Integration**

**Goals:**

- [ ] Phase 6: ArgoCD and promotion
  - [ ] Install ArgoCD on EKS
  - [ ] Configure ArgoCD applications per environment
  - [ ] Set up automated sync for dev
  - [ ] Configure manual approvals for production
  - [ ] Test environment promotion (dev → staging → prod)
  - [ ] Practice rollback procedures
  - [ ] Document GitOps workflow

**Key Milestone:** Full GitOps with audit trail

**Time:** 8-10 hours

---

### **Week 10: Cost Management & Cleanup**

**Goals:**

- [ ] Implement automated nightly destroy for dev
- [ ] Configure Cost Explorer alerts
- [ ] Set up budget notifications
- [ ] Document cost per environment
- [ ] Practice terraform destroy safely
- [ ] Verify state files remain intact
- [ ] Document disaster recovery

**Key Milestone:** Cost-optimized setup, no surprise AWS bills

**Time:** 6-8 hours

**Total:** 42-54 hours over 6 weeks

---

## 📝 Next Steps

### **Getting Started**

1. **Create New Repository**

   ```bash
   mkdir enterprise-infrastructure
   cd enterprise-infrastructure
   git init
   ```

2. **Follow This Roadmap**
   - Copy this README into your repo
   - Work through each phase
   - Commit often, document decisions

3. **Use AI Assistants**
   - Feed sections of this README to GitHub Copilot
   - Ask ChatGPT for code examples
   - Request clarifications on concepts

4. **Build Your Portfolio**
   - Public GitHub repository
   - Clear README with architecture diagrams
   - Demo videos showing deployments
   - Blog posts explaining decisions

### **Interview Preparation**

Practice explaining:

- Your Terraform isolation strategy (5 min)
- Your CI/CD optimization approach (5 min)
- A deployment from code to production (10 min)
- How you'd handle an incident (10 min)

### **Salary Negotiation**

With this project, target:

- **Mid-Senior:** $150k-$180k
- **Senior:** $180k-$220k
- **Staff/Principal:** $220k-$280k

Use phrases like:

- "I've implemented enterprise Terraform patterns used by HashiCorp..."
- "My polyglot CI/CD pipeline handles JavaScript, Java, and Rust with matrix builds..."
- "I use GitOps with ArgoCD for auditable, declarative deployments..."

---

## 🤝 Support

### **Resources**

- **Terraform Docs:** https://www.terraform.io/docs
- **AWS EKS Docs:** https://docs.aws.amazon.com/eks/
- **GitHub Actions Docs:** https://docs.github.com/actions
- **ArgoCD Docs:** https://argo-cd.readthedocs.io/
- **Kubernetes Docs:** https://kubernetes.io/docs/
- **Helm Docs:** https://helm.sh/docs/
- **AWS OIDC Guide:** https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/configuring-openid-connect-in-amazon-web-services
- **This README:** Use with GitHub Copilot or ChatGPT for implementation help

### **Community**

- Stack Overflow: [terraform], [github-actions], [kubernetes], [amazon-eks]
- Reddit: r/devops, r/kubernetes, r/terraform, r/aws
- Discord: DevOps communities, Kubernetes Slack
- AWS re:Post: https://repost.aws/

---

## 🎓 Conclusion

This roadmap will take you from intermediate to **senior/lead/principal engineer level**. It's comprehensive, production-grade, and based on real patterns from top companies operating at scale.

### **What You'll Achieve**

After completing this roadmap, you will have:

✅ **AWS Expertise**

- Production-grade EKS clusters
- OIDC authentication (no access keys!)
- Cost-optimized multi-environment architecture
- ECR container registry mastery

✅ **Kubernetes Mastery**

- Advanced troubleshooting skills
- HPA, PDB, Network Policies, RBAC
- StatefulSets and persistent storage
- Senior-level debugging capabilities

✅ **Terraform Proficiency**

- Environment isolation with S3/DynamoDB
- Infrastructure as Code best practices
- Safe terraform destroy workflows
- Production protection mechanisms

✅ **CI/CD Excellence**

- Multi-language pipeline (JavaScript, Java, Rust)
- Environment-specific security scanning
- GitOps with ArgoCD
- Matrix builds and optimization

✅ **Security Mindset**

- 6-layer security scanning
- OIDC over access keys
- Inline risk documentation
- Manual approval gates

### **Salary Impact**

With this project in your portfolio, target these ranges:

| Role                              | Salary Range (US) | What Unlocks It                                  |
| --------------------------------- | ----------------- | ------------------------------------------------ |
| **Senior Platform Engineer**      | $160k-$200k       | Complete project, can explain all decisions      |
| **Lead DevOps Engineer**          | $180k-$230k       | + Can troubleshoot production incidents live     |
| **Staff SRE**                     | $220k-$280k       | + Can design systems from scratch, mentor others |
| **Principal Kubernetes Engineer** | $250k-$320k       | + Deep K8s expertise, thought leader in org      |

### **Interview Confidence**

After completing this, you can confidently say:

> "I've built a complete platform engineering solution on AWS using EKS, implementing OIDC authentication for GitHub Actions to eliminate access key risks. I designed a multi-environment Terraform architecture with complete state isolation using S3 and DynamoDB locking. I've deployed polyglot microservices—JavaScript, Java, and Rust—with language-optimized CI/CD pipelines achieving sub-5-minute build times through intelligent caching. I implemented environment-aware security scanning that balances developer velocity with production safety, and I've hands-on troubleshooting experience with CrashLoopBackOff, networking issues, and performance debugging at a senior level. I use GitOps with ArgoCD for declarative deployments with full audit trails, and I've implemented cost controls including automated infrastructure cleanup to prevent AWS billing surprises."

**Hiring managers hear:** "This person has built production systems. They understand trade-offs. They can hit the ground running."

---

### **Remember**

- **Take your time** - Understanding beats speed
- **Document everything** - Your future self will thank you
- **Break things** - Best way to learn is fixing what you broke
- **Explain out loud** - If you can teach it, you know it
- **Destroy nightly** - Save money, practice recovery

### **Cost Control Reminder**

```bash
# Every evening before logging off:
cd terraform/environments/dev
terraform destroy -auto-approve

# Every morning:
terraform apply -auto-approve

# Savings: ~$50-80/month
# Learning value: Practiced disaster recovery 30+ times
```

### **You're Ready**

You're building something that **will get you hired and promoted**. Hiring managers see hundreds of "TODO app" portfolios with basic Docker and basic Kubernetes. This project shows you can:

- Build production-grade infrastructure
- Make senior-level architectural decisions
- Balance security with velocity
- Optimize for cost and performance
- Troubleshoot complex distributed systems
- Explain trade-offs clearly

**That's what separates senior engineers from juniors.**

**Good luck, and remember:** The best time to start was yesterday. The second best time is now. 🚀

---

## 📊 Project Summary

**Total Learning Time:** 80-100 hours (10-12 weeks at 8-10 hours/week)

**Monthly AWS Cost:**

- Dev (destroyed nightly): ~$20-30/month
- Dev (always on): ~$155/month
- Staging: ~$310/month
- Production: ~$690/month

**Technologies Mastered:** 20+

- AWS (EKS, ECR, VPC, RDS, S3, IAM, CloudWatch)
- Kubernetes (pods, deployments, services, HPA, PDB, network policies, RBAC)
- Terraform (modules, remote state, environment isolation)
- Helm (charts, values, templating, lifecycle)
- GitHub Actions (workflows, OIDC, matrix builds, environments)
- Docker (multi-stage builds, layer optimization)
- Languages (JavaScript, Java, Rust)
- GitOps (ArgoCD, declarative deployments)
- Security (SAST, SCA, container scanning, IaC scanning)
- Monitoring (Prometheus, Grafana)

**Interview Questions Mastered:** 30+

**Troubleshooting Scenarios Practiced:** 8 senior-level scenarios

**Career Impact:** Qualifies you for Senior/Lead/Staff roles ($160k-$280k+)

---

**Version:** 2.0 (AWS-Focused, Kubernetes-Advanced)  
**Last Updated:** April 3, 2026  
**Author:** Senior Platform Engineering Curriculum  
**Focus:** Production-grade AWS + Kubernetes + Multi-language CI/CD  
**License:** Use freely for learning and portfolio building

---

### **Next Steps**

1. ⭐ Star this guide
2. 📋 Fork into your own repository
3. ✅ Start with Phase 0 (OIDC setup)
4. 📝 Document as you go
5. 💼 Update LinkedIn with new skills
6. 🎯 Apply for senior roles

**You've got this!** 💪
