use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tracing::debug;

/// Represents a parsed JavaScript workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsWorkflow {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub timeout: Option<u64>,
    pub variables: Option<HashMap<String, Value>>,
    pub inputs: Option<HashMap<String, Value>>,
    pub selectors: Option<Value>,
    pub steps: Vec<Value>,
    pub metadata: Option<Value>,
}

/// Parses a JavaScript workflow file and extracts the workflow definition
pub fn parse_js_workflow(content: &str) -> Result<JsWorkflow> {
    debug!("Parsing JavaScript workflow");

    // Try to find export patterns:
    // - export const workflow = {...}
    // - export default {...}
    // - module.exports = {...}
    let workflow_obj = extract_workflow_export(content)?;

    // Parse the workflow object
    let workflow: JsWorkflow = serde_json::from_value(workflow_obj)
        .context("Failed to deserialize workflow object")?;

    // Validate that we have steps
    if workflow.steps.is_empty() {
        anyhow::bail!("Workflow must have at least one step");
    }

    debug!(
        "Successfully parsed JavaScript workflow with {} steps",
        workflow.steps.len()
    );

    Ok(workflow)
}

/// Extracts the workflow object from various export patterns
fn extract_workflow_export(content: &str) -> Result<Value> {
    // Remove comments first
    let content = remove_comments(content);

    // Try different export patterns
    if let Ok(obj) = try_export_const_workflow(&content) {
        return Ok(obj);
    }

    if let Ok(obj) = try_export_default(&content) {
        return Ok(obj);
    }

    if let Ok(obj) = try_module_exports(&content) {
        return Ok(obj);
    }

    anyhow::bail!("Could not find workflow export in JavaScript file. Expected 'export const workflow = {{...}}' or similar pattern")
}

/// Try to extract: export const workflow = {...}
fn try_export_const_workflow(content: &str) -> Result<Value> {
    let re = Regex::new(r"export\s+const\s+workflow\s*=\s*(\{[\s\S]*?\n\})\s*;?")?;

    if let Some(captures) = re.captures(content) {
        if let Some(obj_str) = captures.get(1) {
            return parse_object_literal(obj_str.as_str());
        }
    }

    anyhow::bail!("No 'export const workflow' found")
}

/// Try to extract: export default {...}
fn try_export_default(content: &str) -> Result<Value> {
    let re = Regex::new(r"export\s+default\s+(\{[\s\S]*?\n\})\s*;?")?;

    if let Some(captures) = re.captures(content) {
        if let Some(obj_str) = captures.get(1) {
            return parse_object_literal(obj_str.as_str());
        }
    }

    anyhow::bail!("No 'export default' found")
}

/// Try to extract: module.exports = {...}
fn try_module_exports(content: &str) -> Result<Value> {
    let re = Regex::new(r"module\.exports\s*=\s*(\{[\s\S]*?\n\})\s*;?")?;

    if let Some(captures) = re.captures(content) {
        if let Some(obj_str) = captures.get(1) {
            return parse_object_literal(obj_str.as_str());
        }
    }

    anyhow::bail!("No 'module.exports' found")
}

/// Parse a JavaScript object literal into JSON
/// This is a simplified parser that handles common patterns
fn parse_object_literal(obj_str: &str) -> Result<Value> {
    // Convert JavaScript object notation to JSON
    let json_str = js_to_json(obj_str)?;

    // Parse as JSON
    serde_json::from_str(&json_str)
        .with_context(|| format!("Failed to parse object as JSON: {}", json_str))
}

/// Convert JavaScript object notation to valid JSON
/// Handles:
/// - Unquoted keys: {foo: 'bar'} -> {"foo": "bar"}
/// - Single quotes: 'value' -> "value"
/// - Trailing commas: {a: 1,} -> {a: 1}
/// - Template literals: `value` -> "value"
fn js_to_json(js: &str) -> Result<String> {
    let mut result = js.to_string();

    // Replace template literals with double quotes (simple case only)
    let template_re = Regex::new(r"`([^`]*)`")?;
    result = template_re
        .replace_all(&result, |caps: &regex::Captures| {
            format!("\"{}\"", caps.get(1).map_or("", |m| m.as_str()))
        })
        .to_string();

    // Replace single quotes with double quotes
    // This is tricky because we need to avoid replacing escaped quotes
    result = result.replace("\\'", "<<<ESCAPED_QUOTE>>>"); // Temporarily replace escaped quotes
    let single_quote_re = Regex::new(r"'([^']*)'")?;
    result = single_quote_re
        .replace_all(&result, "\"$1\"")
        .to_string();
    result = result.replace("<<<ESCAPED_QUOTE>>>", "\\'");

    // Quote unquoted object keys
    // Match pattern: word characters followed by colon (not inside quotes)
    let key_re = Regex::new(r"(\w+)\s*:")?;
    result = key_re.replace_all(&result, "\"$1\":").to_string();

    // Remove trailing commas before closing braces/brackets
    let trailing_comma_re = Regex::new(r",\s*([}\]])")?;
    result = trailing_comma_re.replace_all(&result, "$1").to_string();

    // Handle template string references like ${{ variables.name }}
    // Keep them as strings for now
    let template_var_re = Regex::new(r"\$\{\{\s*([^}]+)\s*\}\}")?;
    result = template_var_re
        .replace_all(&result, "${{$1}}")
        .to_string();

    Ok(result)
}

/// Remove JavaScript comments (single-line and multi-line)
fn remove_comments(content: &str) -> String {
    let mut result = content.to_string();

    // Remove multi-line comments /* ... */
    let multiline_re = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    result = multiline_re.replace_all(&result, "").to_string();

    // Remove single-line comments // ... (with multiline flag)
    let singleline_re = Regex::new(r"(?m)//.*$").unwrap();
    result = singleline_re.replace_all(&result, "").to_string();

    result
}

/// Load and parse a JavaScript workflow file
pub async fn load_js_workflow(path: &Path) -> Result<JsWorkflow> {
    let content =
        tokio::fs::read_to_string(path).await.with_context(|| {
            format!("Failed to read JavaScript workflow file: {:?}", path)
        })?;

    parse_js_workflow(&content)
}

/// Convert a JavaScript workflow to the standard workflow format (execute_sequence)
pub fn js_workflow_to_execute_sequence(js_workflow: JsWorkflow) -> Value {
    let mut sequence = serde_json::json!({
        "tool_name": "execute_sequence",
        "arguments": {
            "steps": js_workflow.steps
        }
    });

    // Add optional fields if present
    let args = sequence["arguments"].as_object_mut().unwrap();

    if let Some(variables) = js_workflow.variables {
        args.insert("variables".to_string(), serde_json::to_value(variables).unwrap());
    }

    if let Some(inputs) = js_workflow.inputs {
        args.insert("inputs".to_string(), serde_json::to_value(inputs).unwrap());
    }

    if let Some(selectors) = js_workflow.selectors {
        args.insert("selectors".to_string(), selectors);
    }

    if let Some(timeout) = js_workflow.timeout {
        sequence
            .as_object_mut()
            .unwrap()
            .insert("timeout_ms".to_string(), serde_json::to_value(timeout).unwrap());
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_workflow() {
        let js = r#"
export const workflow = {
  id: 'test-workflow',
  name: 'Test Workflow',
  steps: [
    {
      id: 'step1',
      tool_name: 'open_application',
      arguments: {
        app_name: 'notepad'
      }
    }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("test-workflow".to_string()));
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.steps.len(), 1);
    }

    #[test]
    fn test_parse_with_variables() {
        let js = r#"
export const workflow = {
  id: 'var-workflow',
  variables: {
    userName: {
      type: 'string',
      default: 'John'
    }
  },
  steps: [
    {
      tool_name: 'type_into_element',
      arguments: {
        text: '${{ inputs.userName }}'
      }
    }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert!(workflow.variables.is_some());
        assert_eq!(workflow.steps.len(), 1);
    }

    #[test]
    fn test_export_default() {
        let js = r#"
export default {
  id: 'default-workflow',
  steps: [
    {
      tool_name: 'click_element',
      arguments: { selector: 'button' }
    }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("default-workflow".to_string()));
    }

    #[test]
    fn test_module_exports() {
        let js = r#"
module.exports = {
  id: 'commonjs-workflow',
  steps: [
    {
      tool_name: 'open_application',
      arguments: { app_name: 'chrome' }
    }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("commonjs-workflow".to_string()));
    }

    #[test]
    fn test_with_comments() {
        let js = r#"
// This is a comment
/* Multi-line
   comment */
export const workflow = {
  id: 'commented-workflow', // inline comment
  steps: [
    {
      tool_name: 'test' // another comment
    }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("commented-workflow".to_string()));
    }

    #[test]
    fn test_template_literals() {
        let js = r#"
export const workflow = {
  id: `template-workflow`,
  steps: [
    {
      tool_name: 'type_into_element',
      arguments: {
        text: `Hello World`
      }
    }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("template-workflow".to_string()));
    }
}
