import type { Desktop } from '@mediar/terminator';

/**
 * Workflow metadata that will be parsed by mediar-app
 */
export interface WorkflowMetadata {
  id: string;
  name: string;
  description?: string;
  version?: string;
  variables?: Record<string, VariableDefinition>;
  tags?: string[];
}

/**
 * Variable definition for workflow inputs
 */
export interface VariableDefinition {
  type: 'string' | 'number' | 'boolean';
  label: string;
  description?: string;
  default?: any;
}

/**
 * A step in the workflow - contains executable code
 */
export interface WorkflowStep {
  id: string;
  name: string;
  description?: string;
  execute: (desktop: Desktop, context: WorkflowContext) => Promise<void>;
}

/**
 * Context passed to each step containing variables and shared state
 */
export interface WorkflowContext {
  variables: Record<string, any>;
  state: Record<string, any>;
}

/**
 * Complete workflow definition
 */
export interface Workflow {
  metadata: WorkflowMetadata;
  steps: WorkflowStep[];
}

/**
 * Workflow builder for creating parseable workflow structures
 */
export class WorkflowBuilder {
  private metadata: WorkflowMetadata;
  private steps: WorkflowStep[] = [];

  constructor(id: string, name: string) {
    this.metadata = { id, name };
  }

  /**
   * Set workflow description
   */
  description(desc: string): this {
    this.metadata.description = desc;
    return this;
  }

  /**
   * Set workflow version
   */
  version(ver: string): this {
    this.metadata.version = ver;
    return this;
  }

  /**
   * Add a variable definition
   */
  variable(name: string, definition: VariableDefinition): this {
    if (!this.metadata.variables) {
      this.metadata.variables = {};
    }
    this.metadata.variables[name] = definition;
    return this;
  }

  /**
   * Add tags
   */
  tags(...tags: string[]): this {
    this.metadata.tags = tags;
    return this;
  }

  /**
   * Add a step to the workflow
   * The execute function contains your regular terminator.js SDK code
   */
  step(
    id: string,
    name: string,
    execute: (desktop: Desktop, context: WorkflowContext) => Promise<void>,
    description?: string
  ): this {
    this.steps.push({
      id,
      name,
      description,
      execute,
    });
    return this;
  }

  /**
   * Build the final workflow object
   */
  build(): Workflow {
    return {
      metadata: this.metadata,
      steps: this.steps,
    };
  }

  /**
   * Export in format parseable by mediar-app
   * This returns an object that can be exported as `export const workflow = ...`
   */
  export() {
    return {
      ...this.metadata,
      steps: this.steps.map(step => ({
        id: step.id,
        name: step.name,
        description: step.description,
        // The execute function will be stringified for parsing
        execute: step.execute.toString(),
      })),
    };
  }
}

/**
 * Helper to create a workflow
 */
export function defineWorkflow(id: string, name: string): WorkflowBuilder {
  return new WorkflowBuilder(id, name);
}

/**
 * Execute a workflow with the given context
 */
export async function executeWorkflow(
  workflow: Workflow,
  desktop: Desktop,
  variables: Record<string, any> = {}
): Promise<void> {
  const context: WorkflowContext = {
    variables,
    state: {},
  };

  for (const step of workflow.steps) {
    console.log(`🔄 Executing step: ${step.name}`);
    await step.execute(desktop, context);
    console.log(`✅ Completed step: ${step.name}`);
  }
}
