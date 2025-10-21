// Workflow with Variables
// Demonstrates variable usage in JavaScript workflows

export const workflow = {
  id: 'js-workflow-with-variables',
  name: 'JavaScript Workflow with Variables',
  description: 'Demonstrates using variables in JavaScript workflows',

  // Define variables
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
    }
  },

  // Initial input values
  inputs: {
    userName: 'Jane Smith',
    appName: 'notepad'
  },

  // Workflow steps
  steps: [
    {
      id: 'step1',
      tool_name: 'open_application',
      arguments: {
        app_name: '${{ inputs.appName }}'
      },
      delay_ms: 3000
    },
    {
      id: 'step2',
      tool_name: 'type_into_element',
      arguments: {
        selector: 'role:Edit',
        text: 'Hello, ${{ inputs.userName }}!'
      },
      delay_ms: 1000
    }
  ]
};
