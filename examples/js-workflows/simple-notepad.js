// Simple Notepad Workflow
// Demonstrates basic JavaScript workflow structure

export const workflow = {
  id: 'simple-notepad-js',
  name: 'Simple Notepad Test (JavaScript)',
  description: 'Opens Notepad and types text - defined in JavaScript',

  steps: [
    {
      id: 'open-notepad',
      tool_name: 'open_application',
      arguments: {
        app_name: 'notepad'
      },
      delay_ms: 3000
    },
    {
      id: 'type-text',
      tool_name: 'type_into_element',
      arguments: {
        selector: 'role:Edit',
        text: 'Hello from JavaScript workflow!\nThis workflow was defined in a .js file.'
      },
      delay_ms: 1000
    }
  ]
};

// Optional: Export metadata separately
export const metadata = {
  author: 'Terminator Team',
  version: '1.0.0',
  tags: ['example', 'javascript', 'notepad'],
  created_at: '2025-01-21'
};
