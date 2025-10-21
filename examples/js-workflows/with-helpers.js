// Workflow with Helper Functions
// Demonstrates code reusability with helper functions

// Helper function to create a step
function createStep(id, toolName, args, options = {}) {
  return {
    id,
    tool_name: toolName,
    arguments: args,
    ...options
  };
}

// Reusable step factories
const openApp = (id, appName, delay = 2000) =>
  createStep(id, 'open_application', { app_name: appName }, { delay_ms: delay });

const typeText = (id, selector, text, delay = 1000) =>
  createStep(id, 'type_into_element', { selector, text }, { delay_ms: delay });

const clickElement = (id, selector, delay = 1000) =>
  createStep(id, 'click_element', { selector }, { delay_ms: delay });

const pressKey = (id, key, delay = 500) =>
  createStep(id, 'press_key', { key }, { delay_ms: delay });

// Export the workflow using helper functions
export const workflow = {
  id: 'js-workflow-with-helpers',
  name: 'JavaScript Workflow with Helpers',
  description: 'Demonstrates reusable helper functions',

  steps: [
    openApp('open-notepad', 'notepad', 3000),
    typeText('type-greeting', 'role:Edit', 'This workflow uses helper functions!', 1000),
    pressKey('new-line', 'Enter', 500),
    typeText('type-more', 'role:Edit', 'Helper functions make workflows more readable.', 1000)
  ]
};

export const metadata = {
  author: 'Terminator Team',
  version: '1.0.0',
  tags: ['example', 'javascript', 'helpers', 'patterns']
};
