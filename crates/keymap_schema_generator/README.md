# Keymap Schema Generator

A command-line tool that generates JSON schemas for Zed's `keymap.json` configuration files. This tool extracts the complete keymap schema that Zed uses internally for validation and autocompletion, making it available for third-party tooling.

## Features

- **Complete Schema Generation**: Generates the exact same JSON schema that Zed uses internally
- **Action Metadata**: Optional export of all available actions with their metadata
- **Real-time Accuracy**: Always up-to-date with the current Zed build since it uses the same generation code
- **Multiple Output Formats**: Pretty-printed or minified JSON output

## Usage

### Basic Usage

Generate a keymap schema file:

```bash
cargo run -p keymap_schema_generator
```

This creates `keymap-schema.json` in the current directory.

### Advanced Usage

```bash
# Pretty-printed output
cargo run -p keymap_schema_generator -- --pretty

# Custom output file
cargo run -p keymap_schema_generator -- -o my-keymap-schema.json

# Include action metadata
cargo run -p keymap_schema_generator -- --metadata --pretty

# All options combined
cargo run -p keymap_schema_generator -- --pretty --metadata -o zed-keymap-schema.json
```

### Command Line Options

- `-o, --output <FILE>`: Output file path for the JSON schema (default: `keymap-schema.json`)
- `-p, --pretty`: Pretty-print the JSON schema for better readability
- `-m, --metadata`: Also generate a metadata file with all available actions and their details
- `-h, --help`: Show help information

## Generated Files

### Schema File

The main schema file contains:

- **Complete keymap structure validation**: Validates the overall keymap file format
- **All available actions**: Every action that can be bound to keys, with parameter schemas
- **Deprecation warnings**: Information about deprecated actions and their replacements
- **Documentation strings**: Help text from action definitions
- **Validation rules**: For both string actions (`"action_name"`) and array format (`["action_name", {...}]`)

### Metadata File (optional)

When using `--metadata`, an additional file is generated containing:

- **Total action count**: Number of registered actions
- **Action details**: For each action:
  - Full action name
  - Deprecated aliases
  - Human-readable name (as shown in command palette)

## Schema Contents

The generated schema includes validation for:

- **Keymap sections**: Multiple binding contexts (Editor, Workspace, etc.)
- **Context expressions**: Boolean logic for when bindings are active
- **Key bindings**: Keystroke to action mappings
- **Action parameters**: Type-safe validation of action arguments
- **Key equivalents**: Platform-specific key handling

## Use Cases

This tool enables building third-party applications such as:

- **Keymap editors**: GUI tools for editing Zed keymaps
- **Documentation generators**: Tools that generate keymap documentation
- **Migration tools**: Scripts to convert keymaps between editors
- **Validation tools**: Linters for keymap configuration files
- **IDE integrations**: Language server support for keymap editing

## Example Schema Usage

The generated schema can be used with any JSON schema validator. For example, with a JSON schema library:

```javascript
import Ajv from 'ajv';
import keymapSchema from './keymap-schema.json';

const ajv = new Ajv();
const validate = ajv.compile(keymapSchema);

const isValid = validate(keymapConfig);
if (!isValid) {
  console.log(validate.errors);
}
```

## Implementation Details

This tool works by:

1. **Linking the full Zed application**: Ensures all actions are registered
2. **Creating a minimal GPUI context**: Provides access to the action registry
3. **Calling the same schema generation code**: Uses `KeymapFile::generate_json_schema_for_registered_actions()`
4. **Extracting action metadata**: Via `gpui::generate_list_of_all_registered_actions()`

The generated schema is identical to what Zed uses internally for its JSON language server integration.

## Building

This crate is part of the Zed workspace and builds with the standard Cargo commands:

```bash
cargo build -p keymap_schema_generator
cargo run -p keymap_schema_generator
```

## Dependencies

- Depends on `zed`, `gpui`, `settings`, and other Zed crates
- Uses the same action registration system as the main Zed application
- Requires the complete Zed build environment