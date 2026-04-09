// Load Express (a web server framework for Node.js)
const express = require('express');
const app = express();
const port = 3000;

// Middleware to parse JSON requests
app.use(express.json());

// Health check endpoint (Kubernetes uses this to know if the app is alive)
app.get('/health', (req, res) => {
  res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

// Readiness check (Kubernetes uses this to know if the app is ready for traffic)
app.get('/ready', (req, res) => {
  res.json({ status: 'ready', timestamp: new Date().toISOString() });
});

// Main API endpoint
app.get('/api/hello', (req, res) => {
  res.json({ 
    message: 'Hello from JavaScript API!', 
    service: 'javascript-api',
    version: '1.0.0'
  });
});

// Start the server
app.listen(port, '0.0.0.0', () => {
  console.log(`JavaScript API listening on port ${port}`);
});