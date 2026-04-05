# ==========================================
# Outputs
# ==========================================

output "s3_bucket_id" {
  description = "S3 bucket name for Terraform state"
  value       = aws_s3_bucket.terraform_state.id
}

output "s3_bucket_arn" {
  description = "S3 bucket ARN"
  value       = aws_s3_bucket.terraform_state.arn
}

output "dynamodb_table_name" {
  description = "DynamoDB table name for state locking"
  value       = aws_dynamodb_table.terraform_lock.name
}

output "dynamodb_table_arn" {
  description = "DynamoDB table ARN"
  value       = aws_dynamodb_table.terraform_lock.arn
}

output "backend_config" {
  description = "Backend configuration for use in environment configs"
  value = {
    bucket         = aws_s3_bucket.terraform_state.id
    key            = "infrastructure/terraform.tfstate"
    region         = data.aws_region.current.name
    encrypt        = true
    dynamodb_table = aws_dynamodb_table.terraform_lock.name
  }
}
