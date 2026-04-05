# AWS DevOps Mastery - Production-Grade Platform Engineering

> **Goal:** Master enterprise-level AWS, Kubernetes, and CI/CD patterns to achieve Senior/Lead Platform Engineer competency ($180k-$300k range)

## 📚 Project Status

### ✅ Completed Phases

- **Phase 0: AWS OIDC Authentication** ✅ **COMPLETE**
  - OIDC provider configured (imported existing resource)
  - Custom least privilege IAM policy (13 permission statements)
  - IAM role with 1-hour session duration
  - Tested and verified GitHub Actions → AWS authentication
  - Successfully demonstrated security boundaries (S3 path restrictions, IAM user creation blocked)

- **Phase 1: Enterprise Terraform Isolation** ✅ **COMPLETE**
  - Remote state infrastructure (3 S3 buckets + 3 DynamoDB tables)
  - Complete environment isolation (dev, staging, production)
  - State versioning, encryption, and lifecycle policies
  - DynamoDB state locking configured and tested
  - Professional environment folder structure
  - Backend configurations for all environments
  - Verified remote state storage (no local state files)

### 🚧 In Progress

- **Phase 2:** EKS Cluster with ECR (Next Up!)

### 📋 Upcoming Phases

1. **Phase 2:** EKS Cluster with ECR
2. **Phase 3:** Advanced Kubernetes & Helm Mastery
3. **Phase 4:** Multi-Language CI/CD Pipeline
4. **Phase 5:** Environment-Specific Security Scanning
5. **Phase 6:** Integration & GitOps

---

## 🏗️ Current Architecture

### Authentication & Authorization

**OIDC-Based CI/CD** (Phase 0 - Completed)

- GitHub Actions → AWS authentication via OpenID Connect
- Temporary credentials (1-hour expiration)
- Least privilege IAM policy with resource-level restrictions
- No permanent access keys stored

**Security Boundaries Implemented:**

- S3: Only `*-terraform-state-*` bucket patterns
- IAM Roles: Only `eks-*`, `GitHubActions*`, `*-node-role` patterns
- Secrets: Only `eks/*` and `app/*` namespaces
- DynamoDB: Only `terraform-lock-*` table patterns

---

## 🎓 Learning Notes & Key Concepts

### Phase 0: OIDC Authentication

**Key Learnings:**

- **Why OIDC over Access Keys:** Temporary credentials, automatic expiration, repo/branch restrictions, audit trail
- **Trust Policies:** Define WHO can assume a role (GitHub's OIDC provider)
- **Permission Policies:** Define WHAT the role can do (our least privilege policy)
- **Defense in Depth:** Multiple security layers (OIDC → Session Duration → Least Privilege → Resource Patterns)

**Production Patterns Learned:**

- Resource naming conventions for security boundaries
- IAM policy organization and documentation
- Terraform module structure (root vs child modules)
- Infrastructure as Code version management

---

## 🚀 Advanced Concepts & Future Enhancements

This section documents enterprise-level patterns to implement as the project matures. These represent the difference between mid-level and senior/staff engineer approaches.

### 🔐 Phase 0 Advanced: Multi-Environment OIDC Security

#### 1. Environment-Specific IAM Roles

**Current State:** Single role for all environments
**Advanced Pattern:** Separate roles per environment with different permission levels

```hcl
# Production role - Highly restrictive
resource "aws_iam_role" "github_actions_production" {
  name = "GitHubActionsRole-Production"

  assume_role_policy = jsonencode({
    # ... existing OIDC trust policy ...
    Condition = {
      StringEquals = {
        "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
        # ONLY main branch can deploy to production
        "token.actions.githubusercontent.com:sub" = "repo:chrisjamaica91/aws-devops-mastery:ref:refs/heads/main"
      }
    }
  })
}

# Development role - More permissive
resource "aws_iam_role" "github_actions_dev" {
  name = "GitHubActionsRole-Dev"

  assume_role_policy = jsonencode({
    # ... existing OIDC trust policy ...
    Condition = {
      StringLike = {
        # Any branch can deploy to dev
        "token.actions.githubusercontent.com:sub" = "repo:chrisjamaica91/aws-devops-mastery:*"
      }
    }
  })
}

# Staging role - Medium restrictions
resource "aws_iam_role" "github_actions_staging" {
  name = "GitHubActionsRole-Staging"

  assume_role_policy = jsonencode({
    # ... existing OIDC trust policy ...
    Condition = {
      StringLike = {
        # Only main and develop branches
        "token.actions.githubusercontent.com:sub" = [
          "repo:chrisjamaica91/aws-devops-mastery:ref:refs/heads/main",
          "repo:chrisjamaica91/aws-devops-mastery:ref:refs/heads/develop"
        ]
      }
    }
  })
}
```

**Benefits:**

- Feature branches cannot accidentally deploy to production
- Different permission levels reduce blast radius
- Enforces gitflow/trunk-based development patterns
- Audit trail shows which branch/commit deployed where

#### 2. OIDC Condition Keys for Fine-Grained Control

**Advanced Pattern:** Use all available OIDC claims for enhanced security

```hcl
Condition = {
  StringEquals = {
    "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
    # Restrict to specific repo
    "token.actions.githubusercontent.com:sub" = "repo:chrisjamaica91/aws-devops-mastery:ref:refs/heads/main"
  }
  StringLike = {
    # Only allow from main repository (not forks)
    "token.actions.githubusercontent.com:repository_owner" = "chrisjamaica91"
  }
  # Optional: Restrict to specific GitHub environments
  # "token.actions.githubusercontent.com:environment" = "production"

  # Optional: Require specific workflow file
  # "token.actions.githubusercontent.com:job_workflow_ref" = "chrisjamaica91/aws-devops-mastery/.github/workflows/production-deploy.yml@*"
}
```

**Available OIDC Claims:**

- `sub`: Repository and ref (branch/tag)
- `repository`: Full repo name
- `repository_owner`: GitHub username/org
- `environment`: GitHub environment name
- `ref`: Git ref being deployed
- `sha`: Commit SHA
- `workflow`: Workflow name
- `job_workflow_ref`: Workflow file reference
- `actor`: GitHub user who triggered workflow

**Real-World Use Case:**

```
Company policy: Only releases tagged with semantic versioning
can deploy to production
```

```hcl
Condition = {
  StringLike = {
    # Only tags matching v*.*.* pattern
    "token.actions.githubusercontent.com:ref" = "refs/tags/v*.*.*"
  }
}
```

#### 3. Session Tagging for Advanced Auditing

**Advanced Pattern:** Tag assume-role sessions with metadata for CloudTrail

```hcl
# In GitHub Actions workflow
- name: Configure AWS credentials
  uses: aws-actions/configure-aws-credentials@v4
  with:
    role-to-assume: arn:aws:iam::737026300147:role/GitHubActionsRole-Production
    aws-region: us-east-1
    role-session-name: "GHA-${{ github.actor }}-${{ github.run_id }}"
    # Session tags for auditing
    role-session-tags: |
      GitHubActor=${{ github.actor }}
      GitHubRepository=${{ github.repository }}
      GitHubRef=${{ github.ref }}
      GitHubSHA=${{ github.sha }}
      Environment=production
```

**CloudTrail Query:**

```sql
SELECT
  userIdentity.sessionContext.sessionIssuer.userName as Role,
  resources[0].type as ResourceType,
  eventName as Action,
  sourceIPAddress,
  userAgent
FROM cloudtrail_logs
WHERE
  userIdentity.sessionContext.sessionIssuer.tags.GitHubActor = 'chrisjamaica91'
  AND userIdentity.sessionContext.sessionIssuer.tags.Environment = 'production'
```

**Benefits:**

- Know exactly who deployed what, when, and from which commit
- Correlate AWS actions back to GitHub PRs/commits
- Compliance and security investigations
- Cost attribution by team/service

#### 4. AWS Organizations Service Control Policies (SCPs)

**Enterprise Pattern:** Add organization-level guardrails

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "DenyLeaveOrganization",
      "Effect": "Deny",
      "Action": "organizations:LeaveOrganization",
      "Resource": "*"
    },
    {
      "Sid": "RequireMFAForHighRiskActions",
      "Effect": "Deny",
      "Action": [
        "ec2:TerminateInstances",
        "rds:DeleteDBInstance",
        "s3:DeleteBucket"
      ],
      "Resource": "*",
      "Condition": {
        "BoolIfExists": {
          "aws:MultiFactorAuthPresent": "false"
        }
      }
    },
    {
      "Sid": "DenyRegionsOutsideAllowedList",
      "Effect": "Deny",
      "NotAction": ["iam:*", "organizations:*", "route53:*", "cloudfront:*"],
      "Resource": "*",
      "Condition": {
        "StringNotEquals": {
          "aws:RequestedRegion": ["us-east-1", "us-west-2"]
        }
      }
    }
  ]
}
```

**Why This Matters:**

- Even if IAM policy is compromised, SCP prevents critical actions
- Enforces organizational policies across all accounts
- Prevents region sprawl and associated compliance issues

#### 5. Breakglass Emergency Access

**Advanced Pattern:** Emergency access procedures with full audit trail

```hcl
# Breakglass role - only for emergencies
resource "aws_iam_role" "breakglass" {
  name = "BreakglassRole"
  max_session_duration = 3600  # 1 hour max

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        AWS = "arn:aws:iam::737026300147:user/senior-engineer-1"
      }
      Action = "sts:AssumeRole"
      Condition = {
        # Require MFA
        Bool = {
          "aws:MultiFactorAuthPresent" = "true"
        }
        # Only allow during business hours (example)
        DateGreaterThan = {
          "aws:CurrentTime" = "2024-01-01T00:00:00Z"
        }
      }
    }]
  })
}

# CloudWatch alarm on breakglass usage
resource "aws_cloudwatch_metric_alarm" "breakglass_alert" {
  alarm_name          = "BreakglassRoleUsed"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  metric_name         = "AssumeRole"
  namespace           = "AWS/IAM"
  period              = 60
  statistic           = "Sum"
  threshold           = 0
  alarm_description   = "URGENT: Breakglass role was assumed"
  alarm_actions       = [aws_sns_topic.security_alerts.arn]

  dimensions = {
    RoleName = aws_iam_role.breakglass.name
  }
}
```

#### 6. Permission Boundaries

**Advanced Pattern:** Set maximum permissions for roles created by CI/CD

```hcl
# Permission boundary - CI/CD cannot grant more than this
resource "aws_iam_policy" "permission_boundary" {
  name = "GitHubActionsBoundary"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "eks:*",
          "ec2:*",
          "ecr:*",
          "logs:*"
        ]
        Resource = "*"
      },
      {
        # Deny dangerous actions even if IAM policy allows
        Effect = "Deny"
        Action = [
          "iam:CreateUser",
          "iam:CreateAccessKey",
          "organizations:*"
        ]
        Resource = "*"
      }
    ]
  })
}

# Attach boundary to CI/CD role
resource "aws_iam_role" "github_actions" {
  name                 = "GitHubActionsRole"
  permissions_boundary = aws_iam_policy.permission_boundary.arn
  # ... rest of role config
}
```

**Real-World Scenario:**

```
Even if attacker modifies Terraform to grant themselves admin access,
the permission boundary prevents privilege escalation beyond EKS/EC2/ECR
```

---

### 📋 Implementation Checklist

As the project progresses, implement these advanced patterns:

#### Phase 0 (OIDC) - Advanced

- [ ] Separate IAM roles per environment (dev/staging/prod)
- [ ] Branch-based deployment restrictions
- [ ] Session tagging for CloudTrail queries
- [ ] Permission boundaries on CI/CD roles
- [ ] Breakglass emergency access procedures
- [ ] CloudWatch alarms on sensitive role usage

#### Phase 1 (Terraform State) - Advanced _(Coming Soon)_

- [ ] State file encryption with customer-managed KMS keys
- [ ] Cross-region state replication for DR
- [ ] State file access logging and monitoring
- [ ] Terraform Cloud/Enterprise integration
- [ ] Policy-as-Code with Sentinel/OPA

#### Phase 2 (EKS) - Advanced _(Coming Soon)_

- [ ] Pod security standards/policies (PSS/PSP replacement)
- [ ] Network policies (Calico/Cilium)
- [ ] IRSA (IAM Roles for Service Accounts)
- [ ] Cluster autoscaler with spot instances
- [ ] Multi-AZ and multi-region considerations

#### Phase 3 (Kubernetes) - Advanced _(Coming Soon)_

- [ ] Service mesh (Istio/Linkerd) implementation
- [ ] Advanced observability (distributed tracing)
- [ ] Chaos engineering for resilience testing
- [ ] GitOps with progressive delivery (Flagger/Argo Rollouts)

#### Phase 4 (CI/CD) - Advanced _(Coming Soon)_

- [ ] Build caching strategies across languages
- [ ] Supply chain security (SBOM, signature verification)
- [ ] Ephemeral environments per PR
- [ ] Contract testing for microservices
- [ ] Performance regression detection

#### Phase 5 (Security) - Advanced _(Coming Soon)_

- [ ] Runtime security monitoring (Falco)
- [ ] Policy enforcement (OPA/Kyverno)
- [ ] Secrets rotation automation
- [ ] Vulnerability management workflow
- [ ] Compliance as Code (InSpec/Cloud Custodian)

---

## 💡 Senior Engineer Thinking Patterns

Throughout this project, we're not just building infrastructure - we're developing the mindset that separates senior engineers from mid-level:

### Questions Senior Engineers Ask

1. **"What happens when this fails?"**
   - Not "if" but "when" - design for failure

2. **"How do we prevent this from being abused?"**
   - Security is not an afterthought

3. **"Can we test this without risking production?"**
   - Separation of concerns, blast radius containment

4. **"How will we know if something goes wrong?"**
   - Observability from day one

5. **"What's the maintenance burden of this approach?"**
   - Complexity vs. value tradeoff

6. **"How does this scale as the team grows?"**
   - Conway's Law - architecture reflects organization

---

## 📚 Resources & References

### Documentation

- [AWS IAM Best Practices](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html)
- [GitHub Actions OIDC](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect)
- [Terraform Best Practices](https://www.terraform-best-practices.com/)

### Industry Patterns

- [AWS Well-Architected Framework](https://aws.amazon.com/architecture/well-architected/)
- [CNCF Cloud Native Trail Map](https://github.com/cncf/trailmap)
- [Site Reliability Engineering (SRE) Books](https://sre.google/books/)

---

## 🎯 Interview Preparedness

After completing each phase, you should be able to confidently discuss:

**Phase 0 (OIDC):**

- Explain OIDC vs access keys with security trade-offs
- Describe your IAM least privilege implementation
- Walk through your trust policy and why each condition exists
- Discuss defense-in-depth security layers

**Upcoming Phases:**

- [Will be populated as we complete each phase]

---

## 📊 Progress Tracking

| Phase                 | Status         | Completion Date | Key Achievements                                                        |
| --------------------- | -------------- | --------------- | ----------------------------------------------------------------------- |
| Phase 0: OIDC         | ✅ Complete    | April 4, 2026   | OIDC auth, least privilege policy, Terraform import, tested & verified  |
| Phase 1: Terraform    | ✅ Complete    | April 4, 2026   | Remote state (S3+DynamoDB), environment isolation, state locking tested |
| Phase 2: EKS          | 🚧 In Progress | -               | -                                                                       |
| Phase 3: K8s Advanced | ⏸️ Not Started | -               | -                                                                       |
| Phase 4: CI/CD        | ⏸️ Not Started | -               | -                                                                       |
| Phase 5: Security     | ⏸️ Not Started | -               | -                                                                       |
| Phase 6: GitOps       | ⏸️ Not Started | -               | -                                                                       |

---

**Remember:** Every advanced pattern here represents real production problems and solutions. As we progress, I'll continue to highlight senior/lead-level thinking and suggest advanced implementations. The goal isn't just to complete the project, but to understand it at a depth that makes you invaluable in interviews and on the job. 🚀
