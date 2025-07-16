# CBC Integration

Capability Binding Config (CBC) - Support for dynamic provider discovery for AI MCP clients.

## Project Overview

This project implements a standardized Capability Binding Config (CBC) feature that allows AI MCP clients to:

1. Use a single YAML mapping file to define static providers (Redmine, Jira, filesystem, etc.)
2. Dynamically discover additional back-ends via MCP server features:
   - `roots/list` for filesystem workspaces
   - `tools/list` for available tool invocations
   - `resources/list` for contextual data blobs

## Related Resources

- Redmine Project: CBC Integration (ID: 327)
- GitHub Repository: [zacharyelston/CBC](https://github.com/zacharyelston/CBC)