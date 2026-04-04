# ==========================================
# Bootstrap: OIDC Provider Setup
# ==========================================
# Run this FIRST to enable GitHub Actions → AWS authentication

terraform {
  required_version = ">= 1.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }

  # Using LOCAL state for bootstrap
  # This is the "chicken and egg" - we need OIDC before we can set up remote state
  backend "local" {
    path = "terraform.tfstate"
  }
}

provider "aws" {
  region = var.aws_region
  
  default_tags {
    tags = {
      Project     = "AWS-DevOps-Mastery"
      ManagedBy   = "Terraform"
      Environment = "bootstrap"
    }
  }
}

variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "github_repository" {
  description = "GitHub repository in format: owner/repo (e.g., chrisjamaica91/aws-devops-mastery)"
  type        = string
}

# Use the OIDC module we created
module "github_oidc" {
  source = "../modules/github-oidc"
  
  github_repository = var.github_repository
  role_name         = "GitHubActionsRole-DevOpsMastery"
}

# Outputs
output "oidc_provider_arn" {
  description = "GitHub OIDC Provider ARN"
  value       = module.github_oidc.oidc_provider_arn
}

output "github_actions_role_arn" {
  description = "IAM Role ARN for GitHub Actions - use this in workflows"
  value       = module.github_oidc.role_arn
}
