#![allow(non_snake_case)]
use heck::ToShoutySnakeCase;
use include_dir::{include_dir, Dir};
use serde::Serialize;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use stellar_xdr::ScSpecEntry;

use super::generate;

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/gen/ts/project_template");

pub struct Project(PathBuf);

impl TryInto<Project> for PathBuf {
    type Error = std::io::Error;

    fn try_into(self) -> Result<Project, Self::Error> {
        PROJECT_DIR.extract(&self)?;
        Ok(Project(self))
    }
}

impl AsRef<Path> for Project {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Project {
    pub fn init(
        &self,
        contract_name: &str,
        contract_id: &str,
        spec: &[ScSpecEntry],
    ) -> std::io::Result<()> {
        self.add_contract_id(contract_name, contract_id)?;
        self.add_interface(contract_name)?;
        self.update_package_json(contract_name)?;
        self.append_index_ts(spec)
    }

    fn append_index_ts(&self, spec: &[ScSpecEntry]) -> std::io::Result<()> {
        fs::OpenOptions::new()
            .append(true)
            .open(self.0.join("src/index.ts"))?
            .write_all(generate(spec).as_bytes())
    }

    fn add_contract_id(&self, contract_name: &str, contract_id: &str) -> std::io::Result<()> {
        let root: &Path = self.as_ref();
        let CONTRACT_NAME = contract_name.to_shouty_snake_case();
        fs::OpenOptions::new()
            .append(true)
            .open(root.join("src/constants.ts"))?
            .write_all(
                format!(
                    r#"
/**
 * The Soroban contract ID for the `{contract_name}` contract.
 * 
 * You can override this by setting a `SOROBAN_{CONTRACT_NAME}_CONTRACT_ID` or
 * `PUBLIC_SOROBAN_{CONTRACT_NAME}_CONTRACT_ID` environment variable.
 */
export const CONTRACT_ID = import.meta.env.PUBLIC_SOROBAN_{CONTRACT_NAME}_CONTRACT_ID
    ?? import.meta.env.SOROBAN_{CONTRACT_NAME}_CONTRACT_ID
    ?? '{contract_id}'
"#
                )
                .as_bytes(),
            )
    }

    fn add_interface(&self, contract_name: &str) -> std::io::Result<()> {
        let CONTRACT_NAME = contract_name.to_shouty_snake_case();
        let root: &Path = self.as_ref();

        fs::OpenOptions::new()
            .append(true)
            .open(root.join("src/env.d.ts"))?
            .write_all(
                format!(
                    r#"
interface ImportMetaEnv {{
    readonly PUBLIC_SOROBAN_{CONTRACT_NAME}_CONTRACT_ID: string;
    readonly SOROBAN_{CONTRACT_NAME}_CONTRACT_ID: string;
    
    readonly PUBLIC_SOROBAN_NETWORK_NAME: string;
    readonly SOROBAN_NETWORK_NAME: string;
    
    readonly PUBLIC_SOROBAN_NETWORK_PASSPHRASE: string;
    readonly SOROBAN_NETWORK_PASSPHRASE: string;
    
    readonly PUBLIC_SOROBAN_RPC_URL: string;
    readonly SOROBAN_RPC_URL: string;
}}
"#
                )
                .as_bytes(),
            )
    }

    fn update_package_json(&self, contract_name: &str) -> std::io::Result<()> {
        let p = self.0.join("package.json");
        let mut value: serde_json::Value = fs::read_to_string(&p)?.parse()?;
        let obj = value.as_object_mut().unwrap();
        obj.insert(
            "name".to_owned(),
            serde_json::Value::String(contract_name.to_owned()),
        );
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        obj.serialize(&mut ser).unwrap();
        fs::write(&p, &buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use assert_fs::TempDir;

    use super::*;

    const EXAMPLE_WASM: &[u8] =
    include_bytes!("../../../../../soroban-abundance-token/target/wasm32-unknown-unknown/release/abundance_token.wasm");

    fn init(
        root: impl AsRef<Path>,
        contract_name: &str,
        contract_id: &str,
    ) -> std::io::Result<Project> {
        let spec = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let p: Project = root.as_ref().to_path_buf().try_into()?;
        p.init(contract_name, contract_id, &spec);
        Ok(p)
    }

    #[test]
    fn test_project_dir_location() {
        let temp_dir = TempDir::new().unwrap();
        let project: Project = init(
            &temp_dir,
            "abundance-toke",
            "2c6c3b8ba9923d029d8ef7eb80080384b1da32bcf0698290119fdfbf3f2a01de",
        )
        .unwrap();
    }

    #[test]
    fn test_project_dir_location_non_temp() {
        let root = PathBuf::from("./root");
        std::fs::remove_dir_all(&root).unwrap_or_default();
        std::fs::create_dir(&root).unwrap();
        let project: Project = init(
            &root,
            "abundance-toke",
            "2c6c3b8ba9923d029d8ef7eb80080384b1da32bcf0698290119fdfbf3f2a01de",
        )
        .unwrap();
        println!(
            "{}",
            fs::read_to_string(project.0.join("package.json")).unwrap()
        );
    }
}
