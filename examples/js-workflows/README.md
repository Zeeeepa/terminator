# TypeScript/JavaScript Workflow Examples

These examples demonstrate how to write automation workflows using the **terminator.js SDK** in TypeScript.

## Prerequisites

```bash
npm install @mediar/terminator
npm install -D typescript tsx @types/node
```

## Examples

### 1. Simple Notepad (`simple-notepad.ts`)

Basic workflow that opens Notepad and types text. Great starting point.

```bash
tsx simple-notepad.ts
```

**What it demonstrates:**
- Basic SDK usage (`Desktop`, `locator()`, `type()`)
- Workflow metadata export for mediar-app
- Console logging for step visibility

### 2. With Variables (`with-variables.ts`)

Shows how to use typed variables and command-line arguments.

```bash
tsx with-variables.ts --userName "Jane Doe" --message "Hello World"
```

**What it demonstrates:**
- TypeScript interfaces for type safety
- Variable definitions for mediar-app UI
- Command-line argument parsing
- Parameter defaults

### 3. With Helpers (`with-helpers.ts`)

Demonstrates reusable helper class pattern.

```bash
tsx with-helpers.ts
```

**What it demonstrates:**
- Helper class for common operations
- Code reusability across workflows
- Better code organization
- TypeScript class methods

## Structure

All workflows follow this pattern:

```typescript
import { Desktop } from '@mediar/terminator';

// 1. Export metadata for mediar-app
export const workflow = {
  id: 'unique-id',
  name: 'Display Name',
  description: 'What it does',
  variables: { /* optional variables */ }
};

// 2. Define main logic
async function main(params = {}) {
  const desktop = new Desktop();
  // Your automation code here
}

// 3. Execute if run directly
if (require.main === module) {
  main().catch(console.error);
}
```

## Running Workflows

### From Command Line
```bash
# TypeScript (recommended)
tsx workflow.ts

# Compile and run JavaScript
tsc workflow.ts && node workflow.js
```

### From Terminator CLI
```bash
terminator-cli execute-workflow --url file://./workflow.ts
```

### From mediar-app
1. Open mediar-app
2. Navigate to Workflows
3. Select your `.ts` file
4. Click Run

## Best Practices

1. **Use TypeScript** - Get autocomplete and catch errors early
2. **Export workflow metadata** - Required for mediar-app to display properly
3. **Console logs with emojis** - Makes execution progress clear
4. **Error handling** - Always wrap in try-catch
5. **Helper functions** - Extract reusable patterns
6. **Type safety** - Define interfaces for parameters

## SDK Quick Reference

```typescript
const desktop = new Desktop();

// Applications
desktop.openApplication('notepad');
desktop.application('Chrome');
desktop.activateApplication('Slack');

// Locators (use accessibility selectors)
desktop.locator('role:Button').click();
desktop.locator('name:Submit').type('text');
desktop.locator('role:Edit|name:Search').fill('query');

// Wait & Verify
await desktop.locator('text:Success').waitFor({ timeout: 5000 });

// Screenshots
const screenshot = desktop.screenshot();

// Commands
await desktop.run('echo "Hello"');
```

## Framework Compatibility

Want to use Mastra AI or Inngest patterns? Check the [main documentation](../../docs/JAVASCRIPT_WORKFLOWS.md) for examples.

## Next Steps

- Read the [full documentation](../../docs/JAVASCRIPT_WORKFLOWS.md)
- Check [SDK type definitions](../../bindings/nodejs/index.d.ts)
- Explore [more examples](../../examples/)

## Support

Questions? Join our [Discord](https://discord.gg/dU9EBuw7Uq) or check the [main README](../../README.md).
