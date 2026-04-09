use super::*;

// JSON tests

#[test]
fn json_string_value() {
  Test::new()
    .write("data.json", r#"{"name": "alice"}"#)
    .justfile("x := json('data.json', 'name')")
    .args(["--evaluate", "x"])
    .stdout("alice")
    .success();
}

#[test]
fn json_number_value() {
  Test::new()
    .write("data.json", r#"{"port": 8080}"#)
    .justfile("x := json('data.json', 'port')")
    .args(["--evaluate", "x"])
    .stdout("8080")
    .success();
}

#[test]
fn json_bool_value() {
  Test::new()
    .write("data.json", r#"{"enabled": true}"#)
    .justfile("x := json('data.json', 'enabled')")
    .args(["--evaluate", "x"])
    .stdout("true")
    .success();
}

#[test]
fn json_null_value() {
  Test::new()
    .write("data.json", r#"{"value": null}"#)
    .justfile("x := json('data.json', 'value')")
    .args(["--evaluate", "x"])
    .stdout("null")
    .success();
}

#[test]
fn json_nested_keys() {
  Test::new()
    .write("data.json", r#"{"a": {"b": {"c": "deep"}}}"#)
    .justfile("x := json('data.json', 'a', 'b', 'c')")
    .args(["--evaluate", "x"])
    .stdout("deep")
    .success();
}

#[test]
fn json_array_index() {
  Test::new()
    .write("data.json", r#"{"items": ["foo", "bar", "baz"]}"#)
    .justfile("x := json('data.json', 'items', '1')")
    .args(["--evaluate", "x"])
    .stdout("bar")
    .success();
}

#[test]
fn json_nested_array_and_object() {
  Test::new()
    .write(
      "data.json",
      r#"{"users": [{"name": "alice"}, {"name": "bob"}]}"#,
    )
    .justfile("x := json('data.json', 'users', '1', 'name')")
    .args(["--evaluate", "x"])
    .stdout("bob")
    .success();
}

#[test]
fn json_missing_key() {
  Test::new()
    .write("data.json", r#"{"a": 1}"#)
    .justfile("x := json('data.json', 'b')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: key `b` not found in JSON file `data.json` at key `b`\n.*")
    .failure();
}

#[test]
fn json_file_not_found() {
  Test::new()
    .justfile("x := json('missing.json', 'key')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: I/O error reading `missing.json`: .*")
    .failure();
}

#[test]
fn json_parse_error() {
  Test::new()
    .write("bad.json", "not json at all")
    .justfile("x := json('bad.json', 'key')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: error parsing `bad.json` as JSON: .*")
    .failure();
}

#[test]
fn json_compound_value() {
  Test::new()
    .write("data.json", r#"{"a": {"b": 1}}"#)
    .justfile("x := json('data.json', 'a')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: expected scalar but got compound value in JSON file `data.json` at key `a`\n.*")
    .failure();
}

#[test]
fn json_array_index_not_integer() {
  Test::new()
    .write("data.json", r#"{"items": [1, 2, 3]}"#)
    .justfile("x := json('data.json', 'items', 'abc')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: expected integer index but got `abc` in JSON file `data.json` at key `items`, `abc`\n.*")
    .failure();
}

#[test]
fn json_array_index_out_of_range() {
  Test::new()
    .write("data.json", r#"{"items": [1, 2, 3]}"#)
    .justfile("x := json('data.json', 'items', '99')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: index 99 out of range in JSON file `data.json` at key `items`, `99`\n.*")
    .failure();
}

#[test]
fn json_package_json_version() {
  Test::new()
    .write("package.json", r#"{"name": "my-app", "version": "1.2.3"}"#)
    .justfile("x := json('package.json', 'version')")
    .args(["--evaluate", "x"])
    .stdout("1.2.3")
    .success();
}

// TOML tests

#[test]
fn toml_string_value() {
  Test::new()
    .write(
      "config.toml",
      "[project]\nname = \"my-app\"\nversion = \"0.1.0\"\n",
    )
    .justfile("x := toml('config.toml', 'project', 'version')")
    .args(["--evaluate", "x"])
    .stdout("0.1.0")
    .success();
}

#[test]
fn toml_number_value() {
  Test::new()
    .write("config.toml", "[server]\nport = 3000\n")
    .justfile("x := toml('config.toml', 'server', 'port')")
    .args(["--evaluate", "x"])
    .stdout("3000")
    .success();
}

#[test]
fn toml_bool_value() {
  Test::new()
    .write("config.toml", "[features]\nenabled = true\n")
    .justfile("x := toml('config.toml', 'features', 'enabled')")
    .args(["--evaluate", "x"])
    .stdout("true")
    .success();
}

#[test]
fn toml_array_index() {
  Test::new()
    .write("config.toml", "colors = [\"red\", \"green\", \"blue\"]\n")
    .justfile("x := toml('config.toml', 'colors', '2')")
    .args(["--evaluate", "x"])
    .stdout("blue")
    .success();
}

#[test]
fn toml_missing_key() {
  Test::new()
    .write("config.toml", "[project]\nname = \"x\"\n")
    .justfile("x := toml('config.toml', 'project', 'missing')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `toml` failed: key `missing` not found in TOML file `config.toml` at key `project`, `missing`\n.*")
    .failure();
}

#[test]
fn toml_parse_error() {
  Test::new()
    .write("bad.toml", "not = [valid toml")
    .justfile("x := toml('bad.toml', 'key')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `toml` failed: error parsing `bad.toml` as TOML: .*")
    .failure();
}

#[test]
fn toml_compound_value() {
  Test::new()
    .write("config.toml", "[project]\nname = \"x\"\n")
    .justfile("x := toml('config.toml', 'project')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `toml` failed: expected scalar but got compound value in TOML file `config.toml` at key `project`\n.*")
    .failure();
}

#[test]
fn toml_pyproject_version() {
  Test::new()
    .write(
      "pyproject.toml",
      "[project]\nname = \"my-lib\"\nversion = \"2.0.0\"\n",
    )
    .justfile("x := toml('pyproject.toml', 'project', 'version')")
    .args(["--evaluate", "x"])
    .stdout("2.0.0")
    .success();
}

// YAML tests

#[test]
fn yaml_string_value() {
  Test::new()
    .write("config.yaml", "name: alice\n")
    .justfile("x := yaml('config.yaml', 'name')")
    .args(["--evaluate", "x"])
    .stdout("alice")
    .success();
}

#[test]
fn yaml_number_value() {
  Test::new()
    .write("config.yaml", "port: 8080\n")
    .justfile("x := yaml('config.yaml', 'port')")
    .args(["--evaluate", "x"])
    .stdout("8080")
    .success();
}

#[test]
fn yaml_bool_value() {
  Test::new()
    .write("config.yaml", "debug: true\n")
    .justfile("x := yaml('config.yaml', 'debug')")
    .args(["--evaluate", "x"])
    .stdout("true")
    .success();
}

#[test]
fn yaml_nested_keys() {
  Test::new()
    .write("config.yaml", "env:\n  RUSTFLAGS: \"-D warnings\"\n")
    .justfile("x := yaml('config.yaml', 'env', 'RUSTFLAGS')")
    .args(["--evaluate", "x"])
    .stdout("-D warnings")
    .success();
}

#[test]
fn yaml_array_index() {
  Test::new()
    .write("config.yaml", "items:\n  - foo\n  - bar\n  - baz\n")
    .justfile("x := yaml('config.yaml', 'items', '0')")
    .args(["--evaluate", "x"])
    .stdout("foo")
    .success();
}

#[test]
fn yaml_nested_array_and_object() {
  Test::new()
    .write("config.yaml", "users:\n  - name: alice\n  - name: bob\n")
    .justfile("x := yaml('config.yaml', 'users', '1', 'name')")
    .args(["--evaluate", "x"])
    .stdout("bob")
    .success();
}

#[test]
fn yaml_missing_key() {
  Test::new()
    .write("config.yaml", "a: 1\n")
    .justfile("x := yaml('config.yaml', 'b')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `yaml` failed: key `b` not found in YAML file `config.yaml` at key `b`\n.*")
    .failure();
}

#[test]
fn yaml_parse_error() {
  Test::new()
    .write("bad.yaml", ":\n  - :\n  -: }{")
    .justfile("x := yaml('bad.yaml', 'key')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `yaml` failed: error parsing `bad.yaml` as YAML: .*")
    .failure();
}

#[test]
fn yaml_compound_value() {
  Test::new()
    .write("config.yaml", "env:\n  A: 1\n  B: 2\n")
    .justfile("x := yaml('config.yaml', 'env')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `yaml` failed: expected scalar but got compound value in YAML file `config.yaml` at key `env`\n.*")
    .failure();
}

#[test]
fn yaml_ci_workflow() {
  Test::new()
    .write(
      "ci.yaml",
      "name: CI\non: push\nenv:\n  RUSTFLAGS: --deny warnings\n",
    )
    .justfile("x := yaml('ci.yaml', 'env', 'RUSTFLAGS')")
    .args(["--evaluate", "x"])
    .stdout("--deny warnings")
    .success();
}

// No keys provided

#[test]
fn json_no_keys_compound() {
  Test::new()
    .write("data.json", r#"{"a": 1}"#)
    .justfile("x := json('data.json')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: expected scalar but got compound value in JSON file `data.json`\n.*")
    .failure();
}

// Index into scalar

#[test]
fn json_index_into_scalar() {
  Test::new()
    .write("data.json", r#"{"a": "hello"}"#)
    .justfile("x := json('data.json', 'a', 'b')")
    .args(["--evaluate", "x"])
    .stderr_regex("error: Call to function `json` failed: cannot index into scalar value in JSON file `data.json` at key `a`, `b`\n.*")
    .failure();
}
