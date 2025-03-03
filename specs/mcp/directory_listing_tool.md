# MCP Directory Listing Tool Specification

## Overview
The Directory Listing Tool is an MCP-compliant tool that provides secure directory listing capabilities. It implements strict security controls and access restrictions to prevent unauthorized access to sensitive file system information.

## Tool Definition

### 1. Tool Metadata
```json
{
    "tool_id": "directory_listing",
    "version": "1.0.0",
    "description": "Secure directory listing tool with access controls",
    "capabilities": ["list_directory", "filter_contents"],
    "security_level": "high",
    "required_permissions": ["read_directory"]
}
```

### 2. Input Parameters
```json
{
    "path": "string",           // Target directory path
    "recursive": "boolean",     // Whether to list recursively
    "filter": {                 // Optional filters
        "pattern": "string",    // File pattern matching
        "exclude": ["string"],  // Patterns to exclude
        "max_depth": "integer"  // Maximum recursion depth
    },
    "format": "string",         // Output format (json/text)
    "include_metadata": "boolean" // Whether to include file metadata
}
```

### 3. Output Format
```json
{
    "status": "success|error",
    "data": {
        "entries": [
            {
                "name": "string",
                "type": "file|directory|symlink",
                "size": "integer",
                "permissions": "string",
                "modified": "timestamp",
                "metadata": {
                    "owner": "string",
                    "group": "string",
                    "inode": "integer"
                }
            }
        ],
        "summary": {
            "total_files": "integer",
            "total_dirs": "integer",
            "total_size": "integer"
        }
    },
    "error": {
        "code": "string",
        "message": "string",
        "details": "object"
    }
}
```

## Security Implementation

### 1. Access Controls
- Path validation and sanitization
- Directory traversal prevention
- Permission checking
- User context validation
- Resource limits

### 2. Input Validation
- Path normalization
- Pattern validation
- Size limits
- Character encoding validation
- Malicious input detection

### 3. Output Sanitization
- Sensitive data filtering
- Path normalization
- Size limits
- Character encoding
- XSS prevention

### 4. Resource Management
- Memory limits
- CPU time limits
- File descriptor limits
- Rate limiting
- Concurrent request limits

## LLM System Prompt

```
You are a secure directory listing tool that provides controlled access to file system information. Your primary responsibilities are:

1. Security:
   - Validate and sanitize all input paths
   - Prevent directory traversal attacks
   - Enforce access controls and permissions
   - Filter sensitive information
   - Implement rate limiting

2. Functionality:
   - List directory contents with specified filters
   - Provide file metadata when requested
   - Support recursive listing with depth control
   - Format output according to specified requirements
   - Handle errors gracefully

3. Constraints:
   - Never expose sensitive file information
   - Respect file system permissions
   - Limit resource usage
   - Validate all input parameters
   - Sanitize all output

4. Error Handling:
   - Provide clear error messages
   - Log security-related events
   - Handle edge cases gracefully
   - Implement proper error recovery
   - Maintain audit trail

When processing requests:
1. Validate the user's permissions
2. Sanitize and validate the input path
3. Apply any specified filters
4. Generate the directory listing
5. Sanitize the output
6. Return the results in the requested format

Always prioritize security over functionality. If there's any doubt about the security implications of an operation, deny the request and provide a clear explanation.
```

## Usage Examples

### 1. Basic Directory Listing
```json
{
    "path": "/home/user/documents",
    "recursive": false,
    "format": "json"
}
```

### 2. Recursive Listing with Filters
```json
{
    "path": "/home/user/projects",
    "recursive": true,
    "filter": {
        "pattern": "*.rs",
        "exclude": ["target/", "node_modules/"],
        "max_depth": 3
    },
    "include_metadata": true,
    "format": "json"
}
```

## Error Handling

### 1. Common Error Codes
- `EACCES`: Permission denied
- `ENOENT`: Directory not found
- `ENOTDIR`: Not a directory
- `ELOOP`: Too many symbolic links
- `ENAMETOOLONG`: Path too long
- `EINVAL`: Invalid parameters

### 2. Error Response Format
```json
{
    "status": "error",
    "error": {
        "code": "EACCES",
        "message": "Permission denied",
        "details": {
            "path": "/restricted/path",
            "user": "username",
            "required_permissions": ["read"]
        }
    }
}
```

## Monitoring and Logging

### 1. Security Events
- Access attempts
- Permission violations
- Input validation failures
- Resource limit exceeded
- Error conditions

### 2. Performance Metrics
- Response times
- Resource usage
- Cache hit rates
- Error rates
- Request patterns

## Testing Requirements

### 1. Security Tests
- Path traversal attempts
- Permission bypass attempts
- Resource exhaustion
- Input validation
- Output sanitization

### 2. Functional Tests
- Basic listing
- Recursive listing
- Filtering
- Format conversion
- Error handling

### 3. Performance Tests
- Response time
- Resource usage
- Concurrent requests
- Large directories
- Memory usage 