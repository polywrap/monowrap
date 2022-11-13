import fs from "fs";
import path from "path";

export function renderRustManifest(jsonSchema: string) {
  return `use serde_json::json;
use polywrap_wasm_rs::{Map, JSON};
use jsonschema::{Draft, JSONSchema};
use crate::wrap::monowrap_manifest::*;

pub fn get_schema() -> JSONSchema {
  let json_schema = json!(${jsonSchema});
  JSONSchema::compile(&json_schema).unwrap()
}

pub fn validate_manifest(json_manifest: &String) -> () {
  let manifest = JSON::from_str::<JSON::Value>(&json_manifest).unwrap();
  let schema = get_schema();
  let result = schema.validate(&manifest);
  
  if let Err(errors) = result {
    let mut error_msg: String = String::new();
    for error in errors {
      let str = format!("Validation error: {}\\nInstance path: {}", error, error.instance_path);
      error_msg += &str;
    }
    panic!("{}", error_msg);
  }
}

pub fn deserialize_manifest(json_manifest: String) -> MonowrapManifest {
  validate_manifest(&json_manifest);
  JSON::from_str::<MonowrapManifest>(&json_manifest).unwrap()
}
`;
}

export function main(): void {
  const jsonSchema = fs.readFileSync(
    path.join(__dirname, "..", "src", "schemas", "manifest.json"),
    { encoding: "utf-8" }
  );
  const rendered = renderRustManifest(jsonSchema)

  fs.writeFileSync(path.join(__dirname, "..", "src", "manifest.rs"), rendered, {
    encoding: "utf-8",
  });
}

main()
