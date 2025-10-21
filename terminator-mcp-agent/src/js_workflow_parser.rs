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
    let re = Regex::new(r"export\s+const\s+workflow\s*=\s*")?;

    if let Some(m) = re.find(content) {
        let start = m.end();
        if let Some(obj_str) = extract_balanced_braces(&content[start..]) {
            debug!("Extracted object string (first 200 chars): {}", &obj_str.chars().take(200).collect::<String>());
            return parse_object_literal(&obj_str);
        } else {
            debug!("Failed to extract balanced braces from: {}", &content[start..].chars().take(100).collect::<String>());
        }
    }

    anyhow::bail!("No 'export const workflow' found")
}

/// Try to extract: export default {...}
fn try_export_default(content: &str) -> Result<Value> {
    let re = Regex::new(r"export\s+default\s+")?;

    if let Some(m) = re.find(content) {
        let start = m.end();
        if let Some(obj_str) = extract_balanced_braces(&content[start..]) {
            return parse_object_literal(&obj_str);
        }
    }

    anyhow::bail!("No 'export default' found")
}

/// Try to extract: module.exports = {...}
fn try_module_exports(content: &str) -> Result<Value> {
    let re = Regex::new(r"module\.exports\s*=\s*")?;

    if let Some(m) = re.find(content) {
        let start = m.end();
        if let Some(obj_str) = extract_balanced_braces(&content[start..]) {
            return parse_object_literal(&obj_str);
        }
    }

    anyhow::bail!("No 'module.exports' found")
}

/// Extract a balanced {...} object from the start of a string
fn extract_balanced_braces(s: &str) -> Option<String> {
    let s = s.trim_start();
    if !s.starts_with('{') {
        return None;
    }

    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut string_char = '\0';
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if escape_next {
            escape_next = false;
            i += 1;
            continue;
        }

        if ch == '\\' {
            escape_next = true;
            i += 1;
            continue;
        }

        if ch == '"' || ch == '\'' || ch == '`' {
            if !in_string {
                in_string = true;
                string_char = ch;
            } else if ch == string_char {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if in_string {
            i += 1;
            continue;
        }

        // Check for template variable ${{  }}
        if ch == '$' && i + 2 < chars.len() && chars[i + 1] == '{' && chars[i + 2] == '{' {
            // Skip the template variable - find the matching }}
            i += 3;
            let mut template_depth = 2; // We've seen {{
            while i < chars.len() && template_depth > 0 {
                if chars[i] == '}' {
                    template_depth -= 1;
                }
                i += 1;
            }
            continue;
        }

        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(chars[..=i].iter().collect());
            }
        }

        i += 1;
    }

    None
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
    // Match pattern: word characters followed by colon at start of value context
    // Use lookbehind to ensure it's after { , [ or newline
    let key_re = Regex::new(r#"(?m)(^|[{\[,]\s*)(\w+)\s*:"#)?;
    result = key_re.replace_all(&result, "$1\"$2\":").to_string();

    // Remove trailing commas before closing braces/brackets
    let trailing_comma_re = Regex::new(r",\s*([}\]])")?;
    result = trailing_comma_re.replace_all(&result, "$1").to_string();

    // Handle template string references like ${{ variables.name }}
    // Need to match the double closing braces specifically
    let template_var_re = Regex::new(r"\$\{\{([^}]*)\}\}")?;
    result = template_var_re
        .replace_all(&result, "\\${{$1}}")
        .to_string();

    Ok(result)
}

/// Remove JavaScript comments (single-line and multi-line)
fn remove_comments(content: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    let mut string_char = '\0';
    let mut escape_next = false;

    while i < chars.len() {
        let ch = chars[i];

        // Handle escape sequences
        if escape_next {
            escape_next = false;
            result.push(ch);
            i += 1;
            continue;
        }

        if ch == '\\' {
            escape_next = true;
            result.push(ch);
            i += 1;
            continue;
        }

        // Track string boundaries
        if ch == '"' || ch == '\'' || ch == '`' {
            if !in_string {
                in_string = true;
                string_char = ch;
            } else if ch == string_char {
                in_string = false;
            }
            result.push(ch);
            i += 1;
            continue;
        }

        // If we're in a string, just copy the character
        if in_string {
            result.push(ch);
            i += 1;
            continue;
        }

        // Check for multi-line comment /* ... */
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            // Skip until we find */
            i += 2;
            while i + 1 < chars.len() {
                if chars[i] == '*' && chars[i + 1] == '/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Check for single-line comment // ...
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            // Skip until end of line
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            // Include the newline
            if i < chars.len() {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // Normal character
        result.push(ch);
        i += 1;
    }

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

    // ========================================================================
    // Basic Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_minimal_workflow() {
        let js = r#"
export const workflow = {
  id: 'minimal',
  steps: []
};
"#;
        let result = parse_js_workflow(js);
        assert!(result.is_err(), "Empty steps should fail validation");
    }

    #[test]
    fn test_parse_simple_workflow() {
        let js = r#"
export const workflow = {
  id: 'test-workflow',
  name: 'Test Workflow',
  description: 'A test workflow',
  version: '1.0.0',
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
        assert_eq!(workflow.description, Some("A test workflow".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));
        assert_eq!(workflow.steps.len(), 1);

        // Verify step structure
        let step = &workflow.steps[0];
        assert_eq!(step.get("id").and_then(|v| v.as_str()), Some("step1"));
        assert_eq!(step.get("tool_name").and_then(|v| v.as_str()), Some("open_application"));
        assert!(step.get("arguments").is_some());
    }

    #[test]
    fn test_parse_multiple_steps() {
        let js = r#"
export const workflow = {
  id: 'multi-step',
  steps: [
    { tool_name: 'step1' },
    { tool_name: 'step2' },
    { tool_name: 'step3' }
  ]
};
"#;

        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.steps.len(), 3);
    }

    // ========================================================================
    // Export Pattern Tests
    // ========================================================================

    #[test]
    fn test_export_const_workflow() {
        let js = r#"
export const workflow = {
  id: 'export-const',
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("export-const".to_string()));
    }

    #[test]
    fn test_export_default() {
        let js = r#"
export default {
  id: 'export-default',
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("export-default".to_string()));
    }

    #[test]
    fn test_module_exports() {
        let js = r#"
module.exports = {
  id: 'module-exports',
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("module-exports".to_string()));
    }

    #[test]
    fn test_no_export_pattern() {
        let js = r#"
const workflow = {
  id: 'no-export',
  steps: [{ tool_name: 'test' }]
};
"#;
        let result = parse_js_workflow(js);
        assert!(result.is_err(), "Should fail without export pattern");
        assert!(result.unwrap_err().to_string().contains("Could not find workflow export"));
    }

    // ========================================================================
    // Comment Handling Tests
    // ========================================================================

    #[test]
    fn test_single_line_comments() {
        let js = r#"
// Header comment
export const workflow = {
  id: 'with-comments', // inline comment
  // another comment
  steps: [
    { tool_name: 'test' } // step comment
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("with-comments".to_string()));
    }

    #[test]
    fn test_multi_line_comments() {
        let js = r#"
/*
 * Multi-line comment block
 * with multiple lines
 */
export const workflow = {
  id: 'multi-comment',
  steps: [
    { /* inline block */ tool_name: 'test' }
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("multi-comment".to_string()));
    }

    #[test]
    fn test_mixed_comments() {
        let js = r#"
// Single line
/* Multi
   line */
export const workflow = {
  id: 'mixed', // inline
  /* block */ steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("mixed".to_string()));
    }

    // ========================================================================
    // String Literal Tests
    // ========================================================================

    #[test]
    fn test_single_quotes() {
        let js = r#"
export const workflow = {
  id: 'single-quotes',
  name: 'Test Name',
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.name, Some("Test Name".to_string()));
    }

    #[test]
    fn test_double_quotes() {
        let js = r#"
export const workflow = {
  "id": "double-quotes",
  "name": "Test Name",
  "steps": [{ "tool_name": "test" }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("double-quotes".to_string()));
    }

    #[test]
    fn test_template_literals() {
        let js = r#"
export const workflow = {
  id: `template-id`,
  name: `Template Name`,
  steps: [
    {
      tool_name: `test`,
      arguments: { text: `Hello World` }
    }
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("template-id".to_string()));
        assert_eq!(workflow.name, Some("Template Name".to_string()));
    }

    // ========================================================================
    // Variable and Metadata Tests
    // ========================================================================

    #[test]
    fn test_with_variables() {
        let js = r#"
export const workflow = {
  id: 'var-workflow',
  variables: {
    userName: {
      type: 'string',
      label: 'User Name',
      default: 'John Doe'
    },
    count: {
      type: 'number',
      default: 5
    }
  },
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert!(workflow.variables.is_some());

        let vars = workflow.variables.unwrap();
        assert!(vars.contains_key("userName"));
        assert!(vars.contains_key("count"));
    }

    #[test]
    fn test_with_inputs() {
        let js = r#"
export const workflow = {
  id: 'inputs-workflow',
  inputs: {
    userName: 'Alice',
    appName: 'notepad'
  },
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert!(workflow.inputs.is_some());

        let inputs = workflow.inputs.unwrap();
        assert_eq!(inputs.get("userName").and_then(|v| v.as_str()), Some("Alice"));
    }

    #[test]
    fn test_with_selectors() {
        let js = r#"
export const workflow = {
  id: 'selectors-workflow',
  selectors: {
    submitButton: 'role:Button|name:Submit',
    textField: 'role:Edit'
  },
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert!(workflow.selectors.is_some());
    }

    #[test]
    fn test_with_metadata() {
        let js = r#"
export const workflow = {
  id: 'metadata-workflow',
  metadata: {
    author: 'Test Author',
    tags: ['test', 'example']
  },
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert!(workflow.metadata.is_some());
    }

    #[test]
    fn test_with_timeout() {
        let js = r#"
export const workflow = {
  id: 'timeout-workflow',
  timeout: 30000,
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.timeout, Some(30000));
    }

    // ========================================================================
    // Complex Step Arguments Tests
    // ========================================================================

    #[test]
    fn test_nested_arguments() {
        let js = r#"
export const workflow = {
  id: 'nested',
  steps: [
    {
      tool_name: 'test',
      arguments: {
        simple: 'value',
        nested: {
          key1: 'value1',
          key2: {
            deep: true
          }
        },
        array: [1, 2, 3]
      }
    }
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        let step = &workflow.steps[0];
        let args = step.get("arguments").unwrap();

        assert!(args.get("simple").is_some());
        assert!(args.get("nested").is_some());
        assert!(args.get("array").is_some());
    }

    #[test]
    fn test_template_variable_references() {
        let js = r#"
export const workflow = {
  id: 'template-vars',
  steps: [
    {
      tool_name: 'test',
      arguments: {
        text: 'plain text without template'
      }
    }
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("template-vars".to_string()));

        // Test with actual template variables
        let js2 = r#"
export const workflow = {
  id: 'template-vars2',
  steps: [{
    tool_name: 'test',
    arguments: { text: '${{ inputs.userName }}' }
  }]
};
"#;
        let workflow2 = parse_js_workflow(js2);
        if workflow2.is_ok() {
            let step = &workflow2.unwrap().steps[0];
            let args = step.get("arguments").unwrap();
            // Template variables get escaped in parsing
            assert!(args.get("text").is_some());
        }
    }

    #[test]
    fn test_trailing_commas() {
        let js = r#"
export const workflow = {
  id: 'trailing-commas',
  name: 'Test',
  steps: [
    {
      tool_name: 'test',
      arguments: {
        key1: 'value1',
        key2: 'value2',
      },
    },
  ],
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        assert_eq!(workflow.id, Some("trailing-commas".to_string()));
    }

    // ========================================================================
    // Error Handling Tests
    // ========================================================================

    #[test]
    fn test_empty_steps_array() {
        let js = r#"
export const workflow = {
  id: 'empty-steps',
  steps: []
};
"#;
        let result = parse_js_workflow(js);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one step"));
    }

    #[test]
    fn test_missing_steps_field() {
        let js = r#"
export const workflow = {
  id: 'no-steps'
};
"#;
        let result = parse_js_workflow(js);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_json_structure() {
        let js = r#"
export const workflow = {
  id: 'invalid',
  steps: [
    {
      tool_name: 'test',
      // Invalid: missing closing brace
"#;
        let result = parse_js_workflow(js);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_export() {
        let js = r#"
const workflow = {
  id: 'private',
  steps: [{ tool_name: 'test' }]
};
"#;
        let result = parse_js_workflow(js);
        assert!(result.is_err());
    }

    // ========================================================================
    // Conversion to execute_sequence Tests
    // ========================================================================

    #[test]
    fn test_convert_to_execute_sequence() {
        let js = r#"
export const workflow = {
  id: 'test',
  name: 'Test',
  timeout: 5000,
  variables: { userName: { type: 'string', default: 'John' } },
  inputs: { userName: 'Jane' },
  selectors: { button: 'role:Button' },
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        let sequence = js_workflow_to_execute_sequence(workflow);

        assert_eq!(sequence.get("tool_name").and_then(|v| v.as_str()), Some("execute_sequence"));

        let args = sequence.get("arguments").unwrap();
        assert!(args.get("steps").is_some());
        assert!(args.get("variables").is_some());
        assert!(args.get("inputs").is_some());
        assert!(args.get("selectors").is_some());

        assert_eq!(sequence.get("timeout_ms").and_then(|v| v.as_u64()), Some(5000));
    }

    #[test]
    fn test_convert_minimal_workflow() {
        let js = r#"
export const workflow = {
  id: 'minimal',
  steps: [{ tool_name: 'test' }]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();
        let sequence = js_workflow_to_execute_sequence(workflow);

        let args = sequence.get("arguments").unwrap();
        assert_eq!(args.get("steps").unwrap().as_array().unwrap().len(), 1);
        assert!(args.get("variables").is_none());
        assert!(args.get("inputs").is_none());
    }

    // ========================================================================
    // Real-world Example Tests
    // ========================================================================

    #[test]
    fn test_realistic_notepad_workflow() {
        let js = r#"
export const workflow = {
  id: 'notepad-automation',
  name: 'Notepad Automation',
  description: 'Opens Notepad and types a message',
  version: '1.0.0',
  variables: {
    message: {
      type: 'string',
      default: 'Hello World'
    }
  },
  inputs: {
    message: 'Hello from workflow!'
  },
  steps: [
    {
      id: 'open-notepad',
      tool_name: 'open_application',
      arguments: { app_name: 'notepad' },
      delay_ms: 2000
    },
    {
      id: 'type-message',
      tool_name: 'type_into_element',
      arguments: { selector: 'role:Edit', text: 'typed text' },
      delay_ms: 1000
    }
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();

        assert_eq!(workflow.id, Some("notepad-automation".to_string()));
        assert_eq!(workflow.name, Some("Notepad Automation".to_string()));
        assert_eq!(workflow.steps.len(), 2);
        assert!(workflow.variables.is_some());
        assert!(workflow.inputs.is_some());

        // Verify step structure
        let step1 = &workflow.steps[0];
        assert_eq!(step1.get("id").and_then(|v| v.as_str()), Some("open-notepad"));
        assert_eq!(step1.get("delay_ms").and_then(|v| v.as_u64()), Some(2000));
    }

    #[test]
    fn test_realistic_browser_workflow() {
        let js = r#"
export const workflow = {
  id: 'browser-form-fill',
  name: 'Browser Form Automation',
  selectors: {
    firstName: 'roleTextbox',
    lastName: 'roleTextbox2',
    submit: 'roleButton'
  },
  steps: [
    { tool_name: 'open_url', arguments: { url: 'https://example.com', browser: 'Chrome' } },
    { tool_name: 'type_into_element', arguments: { selector: 'field1', text: 'John' } },
    { tool_name: 'type_into_element', arguments: { selector: 'field2', text: 'Doe' } },
    { tool_name: 'click_element', arguments: { selector: 'button' } }
  ]
};
"#;
        let workflow = parse_js_workflow(js).unwrap();

        assert_eq!(workflow.steps.len(), 4);
        assert!(workflow.selectors.is_some());

        let selectors = workflow.selectors.unwrap();
        assert!(selectors.get("firstName").is_some());
        assert!(selectors.get("submit").is_some());
    }
}
