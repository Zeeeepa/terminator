# JavaScript Workflows

JavaScript workflows allow you to write automation scripts using the **terminator.js SDK** with a standardized, parseable structure. This gives you the power of programmatic control with full IDE support, while allowing the mediar-app to parse and display workflow steps.

## Overview

Instead of defining workflows as YAML with MCP tool calls, JavaScript workflows use the terminator.js SDK directly but wrap the code in parseable step structures inspired by frameworks like Mastra AI and Inngest.

## Why JavaScript Workflows?

- **Full SDK Access**: Use `Desktop`, `locator()`, `click()`, `type()`, etc. directly
- **IDE Support**: Autocomplete, type checking, refactoring
- **Code Reusability**: Share helper functions across workflows
- **Framework Familiar**: Mastra AI / Inngest-style patterns
- **Parseable**: mediar-app can extract and display steps

## Basic Structure

### Simple Workflow (Mastra-style)

```javascript
const { Desktop } = require('@mediar/terminator');
const { createWorkflow, createStep } = require('@mediar/workflow');

// Define reusable steps
const openNotepad = createStep({
  id: 'open-notepad',
  description: 'Open Notepad application',
  execute: async ({ desktop }) => {
    console.log('🚀 Opening Notepad...');
    const app = desktop.openApplication('notepad');
    await new Promise(r => setTimeout(r, 2000));
    return { app };
  }
});

const typeMessage = createStep({
  id: 'type-message',
  description: 'Type a message into Notepad',
  execute: async ({ desktop, context }) => {
    console.log('⌨️  Typing message...');
    const textbox = desktop.locator('role:Edit');
    await textbox.type('Hello from JavaScript workflow!');
    return { success: true };
  }
});

// Define the workflow
export const workflow = createWorkflow({
  id: 'simple-notepad',
  name: 'Simple Notepad Automation',
  description: 'Opens Notepad and types a message'
})
  .then(openNotepad)
  .then(typeMessage)
  .commit();

// Execute the workflow
if (require.main === module) {
  const desktop = new Desktop();
  workflow.run({ desktop }).then(result => {
    console.log('✅ Workflow completed:', result);
  });
}
```

### Inngest-style Workflow

```javascript
const { Desktop } = require('@mediar/terminator');
const { Inngest } = require('inngest');

const inngest = new Inngest({ id: 'terminator-automation' });

export default inngest.createFunction(
  { id: 'notepad-automation' },
  { event: 'automation/notepad' },
  async ({ event, step }) => {
    const desktop = new Desktop();

    // Step 1: Open application
    await step.run('open-notepad', async () => {
      console.log('🚀 Opening Notepad...');
      desktop.openApplication('notepad');
      await new Promise(r => setTimeout(r, 2000));
      return { status: 'opened' };
    });

    // Step 2: Type text
    const result = await step.run('type-message', async () => {
      console.log('⌨️  Typing message...');
      const textbox = desktop.locator('role:Edit');
      await textbox.type('Hello from Inngest workflow!');
      return { charactersTyped: 29 };
    });

    // Step 3: Take screenshot
    await step.run('take-screenshot', async () => {
      console.log('📸 Taking screenshot...');
      const screenshot = desktop.screenshot();
      return { width: screenshot.width, height: screenshot.height };
    });

    return { success: true, result };
  }
);
```

### Inline Steps (Quick and Simple)

For simpler workflows, you can define steps inline:

```javascript
const { Desktop } = require('@mediar/terminator');

export const workflow = {
  id: 'quick-automation',
  name: 'Quick Automation',

  async execute() {
    const desktop = new Desktop();
    const steps = [];

    // Step 1
    steps.push({
      name: 'Open Calculator',
      run: async () => {
        desktop.openApplication('calc');
        await new Promise(r => setTimeout(r, 2000));
      }
    });

    // Step 2
    steps.push({
      name: 'Calculate 7 + 3',
      run: async () => {
        await desktop.locator('name:Seven').click();
        await desktop.locator('name:Plus').click();
        await desktop.locator('name:Three').click();
        await desktop.locator('name:Equals').click();
      }
    });

    // Execute all steps
    for (const step of steps) {
      console.log(`▶️  ${step.name}`);
      await step.run();
      console.log(`✅ ${step.name} completed`);
    }
  }
};

if (require.main === module) {
  workflow.execute();
}
```

## Parseable Structure for mediar-app

The mediar-app can parse these workflows by looking for:

### 1. Mastra Pattern
```javascript
createWorkflow({ id, name, description })
  .then(createStep({ id, description, execute }))
  .commit()
```

### 2. Inngest Pattern
```javascript
inngest.createFunction(
  { id },
  { event },
  async ({ step }) => {
    await step.run('step-id', async () => { ... });
  }
)
```

### 3. Simple Object Pattern
```javascript
export const workflow = {
  id, name, description,
  steps: [
    { id, name, description, run: async () => { ... } }
  ],
  execute: async () => { ... }
}
```

## Full SDK Example

Here's a comprehensive example using the terminator.js SDK:

```javascript
const { Desktop } = require('@mediar/terminator');

async function automateWebForm() {
  const desktop = new Desktop();

  console.log('Step 1: Open Browser');
  desktop.openUrl('https://example.com/form', 'Chrome');
  await new Promise(r => setTimeout(r, 3000));

  console.log('Step 2: Fill Form Fields');
  await desktop.locator('role:textbox|name:First Name').type('John');
  await desktop.locator('role:textbox|name:Last Name').type('Doe');
  await desktop.locator('role:textbox|name:Email').type('john@example.com');

  console.log('Step 3: Select Dropdown');
  await desktop.locator('role:combobox|name:Country').click();
  await desktop.locator('role:option|name:United States').click();

  console.log('Step 4: Check Agreement');
  await desktop.locator('role:checkbox|name:I agree').click();

  console.log('Step 5: Submit Form');
  await desktop.locator('role:button|name:Submit').click();

  console.log('Step 6: Verify Success');
  await desktop.locator('text:Thank you').waitFor({ timeout: 5000 });

  console.log('✅ Form submission completed!');
}

// Export as workflow for parsing
export const workflow = {
  id: 'web-form-automation',
  name: 'Web Form Automation',
  description: 'Automates filling and submitting a web form',
  execute: automateWebForm
};

if (require.main === module) {
  automateWebForm().catch(console.error);
}
```

## Advanced: Variables and Dynamic Data

```javascript
const { Desktop } = require('@mediar/terminator');

export const workflow = {
  id: 'dynamic-workflow',
  name: 'Dynamic Data Entry',

  // Define variables
  variables: {
    companyName: { type: 'string', default: 'Acme Corp' },
    entries: { type: 'array', default: [] }
  },

  async execute({ companyName, entries }) {
    const desktop = new Desktop();

    console.log(`Processing ${entries.length} entries for ${companyName}`);

    for (const [index, entry] of entries.entries()) {
      console.log(`Step ${index + 1}: Processing ${entry.name}`);

      // Custom logic using SDK
      await desktop.locator('role:textbox|name:Company').fill(companyName);
      await desktop.locator('role:textbox|name:Name').fill(entry.name);
      await desktop.locator('role:textbox|name:Amount').fill(entry.amount);
      await desktop.locator('role:button|name:Add Entry').click();

      await new Promise(r => setTimeout(r, 1000));
    }

    console.log('✅ All entries processed');
  }
};
```

## Best Practices

1. **Use Console Logs with Emojis**: Makes it easy for mediar-app to parse step execution
2. **Consistent Step IDs**: Use kebab-case for step identifiers
3. **Error Handling**: Wrap steps in try-catch for better error messages
4. **Timeouts**: Use `setTimeout` or SDK's built-in wait methods
5. **Return Values**: Return meaningful data from steps for debugging

## Execution

### From CLI
```bash
node workflow.js
```

### From Terminator CLI
```bash
terminator-cli execute-workflow --url file://./workflow.js
```

### From mediar-app
Select the `.js` workflow file in the workflow list.

## Metadata File (Optional)

You can still include a `terminator.yml` for additional metadata:

```yaml
name: My JavaScript Workflow
description: Detailed description here
author: Your Name
version: "1.0.0"
tags:
  - automation
  - javascript
```

## TypeScript Support

For TypeScript workflows, use `.ts` extension:

```typescript
import { Desktop } from '@mediar/terminator';
import { createWorkflow, createStep, WorkflowContext } from '@mediar/workflow';

interface WorkflowInput {
  userName: string;
  email: string;
}

const fillUserForm = createStep<WorkflowInput>({
  id: 'fill-user-form',
  description: 'Fill user information form',
  execute: async ({ desktop, input }) => {
    await desktop.locator('role:textbox|name:Name').type(input.userName);
    await desktop.locator('role:textbox|name:Email').type(input.email);
    return { success: true };
  }
});

export const workflow = createWorkflow<WorkflowInput>({
  id: 'user-form-workflow',
  name: 'User Form Workflow'
})
  .then(fillUserForm)
  .commit();
```

## Comparison with YAML Workflows

| Feature | YAML Workflows | JavaScript Workflows |
|---------|---------------|---------------------|
| Syntax | YAML with MCP tools | JavaScript with SDK |
| IDE Support | Limited | Full autocomplete |
| Code Reuse | Include other files | Import/export modules |
| Type Safety | No | Yes (TypeScript) |
| Debugging | Harder | Easier (breakpoints) |
| mediar-app Parsing | Native | Via structure patterns |
| Learning Curve | Lower | Medium |

## Framework Compatibility

- **Mastra AI**: Use `createWorkflow` and `createStep` patterns
- **Inngest**: Use `inngest.createFunction` with `step.run()`
- **Custom**: Define your own patterns following the parseable structure guidelines

This approach gives you the best of both worlds: programmatic power and visual parseability!