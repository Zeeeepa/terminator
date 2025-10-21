// Inngest-Style Workflow
// Demonstrates Inngest-compatible workflow structure

export const workflow = {
  id: 'inngest-style-workflow',
  name: 'Inngest-Style Workflow Example',
  description: 'Workflow following Inngest patterns',

  // Trigger (for reference - not used in execution)
  trigger: {
    event: 'app.automation.requested',
    description: 'Triggered when automation is requested'
  },

  // Configuration
  config: {
    id: 'notepad-automation',
    timeout: 60000
  },

  // Steps array (Inngest-style but adapted for Terminator)
  steps: [
    {
      id: 'setup',
      name: 'Setup Application',
      run: {
        tool_name: 'open_application',
        arguments: {
          app_name: 'notepad'
        }
      },
      delay_ms: 3000
    },
    {
      id: 'execute-script',
      name: 'Execute Automation Script',
      run: {
        tool_name: 'run_command',
        arguments: {
          engine: 'javascript',
          run: `
            console.log('Starting automation...');

            // This is a simple example
            const message = 'Hello from Inngest-style workflow!';
            console.log('Message:', message);

            return {
              success: true,
              set_env: {
                automation_message: message,
                timestamp: new Date().toISOString()
              }
            };
          `
        }
      },
      delay_ms: 2000
    },
    {
      id: 'type-result',
      name: 'Type Automation Result',
      run: {
        tool_name: 'type_into_element',
        arguments: {
          selector: 'role:Edit',
          text: 'Inngest-style workflow executed at: ${{ env.timestamp }}'
        }
      },
      delay_ms: 1000
    }
  ]
};

export const metadata = {
  framework: 'inngest',
  author: 'Terminator Team',
  version: '1.0.0',
  tags: ['inngest', 'example', 'events']
};
