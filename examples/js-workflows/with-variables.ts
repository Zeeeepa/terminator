#!/usr/bin/env tsx
/**
 * Workflow with Variables - TypeScript Example
 *
 * Demonstrates using typed variables in workflows
 * Run with: tsx with-variables.ts --userName "Jane" --message "Hello!"
 */

import { Desktop } from '@mediar/terminator';

// Workflow configuration with variable definitions
export const workflow = {
  id: 'workflow-with-variables',
  name: 'Workflow with Variables',
  description: 'Demonstrates variable usage in TypeScript workflows',

  // Define variables (mediar-app will show these as UI inputs)
  variables: {
    userName: {
      type: 'string',
      label: 'User Name',
      description: 'The name to type into the application',
      default: 'John Doe'
    },
    appName: {
      type: 'string',
      label: 'Application',
      description: 'Application to open',
      default: 'notepad'
    },
    message: {
      type: 'string',
      label: 'Custom Message',
      description: 'Additional message to type',
      default: 'Welcome!'
    }
  }
};

// Typed parameters interface
interface WorkflowParams {
  userName?: string;
  appName?: string;
  message?: string;
}

// Main workflow function with type-safe parameters
async function main(params: WorkflowParams = {}) {
  const {
    userName = 'John Doe',
    appName = 'notepad',
    message = 'Welcome!'
  } = params;

  const desktop = new Desktop();

  console.log(`🚀 Step 1: Opening ${appName}`);
  desktop.openApplication(appName);
  await new Promise(r => setTimeout(r, 3000));

  console.log(`⌨️  Step 2: Typing personalized message for ${userName}`);
  const textbox = desktop.locator('role:Edit');
  await textbox.type(`Hello, ${userName}!\n\n`);
  await textbox.type(`${message}\n\n`);
  await textbox.type(`This workflow was executed at: ${new Date().toLocaleString()}`);

  console.log('✅ Workflow completed');
}

// Parse command line arguments
function parseArgs(): WorkflowParams {
  const args = process.argv.slice(2);
  const params: Record<string, string> = {};

  for (let i = 0; i < args.length; i += 2) {
    const key = args[i].replace(/^--/, '');
    const value = args[i + 1];
    if (key && value) {
      params[key] = value;
    }
  }

  return params as WorkflowParams;
}

// Execute if run directly
if (require.main === module) {
  const params = parseArgs();
  console.log('Parameters:', params);

  main(params).catch(error => {
    console.error('❌ Workflow failed:', error);
    process.exit(1);
  });
}
