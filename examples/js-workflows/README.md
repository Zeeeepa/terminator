# JavaScript Workflow Examples

This directory contains example JavaScript workflows demonstrating various patterns and approaches.

## Examples

### 1. Simple Notepad (`simple-notepad.js`)
Basic JavaScript workflow that opens Notepad and types text. Great starting point for understanding the structure.

### 2. With Variables (`with-variables.js`)
Demonstrates how to define and use variables in JavaScript workflows, similar to YAML workflows.

### 3. With Helpers (`with-helpers.js`)
Shows how to use helper functions for better code reusability and readability. This pattern is especially useful for complex workflows.

### 4. Mastra-Style (`mastra-style.js`)
Compatible with Mastra AI workflow patterns, including input/output schemas and step definitions.

### 5. Inngest-Style (`inngest-style.js`)
Following Inngest workflow patterns with event triggers and run configurations.

## Running JavaScript Workflows

JavaScript workflows can be executed just like YAML workflows:

```bash
# Via MCP
terminator-cli execute-workflow --url file://./workflows/js-examples/simple-notepad.js

# Or from the mediar-app desktop application
```

## Advantages

- **IDE Support**: Full autocomplete, syntax highlighting, and refactoring
- **Type Safety**: Use TypeScript for compile-time error checking
- **Code Reuse**: Share helper functions across workflows
- **Better Diffing**: Easier to see changes in version control
- **Modularity**: Import/export between workflow files

## Best Practices

1. Keep workflow definitions declarative
2. Use helper functions for repeated patterns
3. Add comments to explain complex logic
4. Consider TypeScript for larger workflows
5. Test locally before deployment
