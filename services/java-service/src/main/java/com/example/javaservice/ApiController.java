package com.example.javaservice;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import java.util.HashMap;
import java.util.Map;

@RestController
public class ApiController {
    
    @GetMapping("/api/hello")
    public Map<String, String> hello() {
        Map<String, String> response = new HashMap<>();
        response.put("message", "Hello from Java Service!");
        response.put("service", "java-service");
        response.put("version", "1.0.0");
        return response;
    }
    
    @GetMapping("/api/status")
    public Map<String, Object> status() {
        Map<String, Object> response = new HashMap<>();
        response.put("status", "running");
        response.put("uptime", System.currentTimeMillis());
        response.put("javaVersion", System.getProperty("java.version"));
        return response;
    }
}