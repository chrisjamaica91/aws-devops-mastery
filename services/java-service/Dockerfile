# Stage 1: Build stage (compile Java code with Maven)
FROM maven:3.9-eclipse-temurin-17 AS builder

WORKDIR /app

# Copy pom.xml first (Docker layer caching - if pom.xml unchanged, reuse dependencies)
COPY pom.xml ./

# Download dependencies (cached layer if pom.xml doesn't change)
RUN mvn dependency:go-offline -B

# Copy source code
COPY src ./src

# Build the application (creates target/*.jar)
RUN mvn clean package -DskipTests -B

# Debug: List target directory contents
RUN echo "=== Contents of /app/target/ ===" && ls -la /app/target/

# Stage 2: Runtime stage (lightweight JRE, no Maven)
FROM eclipse-temurin:17-jre-alpine

# Run as non-root user for security
RUN addgroup -g 1001 -S spring && adduser -S spring -u 1001

WORKDIR /app

# Copy JAR file - use wildcard to match whatever Maven created
COPY --from=builder --chown=spring:spring /app/target/*.jar app.jar

# Switch to non-root user
USER spring

# Expose port 8080
EXPOSE 8080

# Health check (Docker can restart container if unhealthy)
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/actuator/health/liveness || exit 1

# JVM optimization flags for containers
ENV JAVA_OPTS="-XX:+UseContainerSupport -XX:MaxRAMPercentage=75.0 -XX:InitialRAMPercentage=50.0"

# Start the application
ENTRYPOINT ["sh", "-c", "java $JAVA_OPTS -jar app.jar"]