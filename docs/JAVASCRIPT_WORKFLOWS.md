# JavaScript Workflows

JavaScript workflows allow you to define entire workflows as JavaScript/TypeScript files, providing better IDE support, type safety, and code reusability while maintaining parseability for the mediar-app desktop application.

## Overview

A JavaScript workflow is a `.js` or `.ts` file that exports a workflow definition using a standardized structure. The workflow can still include a `terminator.yml` file in the same directory for metadata.

## Structure

### Basic Example

```javascript
// workflow.js

export const workflow = {
  id: 'my-workflow',
  name: 'My Workflow',
  description: 'Example workflow',

  // Optional metadata
  timeout: 60000,
  version: '1.0.0',

  // Variables definition (like YAML workflows)
  variables: {
    userName: {
      type: 'string',
      label: 'User Name',
      default: 'John Doe'
    }
  },

  // Initial input values
  inputs: {
    userName: 'Jane Smith'
  },

  // Reusable selectors
  selectors: {
    submitButton: 'role:Button|name:Submit'
  },

  // Steps array - the core of the workflow
  steps: [
    {
      id: 'step1',
      tool_name: 'open_application',
      arguments: {
        app_name: 'notepad'
      },
      delay_ms: 2000
    },
    {
      id: 'step2',
      tool_name: 'type_into_element',
      arguments: {
        selector: 'role:Edit',
        text: '${{ inputs.userName }}'
      }
    }
  ]
};

// Optional: Export metadata separately
export const metadata = {
  author: 'Your Name',
  createdAt: '2025-01-15',
  tags: ['automation', 'notepad']
};
```

### Advanced: Step Factory Functions

For better code reusability and type safety:

```javascript
// workflow.js

// Helper function to create steps
function createStep(id, toolName, args, options = {}) {
  return {
    id,
    tool_name: toolName,
    arguments: args,
    ...options
  };
}

// Reusable step creators
const openApp = (appName, delay = 2000) =>
  createStep('open-app', 'open_application', { app_name: appName }, { delay_ms: delay });

const typeText = (selector, text) =>
  createStep('type-text', 'type_into_element', { selector, text });

const clickElement = (selector) =>
  createStep('click', 'click_element', { selector });

export const workflow = {
  id: 'notepad-workflow',
  name: 'Notepad Automation',
  steps: [
    openApp('notepad', 3000),
    typeText('role:Edit', 'Hello from JavaScript workflow!'),
    clickElement('role:Button|name:Save')
  ]
};
```

### Mastra-Style Pattern (Alternative)

For developers familiar with Mastra AI:

```javascript
// workflow.js

export const workflow = {
  id: 'mastra-style-workflow',
  name: 'Mastra-Style Workflow',

  inputSchema: {
    type: 'object',
    properties: {
      userName: { type: 'string' }
    }
  },

  steps: [
    {
      id: 'step1',
      name: 'Open Application',
      inputSchema: {},
      outputSchema: {
        type: 'object',
        properties: {
          success: { type: 'boolean' }
        }
      },
      execute: {
        tool_name: 'open_application',
        arguments: {
          app_name: 'notepad'
        }
      }
    }
  ]
};
```

### Inngest-Style Pattern (Alternative)

For developers familiar with Inngest:

```javascript
// workflow.js

export const workflow = {
  id: 'inngest-style-workflow',
  name: 'Inngest-Style Workflow',

  trigger: {
    event: 'user.signup'
  },

  steps: [
    {
      id: 'send-email',
      run: {
        tool_name: 'run_command',
        arguments: {
          engine: 'javascript',
          run: `
            console.log('Sending welcome email...');
            return { success: true };
          `
        }
      }
    }
  ]
};
```

## TypeScript Support

For TypeScript users, you can add type definitions:

```typescript
// workflow.ts

import type { Workflow, Step, VariableDefinition } from '@terminator/types';

interface WorkflowInputs {
  userName: string;
  userEmail: string;
}

export const workflow: Workflow<WorkflowInputs> = {
  id: 'typed-workflow',
  name: 'Typed Workflow',

  variables: {
    userName: {
      type: 'string',
      label: 'User Name',
      default: 'John Doe'
    } as VariableDefinition,
    userEmail: {
      type: 'string',
      label: 'Email Address',
      default: 'user@example.com'
    } as VariableDefinition
  },

  steps: [
    {
      id: 'step1',
      tool_name: 'open_application',
      arguments: {
        app_name: 'chrome'
      }
    }
  ]
};
```

## Metadata File (terminator.yml)

You can still include a `terminator.yml` file alongside your JavaScript workflow for additional metadata:

```yaml
# terminator.yml
name: My JavaScript Workflow
description: A workflow defined in JavaScript
version: "1.0.0"
author: Your Name
tags:
  - automation
  - javascript
created_at: "2025-01-15"
```

## Directory Structure

```
my-workflow/
├── workflow.js          # Main workflow definition
├── terminator.yml       # Optional metadata
├── helpers.js           # Optional helper functions
└── README.md            # Optional documentation
```

## Parsing by mediar-app

The mediar-app will parse JavaScript workflows using a safe JavaScript parser that extracts the exported workflow object structure. It looks for:

1. **Export patterns**: `export const workflow = {...}` or `module.exports = {...}`
2. **Step array**: Extracts the `steps` array
3. **Metadata**: Combines JS exports with `terminator.yml` if present
4. **Variables and selectors**: Extracts for UI display

The parser supports:
- Object literals
- Template strings with variable references
- Function calls that return step objects
- Comments for documentation

## Execution Flow

1. **Discovery**: Terminator finds `.js`/`.ts` files in workflow directories
2. **Parsing**: JavaScript is parsed to extract workflow structure
3. **Validation**: Validates against workflow schema
4. **Execution**: Steps are executed like YAML workflows
5. **Result**: Same output structure as YAML workflows

## Best Practices

1. **Keep it declarative**: Avoid complex logic in workflow definitions
2. **Use helper functions**: Extract reusable patterns
3. **Add comments**: Document complex steps
4. **Type definitions**: Use TypeScript for better IDE support
5. **Test locally**: Validate workflow structure before deployment

## Advantages Over YAML

- **IDE support**: Autocomplete, syntax highlighting, refactoring
- **Type safety**: Catch errors before runtime (TypeScript)
- **Code reuse**: Share helper functions across workflows
- **Version control**: Better diff viewing for changes
- **Modularity**: Import/export between workflow files
- **Validation**: Lint and format with standard JavaScript tools

## Limitations

- Must maintain parseable structure (no dynamic runtime generation)
- Complex logic should be in `run_command` steps, not workflow definition
- Export must be statically analyzable for mediar-app parsing
