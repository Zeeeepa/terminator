# JavaScript/TypeScript Workflows

Write automation workflows using the **terminator.js SDK** in TypeScript or JavaScript. This gives you full programmatic control with IDE autocomplete and type safety, while allowing mediar-app to parse and display your workflow steps.

## Overview

JavaScript/TypeScript workflows are **regular Node.js/TypeScript scripts** that use the terminator.js SDK directly. They export metadata for mediar-app to parse and display steps in the UI.

**Key Points:**
- ✅ Use TypeScript by default for type safety
- ✅ Import from `@mediar/terminator` package
- ✅ Write normal async/await code with SDK methods
- ✅ Export workflow metadata for mediar-app parsing
- ✅ Run directly with `tsx` or `node`
- ✅ Compatible with Mastra AI / Inngest patterns

## Quick Start

### 1. Install Dependencies

```bash
npm install @mediar/terminator
npm install -D typescript tsx @types/node
```

### 2. Create a Workflow (TypeScript)

```typescript
// workflow.ts
import { Desktop } from '@mediar/terminator';

// Export metadata for mediar-app
export const workflow = {
  id: 'my-workflow',
  name: 'My First Workflow',
  description: 'Automates a simple task',
};

// Main workflow logic
async function main() {
  const desktop = new Desktop();

  console.log('🚀 Step 1: Opening Notepad');
  desktop.openApplication('notepad');
  await new Promise(r => setTimeout(r, 2000));

  console.log('⌨️  Step 2: Typing message');
  const textbox = desktop.locator('role:Edit');
  await textbox.type('Hello from TypeScript workflow!');

  console.log('✅ Workflow completed');
}

// Execute
if (require.main === module) {
  main().catch(console.error);
}
```

### 3. Run It

```bash
tsx workflow.ts
```

## Parseable Structure for mediar-app

The mediar-app parses the exported `workflow` object to display steps in the UI:

```typescript
export const workflow = {
  id: 'unique-id',           // Required: unique identifier
  name: 'Display Name',      // Required: human-readable name
  description: 'What it does', // Optional: description
  version: '1.0.0',          // Optional: version

  // Optional: Define variables for UI inputs
  variables: {
    userName: {
      type: 'string',
      label: 'User Name',
      default: 'John Doe'
    }
  },

  // Optional: Explicitly list steps for better parsing
  steps: [
    {
      id: 'step-1',
      name: 'Open Application',
      description: 'Opens the target app'
    },
    {
      id: 'step-2',
      name: 'Fill Form',
      description: 'Fills in the form fields'
    }
  ]
};
```

## Framework Patterns

### Mastra AI Style

```typescript
import { Desktop } from '@mediar/terminator';
import { createWorkflow, createStep } from '@mastra/core/workflows';
import { z } from 'zod';

const openNotepad = createStep({
  id: 'open-notepad',
  description: 'Open Notepad application',
  inputSchema: z.object({}),
  outputSchema: z.object({ success: z.boolean() }),
  execute: async () => {
    const desktop = new Desktop();
    console.log('🚀 Opening Notepad...');
    desktop.openApplication('notepad');
    await new Promise(r => setTimeout(r, 2000));
    return { success: true };
  }
});

const typeMessage = createStep({
  id: 'type-message',
  description: 'Type message',
  inputSchema: z.object({ success: z.boolean() }),
  outputSchema: z.object({ length: z.number() }),
  execute: async () => {
    const desktop = new Desktop();
    const textbox = desktop.locator('role:Edit');
    const msg = 'Hello from Mastra!';
    await textbox.type(msg);
    return { length: msg.length };
  }
});

export const workflow = createWorkflow({
  id: 'mastra-notepad',
  name: 'Mastra Notepad Workflow',
  inputSchema: z.object({}),
  outputSchema: z.object({ length: z.number() })
})
  .then(openNotepad)
  .then(typeMessage)
  .commit();
```

### Inngest Style

```typescript
import { Desktop } from '@mediar/terminator';
import { Inngest } from 'inngest';

const inngest = new Inngest({ id: 'terminator-automation' });

export default inngest.createFunction(
  { id: 'notepad-automation' },
  { event: 'automation/notepad' },
  async ({ event, step }) => {
    const desktop = new Desktop();

    await step.run('open-notepad', async () => {
      console.log('🚀 Opening Notepad');
      desktop.openApplication('notepad');
      await new Promise(r => setTimeout(r, 2000));
      return { status: 'opened' };
    });

    const result = await step.run('type-message', async () => {
      console.log('⌨️  Typing message');
      const textbox = desktop.locator('role:Edit');
      await textbox.type('Hello from Inngest!');
      return { length: 17 };
    });

    await step.run('take-screenshot', async () => {
      console.log('📸 Screenshot');
      return desktop.screenshot();
    });

    return { success: true, result };
  }
);
```

## Full SDK Example

```typescript
import { Desktop } from '@mediar/terminator';

export const workflow = {
  id: 'web-form-automation',
  name: 'Web Form Automation',
  description: 'Fill and submit a web form',
};

async function main() {
  const desktop = new Desktop();

  // Step 1: Open browser
  console.log('🌐 Step 1: Opening browser');
  desktop.openUrl('https://example.com/form', 'Chrome');
  await new Promise(r => setTimeout(r, 3000));

  // Step 2: Fill form
  console.log('📝 Step 2: Filling form');
  await desktop.locator('role:textbox|name:First Name').type('John');
  await desktop.locator('role:textbox|name:Last Name').type('Doe');
  await desktop.locator('role:textbox|name:Email').type('john@example.com');

  // Step 3: Select dropdown
  console.log('🔽 Step 3: Selecting country');
  await desktop.locator('role:combobox|name:Country').click();
  await desktop.locator('role:option|name:United States').click();

  // Step 4: Submit
  console.log('✅ Step 4: Submitting');
  await desktop.locator('role:button|name:Submit').click();

  // Step 5: Verify
  console.log('🔍 Step 5: Verifying success');
  await desktop.locator('text:Thank you').waitFor({ timeout: 5000 });

  console.log('✅ Form submitted successfully!');
}

if (require.main === module) {
  main().catch(console.error);
}
```

## With Variables

```typescript
import { Desktop } from '@mediar/terminator';

export const workflow = {
  id: 'dynamic-workflow',
  name: 'Dynamic Data Entry',

  variables: {
    companyName: { type: 'string', default: 'Acme Corp' },
    entries: { type: 'array', default: [] }
  }
};

interface Entry {
  name: string;
  amount: string;
}

async function main(params: { companyName?: string; entries?: Entry[] } = {}) {
  const { companyName = 'Acme Corp', entries = [] } = params;
  const desktop = new Desktop();

  console.log(`📊 Processing ${entries.length} entries for ${companyName}`);

  for (const [index, entry] of entries.entries()) {
    console.log(`▶️  Step ${index + 1}: ${entry.name}`);

    await desktop.locator('role:textbox|name:Company').fill(companyName);
    await desktop.locator('role:textbox|name:Name').fill(entry.name);
    await desktop.locator('role:textbox|name:Amount').fill(entry.amount);
    await desktop.locator('role:button|name:Add').click();

    await new Promise(r => setTimeout(r, 1000));
  }

  console.log('✅ All entries processed');
}

if (require.main === module) {
  main({
    companyName: 'My Company',
    entries: [
      { name: 'Item 1', amount: '100' },
      { name: 'Item 2', amount: '200' }
    ]
  }).catch(console.error);
}
```

## Helper Functions Pattern

```typescript
import { Desktop } from '@mediar/terminator';

export const workflow = {
  id: 'workflow-with-helpers',
  name: 'Workflow with Helpers',
};

class WorkflowHelpers {
  constructor(private desktop: Desktop) {}

  async openApp(appName: string, waitMs = 2000) {
    console.log(`🚀 Opening ${appName}...`);
    this.desktop.openApplication(appName);
    await this.wait(waitMs);
  }

  async typeInto(selector: string, text: string) {
    console.log(`⌨️  Typing into ${selector}`);
    await this.desktop.locator(selector).type(text);
  }

  async click(selector: string) {
    console.log(`👆 Clicking ${selector}`);
    await this.desktop.locator(selector).click();
  }

  async wait(ms: number) {
    await new Promise(r => setTimeout(r, ms));
  }

  async screenshot(desc = 'screenshot') {
    console.log(`📸 Taking ${desc}`);
    return this.desktop.screenshot();
  }
}

async function main() {
  const desktop = new Desktop();
  const helpers = new WorkflowHelpers(desktop);

  await helpers.openApp('notepad', 3000);
  await helpers.typeInto('role:Edit', 'Hello with helpers!');
  await helpers.screenshot('result');

  console.log('✅ Done');
}

if (require.main === module) {
  main().catch(console.error);
}
```

## Best Practices

1. **Use TypeScript** - Get autocomplete and type safety
2. **Console logs with emojis** - Makes step execution visible and parseable
3. **Export workflow metadata** - Required for mediar-app UI
4. **Error handling** - Wrap in try-catch for better debugging
5. **Async/await** - Use promises for all SDK calls
6. **Helper classes** - Reuse common patterns across workflows

## Execution

### From CLI
```bash
# TypeScript
tsx workflow.ts

# JavaScript
node workflow.js

# With Terminator CLI
terminator-cli execute-workflow --url file://./workflow.ts
```

### From mediar-app
Select the `.ts` or `.js` file in the workflow list - it will execute automatically.

## Metadata File (Optional)

Add `terminator.yml` alongside your workflow:

```yaml
name: My TypeScript Workflow
description: Detailed description
author: Your Name
version: "1.0.0"
tags:
  - automation
  - typescript
```

## Comparison with YAML Workflows

| Feature | YAML Workflows | TypeScript Workflows |
|---------|---------------|---------------------|
| Syntax | YAML + MCP tools | TypeScript + SDK |
| IDE Support | Limited | Full autocomplete |
| Code Reuse | Limited | Full module system |
| Type Safety | No | Yes |
| Debugging | Harder | Easy (breakpoints) |
| Learning Curve | Lower | Medium |
| Power | Limited | Full programmatic control |

## TypeScript SDK Reference

```typescript
import { Desktop, Element, Locator } from '@mediar/terminator';

const desktop = new Desktop();

// Applications
desktop.openApplication('notepad');
desktop.application('Chrome');
desktop.activateApplication('Slack');

// URLs & Files
desktop.openUrl('https://example.com', 'Chrome');
desktop.openFile('/path/to/file.pdf');

// Locators (chainable)
desktop.locator('role:Button').click();
desktop.locator('name:Submit').type('text');
desktop.locator('role:Edit|name:Search').fill('query');

// Wait & Verify
desktop.locator('text:Success').waitFor({ timeout: 5000 });

// Screenshots
const screenshot = desktop.screenshot();

// Commands
await desktop.run('echo "Hello"');
await desktop.runCommand('dir', 'ls');

// Browser
const browser = await desktop.getCurrentBrowserWindow();
```

This approach gives you the full power of TypeScript with the terminator.js SDK!