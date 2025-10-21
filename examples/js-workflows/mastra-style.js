// Mastra-Style Workflow
// Demonstrates Mastra AI compatible workflow structure

export const workflow = {
  id: 'mastra-style-workflow',
  name: 'Mastra-Style Workflow Example',
  description: 'Workflow following Mastra AI patterns',

  // Input schema (Mastra-style)
  inputSchema: {
    type: 'object',
    properties: {
      applicationName: {
        type: 'string',
        description: 'Application to automate'
      },
      message: {
        type: 'string',
        description: 'Message to type'
      }
    },
    required: ['applicationName', 'message']
  },

  // Default inputs
  inputs: {
    applicationName: 'notepad',
    message: 'Hello from Mastra-style workflow!'
  },

  // Steps with schema definitions
  steps: [
    {
      id: 'open-app-step',
      name: 'Open Application',
      description: 'Opens the specified application',
      inputSchema: {
        type: 'object',
        properties: {
          app_name: { type: 'string' }
        }
      },
      outputSchema: {
        type: 'object',
        properties: {
          success: { type: 'boolean' }
        }
      },
      // Terminator execution
      tool_name: 'open_application',
      arguments: {
        app_name: '${{ inputs.applicationName }}'
      },
      delay_ms: 3000
    },
    {
      id: 'type-message-step',
      name: 'Type Message',
      description: 'Types the message into the application',
      inputSchema: {
        type: 'object',
        properties: {
          text: { type: 'string' }
        }
      },
      outputSchema: {
        type: 'object',
        properties: {
          success: { type: 'boolean' },
          text_length: { type: 'number' }
        }
      },
      // Terminator execution
      tool_name: 'type_into_element',
      arguments: {
        selector: 'role:Edit',
        text: '${{ inputs.message }}'
      },
      delay_ms: 1000
    }
  ]
};

export const metadata = {
  framework: 'mastra',
  author: 'Terminator Team',
  version: '1.0.0',
  tags: ['mastra', 'example', 'schema']
};
