# ==========================================
# Variables - Staging Environment
# ==========================================

variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "staging"
}

variable "project_name" {
  description = "Project name for resource naming"
  type        = string
  default     = "aws-devops-mastery"
}
