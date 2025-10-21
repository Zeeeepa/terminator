#!/usr/bin/env node
/**
 * Simple Notepad Workflow - Using terminator.js SDK
 *
 * This demonstrates a basic workflow using the SDK directly
 * with a parseable structure for mediar-app
 */

const { Desktop } = require('@mediar/terminator');

// Workflow metadata for parsing
export const workflow = {
  id: 'simple-notepad',
  name: 'Simple Notepad Test',
  description: 'Opens Notepad and types a message using the terminator.js SDK',
  version: '1.0.0',
};

// Main workflow function
async function main() {
  const desktop = new Desktop();

  console.log('🚀 Step 1: Opening Notepad');
  desktop.openApplication('notepad');
  await new Promise(r => setTimeout(r, 3000));

  console.log('⌨️  Step 2: Typing message');
  const textbox = desktop.locator('role:Edit');
  await textbox.type('Hello from JavaScript workflow!\n');
  await textbox.type('This uses the terminator.js SDK directly.\n');
  await textbox.type('\nCreated at: ' + new Date().toLocaleString());

  console.log('✅ Workflow completed successfully');
}

// Execute if run directly
if (require.main === module) {
  main().catch(error => {
    console.error('❌ Workflow failed:', error);
    process.exit(1);
  });
}

// Export for programmatic use
module.exports = { workflow, execute: main };
