# @mediar/terminator-workflow

Workflow builder for Terminator automation that provides a parseable structure for mediar-app.

## Installation

```bash
npm install @mediar/terminator-workflow @mediar/terminator
```

## Usage

This package helps you create workflows with a clean, parseable structure. Each step contains regular terminator.js SDK code:

```typescript
import { Desktop } from '@mediar/terminator';
import { defineWorkflow, executeWorkflow } from '@mediar/terminator-workflow';

// Define workflow with parseable structure
const workflow = defineWorkflow('notepad-hello', 'Notepad Hello World')
  .description('Opens Notepad and types a greeting')
  .version('1.0.0')
  .variable('userName', {
    type: 'string',
    label: 'User Name',
    description: 'Name to greet',
    default: 'World'
  })
  .step('open-notepad', 'Open Notepad', async (desktop, context) => {
    // Regular terminator.js SDK code here
    desktop.openApplication('notepad');
    await new Promise(r => setTimeout(r, 2000));
  })
  .step('type-greeting', 'Type Greeting', async (desktop, context) => {
    // Access variables from context
    const { userName } = context.variables;

    // Use regular SDK methods
    const textbox = desktop.locator('role:Edit');
    await textbox.type(`Hello, ${userName}!\\n`);
    await textbox.type(`This is a structured workflow.\\n`);
  })
  .build();

// Export for mediar-app parsing
export const workflowDefinition = workflow.metadata;

// Execute the workflow
async function main() {
  const desktop = new Desktop();
  await executeWorkflow(workflow, desktop, { userName: 'Claude' });
}

if (require.main === module) {
  main().catch(console.error);
}
```

## Key Features

- **Parseable Structure**: mediar-app can parse the workflow steps and metadata
- **Type Safety**: Full TypeScript support with type checking
- **Regular SDK Code**: Use normal `desktop.locator()`, `desktop.openApplication()`, etc. inside steps
- **Variables**: Define typed variables with UI metadata
- **Context**: Share state between steps via the context object

## API

### `defineWorkflow(id, name)`

Creates a new workflow builder.

### Builder Methods

- `.description(desc)` - Set workflow description
- `.version(ver)` - Set version string
- `.variable(name, definition)` - Add a variable definition
- `.tags(...tags)` - Add tags for categorization
- `.step(id, name, execute, description?)` - Add a step with executable code
- `.build()` - Build the final workflow object
- `.export()` - Export in mediar-app parseable format

### `executeWorkflow(workflow, desktop, variables)`

Execute a workflow with the given Desktop instance and variable values.

## Step Execution

Each step receives:
- `desktop`: The Desktop instance for using terminator.js SDK
- `context`: Contains `variables` and shared `state` object

```typescript
.step('my-step', 'My Step', async (desktop, context) => {
  // Access variables
  const myVar = context.variables.myVar;

  // Use terminator.js SDK
  const button = desktop.locator('role:Button|name:Submit');
  await button.click();

  // Store state for next steps
  context.state.clickedButton = true;
})
```

## Differences from Raw Workflows

**Before** (raw structure):
```typescript
export const workflow = {
  id: 'my-workflow',
  name: 'My Workflow',
  steps: [
    { tool_name: 'open_app', arguments: { app_name: 'notepad' } }
  ]
};
```

**After** (with workflow builder):
```typescript
const workflow = defineWorkflow('my-workflow', 'My Workflow')
  .step('open-app', 'Open Application', async (desktop, context) => {
    desktop.openApplication('notepad');
  })
  .build();
```

The new approach:
- Provides better structure and type safety
- Makes steps easily parseable by mediar-app
- Uses actual SDK code instead of tool call objects
- Supports variables and context sharing
