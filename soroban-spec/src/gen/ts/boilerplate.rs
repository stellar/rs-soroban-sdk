#![allow(non_snake_case)]
use heck::{ToLowerCamelCase, ToShoutySnakeCase};
use include_dir::{include_dir, Dir};
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
    /// Initialize a new JS client project, updating placeholder strings in the template and
    /// appending functions for each method in the contract to the index.ts file.
    ///
    /// # Arguments
    ///
    /// * `contract_name` - The colloquial name of this contract that will be used in the README and package.json
    /// * `contract_id` - The ID/address of the contract on the network. Will be overridable with environment variables.
    /// * `rpc_url` - The RPC URL of the network where this contract is deployed. Will be overridable with environment variables.
    /// * `network_passphrase` - The passphrase of the network where this contract is deployed. Will be overridable with environment variables.
    /// * `spec` - The contract specification.
    pub fn init(
        &self,
        contract_name: &str,
        contract_id: &str,
        rpc_url: &str,
        network_passphrase: &str,
        spec: &[ScSpecEntry],
    ) -> std::io::Result<()> {
        self.replace_placeholder_patterns(contract_name, contract_id, rpc_url, network_passphrase)?;
        self.append_index_ts(spec)
    }

    fn replace_placeholder_patterns(
        &self,
        contract_name: &str,
        contract_id: &str,
        rpc_url: &str,
        network_passphrase: &str,
    ) -> std::io::Result<()> {
        let replacement_strings = &[
            ("INSERT_CONTRACT_NAME_HERE", contract_name),
            (
                "INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE",
                &contract_name.to_shouty_snake_case(),
            ),
            (
                "INSERT_CAMEL_CASE_CONTRACT_NAME_HERE",
                &contract_name.to_lower_camel_case(),
            ),
            ("INSERT_CONTRACT_ID_HERE", contract_id),
            ("INSERT_NETWORK_PASSPHRASE_HERE", network_passphrase),
            ("INSERT_RPC_URL_HERE", rpc_url),
        ];
        let root: &Path = self.as_ref();
        [
            "package.json",
            "README.md",
            "src/constants.ts",
            "src/env.d.ts",
            "src/index.ts",
            "src/invoke.ts",
        ]
        .into_iter()
        .try_for_each(|file_name| {
            let file = &root.join(file_name);
            let mut contents = fs::read_to_string(file).unwrap();
            replacement_strings
                .iter()
                .for_each(|(pattern, replacement)| {
                    contents = contents.replace(pattern, replacement);
                });
            fs::write(file, contents)
        })
    }

    fn append_index_ts(&self, spec: &[ScSpecEntry]) -> std::io::Result<()> {
        fs::OpenOptions::new()
            .append(true)
            .open(self.0.join("src/index.ts"))?
            .write_all(generate(spec).as_bytes())
    }
}

#[cfg(test)]
mod test {
    use assert_fs::TempDir;

    use super::*;

    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/test_contract_data.wasm");

    fn init(root: impl AsRef<Path>) -> std::io::Result<Project> {
        let spec = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let p: Project = root.as_ref().to_path_buf().try_into()?;
        p.init(
            "contract-data-example",
            "C…",
            "https://rpc-futurenet.stellar.org:443/soroban/rpc",
            "Test SDF Future Network ; October 2022",
            &spec,
        )
        .unwrap();
        Ok(p)
    }

    #[test]
    fn test_project_dir_location() {
        let temp_dir = TempDir::new().unwrap();
        let _: Project = init(&temp_dir).unwrap();
    }

    #[test]
    fn test_project_dir_location_non_temp() {
        let root = PathBuf::from("./root");
        std::fs::remove_dir_all(&root).unwrap_or_default();
        std::fs::create_dir(&root).unwrap();
        let project: Project = init(&root).unwrap();
        println!(
            "{}",
            fs::read_to_string(project.0.join("package.json")).unwrap()
        );
        std::fs::remove_dir_all(&root).unwrap();
    }
}
