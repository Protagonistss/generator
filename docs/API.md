# Project Generator API Documentation

## Overview
This document describes the API for the Project Generator napi-rs module.

## Functions

### `generate_project(options: GenerateOptions) -> Promise<GenerateResult>`
Generates a new project based on the provided options.

#### Parameters
- `options`: GenerateOptions object containing:
  - `name`: Project name (string)
  - `project_type`: Project type - "java", "vue", or "react" (string)
  - `template`: Template name (optional string)
  - `output_path`: Output directory path (optional string)
  - `variables`: Additional template variables (optional object)

#### Returns
Promise resolving to GenerateResult:
- `success`: Whether generation was successful (boolean)
- `files`: List of generated files (string[])
- `message`: Optional message (string)

### `list_templates(project_type: string) -> string[]`
Lists available templates for a given project type.

#### Parameters
- `project_type`: Project type - "java", "vue", or "react"

#### Returns
Array of template names

### `get_template_info(project_type: string, template: string) -> string`
Gets information about a specific template.

#### Parameters
- `project_type`: Project type
- `template`: Template name

#### Returns
Template information as string

## Usage Examples

```javascript
const generator = require('./index.node');

// Generate a Vue project
const result = await generator.generate_project({
  name: 'my-vue-app',
  project_type: 'vue',
  template: 'basic',
  output_path: './projects'
});

// List Vue templates
const templates = generator.list_templates('vue');
console.log(templates); // ['basic']

// Get template info
const info = generator.get_template_info('vue', 'basic');
console.log(info);
```