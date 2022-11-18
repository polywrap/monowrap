use serde_json::json;
use polywrap_wasm_rs::{JSON};
use jsonschema::{JSONSchema};
use crate::wrap::monowrap_manifest::*;

pub fn get_schema() -> JSONSchema {
  let json_schema = json!({
  "id": "MonowrapManifest",
  "type": "object",
  "additionalProperties": false,
  "required": ["name", "commands", "dependencies", "sub_deps_execute_after"],
  "properties": {
    "name": {
      "description": "Name of the monorepo",
      "type": "string"
    },
    "path": {
      "description": "Path of the manifest",
      "type": "string"
    },
    "commands": {
      "description": "Commands that can be executed from the monorepo",
      "type": "array",
      "items": {
        "$ref": "#/definitions/command"
      }
    },
    "dependencies": {
      "description": "Dependency list for the packages of the monorepo",
      "type": "array",
      "items": {
        "$ref": "#/definitions/dependency"
      }
    },
    "sub_deps_execute_after": {
      "description": "Execute sub dependency after the given command completes",
      "type": "string"
    }
  },
  "definitions": {
    "command": {
      "type": "object",
      "additionalProperties": false,
      "required": ["alias", "uri", "args"],
      "properties": {
        "alias": {
          "description": "Alias for the given command",
          "type": "string",
          "pattern": "^[a-zA-Z0-9\\-\\_]+$"
        },
        "uri": {
          "description": "Uri of the wrapper that this command will execute",
          "type": "string",
          "pattern": "(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*\\(\\),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+"
        },
        "args": {
          "description": "Msgpack encoded hexadecimal arguments that needs to be passed for the invocation",
          "type": "string"
        },
        "requires": {
          "description": "Dependencies of the current command",
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "dependency": {
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "path"],
      "properties": {
        "name": {
          "description": "Name of the package",
          "type": "string",
          "pattern": "^[a-zA-Z0-9\\-\\_]+$"
        },
        "path": {
          "description": "Path of the package",
          "type": "string"
        },
        "requires": {
          "description": "Dependencies of the current package",
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    }
  }
}
);
  JSONSchema::compile(&json_schema).unwrap()
}

pub fn validate_manifest(json_manifest: &String) -> () {
  let manifest = JSON::from_str::<JSON::Value>(&json_manifest).unwrap();
  let schema = get_schema();
  let result = schema.validate(&manifest);
  
  if let Err(errors) = result {
    let mut error_msg: String = String::new();
    for error in errors {
      let str = format!("Validation error: {}\nInstance path: {}", error, error.instance_path);
      error_msg += &str;
    }
    panic!("{}", error_msg);
  }
}

pub fn deserialize_manifest(json_manifest: String) -> MonowrapManifest {
  validate_manifest(&json_manifest);
  JSON::from_str::<MonowrapManifest>(&json_manifest).unwrap()
}
