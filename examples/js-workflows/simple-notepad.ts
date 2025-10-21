#!/usr/bin/env tsx
/**
 * Simple Notepad Workflow - TypeScript Example
 *
 * Demonstrates basic workflow using terminator.js SDK
 * Run with: tsx simple-notepad.ts
 */

import { Desktop } from '@mediar/terminator';

// Workflow metadata for mediar-app parsing
export const workflow = {
  id: 'simple-notepad',
  name: 'Simple Notepad Test',
  description: 'Opens Notepad and types a message using the terminator.js SDK',
  version: '1.0.0',
};

// Main workflow logic
async function main() {
  const desktop = new Desktop();

  console.log('🚀 Step 1: Opening Notepad');
  desktop.openApplication('notepad');
  await new Promise(r => setTimeout(r, 3000));

  console.log('⌨️  Step 2: Typing message');
  const textbox = desktop.locator('role:Edit');
  await textbox.type('Hello from TypeScript workflow!\n');
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
