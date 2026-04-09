use {super::*, serde_json::Value};

pub(crate) enum Format {
  Json,
  Toml,
  Yaml,
}

impl Format {
  fn name(&self) -> &'static str {
    match self {
      Self::Json => "JSON",
      Self::Toml => "TOML",
      Self::Yaml => "YAML",
    }
  }
}

pub(crate) fn index(
  context: function::Context,
  filename: &str,
  keys: &[String],
  format: Format,
) -> FunctionResult {
  let path = context.execution_context.working_directory().join(filename);

  let content =
    fs::read_to_string(&path).map_err(|err| format!("I/O error reading `{filename}`: {err}"))?;

  let value: Value = match format {
    Format::Json => serde_json::from_str(&content)
      .map_err(|err| format!("error parsing `{filename}` as JSON: {err}"))?,
    Format::Toml => toml::from_str(&content)
      .map_err(|err| format!("error parsing `{filename}` as TOML: {err}"))?,
    Format::Yaml => serde_yml::from_str(&content)
      .map_err(|err| format!("error parsing `{filename}` as YAML: {err}"))?,
  };

  let format_name = format.name();
  let mut current = &value;

  for (i, key) in keys.iter().enumerate() {
    let path_hint = format_key_path(filename, format_name, &keys[..=i]);
    current = match current {
      Value::Object(map) => map
        .get(key.as_str())
        .ok_or_else(|| format!("key `{key}` not found in {path_hint}"))?,
      Value::Array(arr) => {
        let index: usize = key
          .parse()
          .map_err(|_| format!("expected integer index but got `{key}` in {path_hint}"))?;
        arr
          .get(index)
          .ok_or_else(|| format!("index {index} out of range in {path_hint}"))?
      }
      _ => {
        return Err(format!("cannot index into scalar value in {path_hint}"));
      }
    };
  }

  match current {
    Value::Null => Ok("null".to_owned()),
    Value::Bool(b) => Ok(b.to_string()),
    Value::Number(n) => Ok(n.to_string()),
    Value::String(s) => Ok(s.clone()),
    Value::Array(_) | Value::Object(_) => {
      let path_hint = format_key_path(filename, format_name, keys);
      Err(format!(
        "expected scalar but got compound value in {path_hint}"
      ))
    }
  }
}

fn format_key_path(filename: &str, format_name: &str, keys: &[String]) -> String {
  if keys.is_empty() {
    format!("{format_name} file `{filename}`")
  } else {
    let keys_str = keys
      .iter()
      .map(|k| format!("`{k}`"))
      .collect::<Vec<_>>()
      .join(", ");
    format!("{format_name} file `{filename}` at key {keys_str}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn format_key_path_empty() {
    assert_eq!(
      format_key_path("foo.json", "JSON", &[]),
      "JSON file `foo.json`"
    );
  }

  #[test]
  fn format_key_path_single() {
    assert_eq!(
      format_key_path("foo.json", "JSON", &["bar".into()]),
      "JSON file `foo.json` at key `bar`"
    );
  }

  #[test]
  fn format_key_path_multiple() {
    assert_eq!(
      format_key_path("foo.toml", "TOML", &["a".into(), "b".into(), "c".into()]),
      "TOML file `foo.toml` at key `a`, `b`, `c`"
    );
  }
}
