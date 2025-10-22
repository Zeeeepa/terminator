#!/usr/bin/env tsx
/**
 * Workflow using @mediar/terminator-workflow package
 *
 * This demonstrates the clean, parseable structure for mediar-app
 * Run with: tsx with-workflow-builder.ts --userName "Alice"
 */

import { Desktop } from '@mediar/terminator';
import { defineWorkflow, executeWorkflow } from '@mediar/terminator-workflow';

// Define workflow with clean, parseable structure
const workflow = defineWorkflow('notepad-structured', 'Structured Notepad Workflow')
  .description('Demonstrates workflow builder with parseable steps')
  .version('1.0.0')
  .tags('demo', 'notepad', 'automation')
  .variable('userName', {
    type: 'string',
    label: 'User Name',
    description: 'Name to include in the greeting',
    default: 'World'
  })
  .variable('includeTimestamp', {
    type: 'boolean',
    label: 'Include Timestamp',
    description: 'Whether to add a timestamp',
    default: true
  })

  // Step 1: Open Notepad
  .step('open-notepad', 'Open Notepad Application', async (desktop, context) => {
    console.log('📝 Opening Notepad...');
    desktop.openApplication('notepad');
    await new Promise(r => setTimeout(r, 2000));
  }, 'Launches Notepad and waits for it to be ready')

  // Step 2: Type greeting
  .step('type-greeting', 'Type Personalized Greeting', async (desktop, context) => {
    const { userName } = context.variables;
    console.log(`👋 Typing greeting for ${userName}...`);

    const textbox = desktop.locator('role:Edit');
    await textbox.type(`Hello, ${userName}!\\n\\n`);

    // Store info in context for later steps
    context.state.greetingComplete = true;
  }, 'Types a personalized greeting message')

  // Step 3: Add workflow info
  .step('add-info', 'Add Workflow Information', async (desktop, context) => {
    console.log('ℹ️  Adding workflow info...');

    const textbox = desktop.locator('role:Edit');
    await textbox.type('This workflow demonstrates:\\n');
    await textbox.type('- Structured, parseable steps\\n');
    await textbox.type('- Type-safe variables\\n');
    await textbox.type('- Context sharing between steps\\n');
    await textbox.type('- Clean SDK usage\\n\\n');
  }, 'Adds information about the workflow features')

  // Step 4: Optionally add timestamp
  .step('add-timestamp', 'Add Timestamp (Optional)', async (desktop, context) => {
    const { includeTimestamp } = context.variables;

    if (includeTimestamp) {
      console.log('🕐 Adding timestamp...');
      const textbox = desktop.locator('role:Edit');
      const timestamp = new Date().toLocaleString();
      await textbox.type(`Generated at: ${timestamp}\\n`);
    } else {
      console.log('⏭️  Skipping timestamp (disabled)');
    }
  }, 'Conditionally adds a timestamp if enabled')

  .build();

// Export for mediar-app to parse
export const workflowDefinition = workflow.export();

// Parse command line arguments
function parseArgs(): Record<string, any> {
  const args = process.argv.slice(2);
  const params: Record<string, any> = {};

  for (let i = 0; i < args.length; i += 2) {
    const key = args[i].replace(/^--/, '');
    const value = args[i + 1];
    if (key && value !== undefined) {
      // Try to parse as boolean
      if (value === 'true') params[key] = true;
      else if (value === 'false') params[key] = false;
      else params[key] = value;
    }
  }

  return params;
}

// Execute if run directly
async function main() {
  const params = parseArgs();

  console.log('='.repeat(60));
  console.log('Starting Structured Workflow');
  console.log('Variables:', params);
  console.log('='.repeat(60));
  console.log('');

  const desktop = new Desktop();
  await executeWorkflow(workflow, desktop, params);

  console.log('');
  console.log('='.repeat(60));
  console.log('✅ Workflow completed successfully');
  console.log('='.repeat(60));
}

if (require.main === module) {
  main().catch(error => {
    console.error('\\n❌ Workflow failed:', error);
    process.exit(1);
  });
}
