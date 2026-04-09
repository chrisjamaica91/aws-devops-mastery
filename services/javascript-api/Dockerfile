# Stage 1: Build stage (installs all dependencies including dev dependencies)
FROM node:18-alpine AS builder

WORKDIR /app

# Copy package files first (Docker layer caching - if package.json unchanged, skip npm install)
COPY package*.json ./

# Install ALL dependencies (production + development)
RUN npm ci --only=production

# Stage 2: Runtime stage (final lightweight image)
FROM node:18-alpine

# Run as non-root user for security (CVE compliance)
RUN addgroup -g 1001 -S nodejs && adduser -S nodejs -u 1001

WORKDIR /app

# Copy only production node_modules from builder stage
COPY --from=builder --chown=nodejs:nodejs /app/node_modules ./node_modules

# Copy application code
COPY --chown=nodejs:nodejs index.js ./

# Switch to non-root user
USER nodejs

# Expose port 3000 (doesn't actually open port, just documentation)
EXPOSE 3000

# Health check (Docker can restart container if unhealthy)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD node -e "require('http').get('http://localhost:3000/health', (r) => {process.exit(r.statusCode === 200 ? 0 : 1)})"

# Start the application
CMD ["node", "index.js"]