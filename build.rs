use anyhow::{anyhow, Context, Result};
use convert_case::Casing;
use lexpr::Value;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug)]
struct Contract {
    path: PathBuf,
    name: String,
    transitions: Vec<Transition>,
    contract_params: FieldList,
}

#[derive(Debug)]
struct Transition {
    name: String,
    params: FieldList,
}

#[derive(Debug)]
struct Field {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug)]
struct FieldList(Vec<Field>);

impl FieldList {
    fn to_string_for_rust_function_signature(&self) -> String {
        self.iter().fold("".to_string(), |acc, e| format!("{acc}, {e}"))
    }

    fn to_string_for_scilla_init(&self) -> String {
        self.iter().fold("".to_string(), |acc, e| {
            let delim = if acc.is_empty() { "" } else { ", " };
            format!(
                r#"{acc}{delim}Value::new("{}".to_string(), "{}".to_string(), {}) "#,
                e.name,
                e.r#type.scilla_type,
                e.name.to_case(convert_case::Case::Snake)
            )
        })
    }
}

impl std::ops::Deref for FieldList {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Type {
    scilla_type: String,
    rust_type: String,
}

impl FromStr for Type {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let rust_type = match s {
            "Int64" => "i64",
            "Int128" => "i128",
            "Int256" => "i256",
            "Uint32" => "u32",
            "Uint64" => "u64",
            "Uint128" => "u128",
            "BNum" => "primitive_types::U256",
            "ByStr20" | "String" => "String",
            _ => return Err(anyhow!("Failed to map {} to any rust type", s)),
        };

        Ok(Self {
            rust_type: rust_type.to_string(),
            scilla_type: s.to_string(),
        })
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contract_name = &self.name;
        let contract_params = self.contract_params.to_string_for_rust_function_signature();
        let contract_params_init = self.contract_params.to_string_for_scilla_init();
        let transitions = self.transitions.iter().fold("".to_string(), |acc, e| format!("{acc}\n{e}"));

        write!(
            f,
            r#"#[derive(Debug)]
pub struct {contract_name}<T: Middleware> {{
    pub base: BaseContract<T>,
}}

impl<T: Middleware> {contract_name}<T> {{
    pub async fn deploy(client: Arc<T> {contract_params}) -> Result<Self, Error> {{
        let factory = ContractFactory::new(client.clone());
        let init = Init(vec![
            Value::new("_scilla_version".to_string(), "Uint32".to_string(), "0".to_string()),
            {contract_params_init}
        ]);

        Ok(Self::new(factory.deploy_from_file(&std::path::PathBuf::from("{}"), init, None).await?))
    }}

    pub fn new(base: BaseContract<T>) -> Self {{
        Self{{base}}
    }}

    {transitions}
}}
"#,
            self.path.display()
        )
    }
}

impl std::fmt::Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
    pub async fn {}(&self {}) -> Result<GetTransactionResponse, Error> {{
        self.base.call("{}", vec![{}]).await
    }}
"#,
            self.name.to_case(convert_case::Case::Snake),
            self.params.to_string_for_rust_function_signature(),
            self.name,
            self.params.to_string_for_scilla_init()
        )
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name.to_case(convert_case::Case::Snake),
            self.r#type.rust_type
        )
    }
}

fn add_to_log(log: &str) {
    let mut file = OpenOptions::new().append(true).create(true).open("/tmp/log.txt").unwrap();
    writeln!(file, "{}", log).unwrap();
}

fn run_scilla_fmt(path: &Path) -> Result<PathBuf> {
    //docker run --rm -v contract.scilla:/tmp/input.scilla  -i zilliqa/scilla:v0.13.3 /scilla/0/bin/scilla-fmt --sexp --human-readable -d /tmp/input.scilla
    let volume = &format!("{}:/tmp/input.scilla", path.canonicalize().unwrap().display());

    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            volume,
            "-i",
            "zilliqa/scilla:v0.13.3",
            "/scilla/0/bin/scilla-fmt",
            "--sexp",
            "--human-readable",
            "-d",
            "/tmp/input.scilla",
        ])
        .output()
        .context("Failed to execute docker command")?;

    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join(format!(
        "{}.sexp",
        path.file_stem()
            .context("Failed to get file stem")?
            .to_str()
            .context("Failed to convert to string")?
    ));
    std::fs::write(&dest_path, String::from_utf8(output.stdout)?)?;

    Ok(dest_path)
}

fn parse_sexp(sexp_path: &Path, contract_path: PathBuf) -> Result<Contract> {
    let sexp = std::fs::read_to_string(sexp_path).context(format!("Failed to read {} to string", sexp_path.display()))?;
    let v = lexpr::from_str(&sexp).context("Failed to parse sexp")?;
    let name = v["contr"][0]["cname"]["Ident"][0][1].to_string();
    let transitions = extract_transitions(&v["contr"][0]["ccomps"])?;
    let contract_params = parse_fields(&v["contr"][0]["cparams"][0])?;
    Ok(Contract {
        path: contract_path.canonicalize().context("Failed to canonicalize contract path")?,
        name,
        transitions,
        contract_params,
    })
}

fn extract_transitions(ccomps: &Value) -> Result<Vec<Transition>> {
    let mut transitions = vec![];
    for elem in ccomps[0].list_iter().unwrap() {
        let transition_name = elem["comp_name"][0]["SimpleLocal"][0].to_string();
        transitions.push(Transition {
            name: transition_name,
            params: parse_fields(&elem["comp_params"][0])?,
        })
    }

    Ok(transitions)
}

fn parse_fields(cparams: &Value) -> Result<FieldList> {
    let mut params = vec![];
    for elem in cparams.list_iter().unwrap() {
        let name = elem[0]["SimpleLocal"][0].to_string();
        let r#type = elem[1][1].to_string().parse()?;
        params.push(Field { name, r#type })
    }

    Ok(FieldList(params))
}

fn generate(contracts_path: PathBuf) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("scilla_contracts.rs");

    let mut file = std::fs::File::create(&dest_path).context(format!("Failed to open {}", dest_path.display()))?;
    for entry in std::fs::read_dir(contracts_path).context("Failed to read files in contracts folder")? {
        let entry = entry.context("Failed to get contract entry")?;
        let path = entry.path();
        if path.is_file() {
            match run_scilla_fmt(&path) {
                Ok(sexp_path) => {
                    let contract = parse_sexp(&sexp_path, path)?;
                    add_to_log(&format!("Parsed: {:?}", contract));
                    writeln!(file, "{}", contract)?;
                }
                Err(_) => {
                    add_to_log(&format!("Failed to call scilla_fmt for {}", path.display()));
                    continue;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    add_to_log("Start...");
    let contracts_path = env::var("CONTRACTS_PATH")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("contracts"));

    add_to_log(&format!("Contract path: {}", contracts_path.display()));
    if let Err(x) = generate(contracts_path) {
        add_to_log(&x.to_string())
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=contracts");
    Ok(())
}
