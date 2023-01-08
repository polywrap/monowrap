import fs from "fs";
import path from "path";
import { execSync } from "child_process";

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

export function renderAndWriteRustManifest() {
  const jsonSchemaPath = path.join(
    __dirname,
    "..",
    "src",
    "schemas",
    "manifest.json"
  );
  const jsonSchema = fs.readFileSync(jsonSchemaPath, { encoding: "utf-8" });
  const targetFilename = path.join(__dirname, "..", "src", "manifest.rs");
  const rendered = renderRustManifest(jsonSchema);

  fs.writeFileSync(targetFilename, rendered, {
    encoding: "utf-8",
  });
  console.log(`✅ Wrote Rust schema to "${targetFilename}"`);
}

function renderAndWriteGraphQLManifest(): void {
  const jsonSchemaPath = path.join(
    __dirname,
    "..",
    "src",
    "schemas",
    "manifest.json"
  );
  const targetFilename = path.join(
    __dirname,
    "..",
    "src",
    "schemas",
    "manifest.graphql"
  );

  execSync(`json-schema-to-graphql ${jsonSchemaPath} ${targetFilename}`, {
    encoding: "utf-8",
  });

  let output = fs.readFileSync(targetFilename, { encoding: "utf-8" });

  // replace CommandArgs with JSON
  output = output.replace(/args: CommandArgs!/g, "args: JSON!");
  // Remove unnecessary union types
  output = output.replace(/}[^{]*union CommandArgs/g, "}");
  console.log(output)

  fs.writeFileSync(targetFilename, output);
  console.log(`✅ Wrote GraphQL schema to "${targetFilename}"`);
}

export function main(): void {
  renderAndWriteRustManifest();
  renderAndWriteGraphQLManifest();
}

main();
