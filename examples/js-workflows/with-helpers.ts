#!/usr/bin/env tsx
/**
 * Workflow with Helper Functions - TypeScript Example
 *
 * Demonstrates reusable helper patterns
 * Run with: tsx with-helpers.ts
 */

import { Desktop } from '@mediar/terminator';

// Workflow metadata
export const workflow = {
  id: 'workflow-with-helpers',
  name: 'Workflow with Helper Functions',
  description: 'Demonstrates reusable helper patterns for better code organization',
  version: '1.0.0',
};

// ============================================================================
// Helper Class - Reusable across workflows
// ============================================================================

class WorkflowHelpers {
  constructor(private desktop: Desktop) {}

  /** Open an application and wait for it to be ready */
  async openApp(appName: string, waitMs = 2000): Promise<void> {
    console.log(`🚀 Opening ${appName}...`);
    this.desktop.openApplication(appName);
    await this.wait(waitMs);
  }

  /** Type text into a specific element */
  async typeInto(selector: string, text: string, waitAfter = 500): Promise<void> {
    console.log(`⌨️  Typing into ${selector}...`);
    const element = this.desktop.locator(selector);
    await element.type(text);
    await this.wait(waitAfter);
  }

  /** Click an element */
  async click(selector: string, waitAfter = 500): Promise<void> {
    console.log(`👆 Clicking ${selector}...`);
    const element = this.desktop.locator(selector);
    await element.click();
    await this.wait(waitAfter);
  }

  /** Wait for a specified time */
  async wait(ms: number): Promise<void> {
    if (ms > 0) {
      console.log(`⏸️  Waiting ${ms}ms...`);
      await new Promise(r => setTimeout(r, ms));
    }
  }

  /** Take a screenshot */
  async screenshot(description = 'screenshot') {
    console.log(`📸 Taking ${description}...`);
    const screenshot = this.desktop.screenshot();
    console.log(`   Captured: ${screenshot.width}x${screenshot.height}`);
    return screenshot;
  }

  /** Fill a form with multiple fields */
  async fillForm(fields: Array<{ selector: string; value: string; wait?: number }>) {
    console.log(`📝 Filling form with ${fields.length} fields...`);
    for (const field of fields) {
      await this.typeInto(field.selector, field.value, field.wait || 300);
    }
  }
}

// ============================================================================
// Main Workflow
// ============================================================================

async function main() {
  const desktop = new Desktop();
  const helpers = new WorkflowHelpers(desktop);

  console.log('='.repeat(60));
  console.log('Starting Notepad Workflow with Helpers');
  console.log('='.repeat(60));

  // Step 1: Open application
  console.log('\n📌 Step 1: Open Application');
  await helpers.openApp('notepad', 3000);

  // Step 2: Type structured content
  console.log('\n📌 Step 2: Type Content');
  await helpers.typeInto('role:Edit',
    'Hello from Workflow with Helpers!\n\n' +
    'This workflow demonstrates:\n' +
    '- Reusable helper functions\n' +
    '- Better code organization\n' +
    '- Type-safe TypeScript\n' +
    '- Easier maintenance\n' +
    '- Improved readability\n\n' +
    `Executed at: ${new Date().toLocaleString()}\n`,
    1000
  );

  // Step 3: Take screenshot
  console.log('\n📌 Step 3: Capture Result');
  await helpers.screenshot('final result');

  console.log('\n' + '='.repeat(60));
  console.log('✅ Workflow completed successfully');
  console.log('='.repeat(60));
}

// Execute if run directly
if (require.main === module) {
  main().catch(error => {
    console.error('\n❌ Workflow failed:', error);
    process.exit(1);
  });
}

// Export helpers for use in other workflows
export { WorkflowHelpers };
