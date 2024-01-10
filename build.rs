use anyhow::{Context, Result};
use convert_case::Casing;
use scilla_parser::Contract;
use scilla_parser::Field;
use scilla_parser::FieldList;
use scilla_parser::Transition;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn scilla_type_to_rust(scilla_type: &scilla_parser::Type) -> String {
    match scilla_type {
        scilla_parser::Type::Int32 => "i32".to_string(),
        scilla_parser::Type::Int64 => "i64".to_string(),
        scilla_parser::Type::Int128 => "i128".to_string(),
        scilla_parser::Type::Int256 => "primitive_types::I256".to_string(),
        scilla_parser::Type::Uint32 => "u32".to_string(),
        scilla_parser::Type::Uint64 => "u64".to_string(),
        scilla_parser::Type::Uint128 => "u128".to_string(),
        scilla_parser::Type::Uint256 => "primitive_types::U256".to_string(),
        scilla_parser::Type::String => "String".to_string(),
        scilla_parser::Type::BNum => "crate::core::BNum".to_string(),
        scilla_parser::Type::Map(key, value) => format!(
            "std::collections::HashMap<{}, {}>",
            scilla_type_to_rust(key),
            scilla_type_to_rust(value)
        ),
        scilla_parser::Type::ByStr(x) if *x == 20 => "ZilAddress".to_string(),
        scilla_parser::Type::ByStr(_) => "String".to_string(),
        scilla_parser::Type::Other(_) => {
            add_to_log(&format!(
                "Failed to map {:?} to any rust type. `ScillaVariable` is used instead.",
                scilla_type
            ));
            "ScillaVariable".to_string()
        }
        scilla_parser::Type::Bool => "bool".to_string(),
        scilla_parser::Type::Option(t) => format!("Option<{}>", scilla_type_to_rust(t)),
        scilla_parser::Type::Pair(a, b) => {
            format!("({}, {})", scilla_type_to_rust(a), scilla_type_to_rust(b))
        }
        scilla_parser::Type::List(t) => format!("Vec<{}>", scilla_type_to_rust(t)),
    }
}

fn transition_to_rust_function(transition: &Transition) -> String {
    let transition_name_snake = transition.name.to_case(convert_case::Case::Snake);
    format!(
        r#"
    pub fn {transition_name_snake}(&self {}) -> RefMut<'_, transition_call::TransitionCall<T>> {{
        self.{transition_name_snake}.borrow_mut().args(vec![{}]);
        self.{transition_name_snake}.borrow_mut()
    }}
"#,
        fields_to_parameters_of_functions_signature(&transition.params),
        fields_to_values(&transition.params)
    )
}

fn fields_to_contract_state_struct(fields: &FieldList) -> String {
    fields
        .iter()
        .map(|field| format!("    pub {}: ScillaValue,", field.name))
        .fold("".to_string(), |acc, e| format!("{acc}\n{e}"))
}

fn get_contract_init_fields_getters(init_params: &FieldList) -> String {
    init_params
        .iter()
        .map(|field| {
            let rust_type = scilla_type_to_rust(&field.r#type);
            // If rust type is `ScillaVariable` it means we couldn't map the scilla type to a rust one. So we consider it as a string
            let rust_type = if rust_type == "ScillaVariable" {
                "String".to_string()
            } else {
                rust_type
            };
            let field_name = &field.name;
            format!(
                r#"
    pub async fn {field_name}(&self) -> Result<{rust_type}, Error> {{
        self.base.get_init()
            .await?
            .iter()
            .find(|value| value.vname == "{field_name}").ok_or(Error::NoSuchFieldInContractInit("{field_name}".to_string()))?
            .value
            .parse().map_err(|_| Error::FailedToParseContractField("{field_name}".to_string()))
    }}"#,
            )
        })
        .fold("".to_string(), |acc, e| format!("{acc}\n{e}"))
}

fn field_to_function_param(field: &Field) -> String {
    let field_name = field.name.to_case(convert_case::Case::Snake);
    let rust_type = scilla_type_to_rust(&field.r#type);
    format!("{field_name}: {rust_type}",)
}

fn transitions_as_struct_fields(transitions: &Vec<Transition>) -> String {
    transitions
        .iter()
        .map(|tr| format!("{}: RefCell<TransitionCall<T>>,", tr.name.to_case(convert_case::Case::Snake)))
        .reduce(|acc, e| format!("{acc}\n    {e}"))
        .unwrap_or_default()
}

fn fields_to_parameters_of_functions_signature(params: &FieldList) -> String {
    params
        .iter()
        .map(|field| field_to_function_param(&field))
        .fold("".to_string(), |acc, e| format!("{acc}, {e}"))
}

fn fields_to_values(params: &FieldList) -> String {
    params.iter().fold("".to_string(), |acc, e| {
        let delim = if acc.is_empty() { "" } else { ", " };
        let rust_type = scilla_type_to_rust(&e.r#type);
        match rust_type.as_str() {
            "ScillaVariable" => {
                format!(r#"{acc}{delim}{} "#, e.name.to_case(convert_case::Case::Snake))
            }
            _ => {
                format!(
                    r#"{acc}{delim}ScillaVariable::new("{}".to_string(), "{}".to_string(), {}.to_value()) "#,
                    e.name,
                    e.r#type,
                    e.name.to_case(convert_case::Case::Snake)
                )
            }
        }
    })
}

fn transitions_to_transition_call_object(transitions: &Vec<Transition>) -> String {
    transitions
        .iter()
        .map(|tr| {
            format!(
                "{}: RefCell::new(TransitionCall::new(\"{}\", &base.address, base.client.clone())),",
                tr.name.to_case(convert_case::Case::Snake),
                tr.name
            )
        })
        .reduce(|acc, e| format!("{acc}\n            {e}"))
        .unwrap_or_default()
}

fn to_string_for_contract_field_getters(contract_fields: &FieldList, contract_name: &str) -> String {
    contract_fields.iter()
            .map(|field| {
                let rust_type = scilla_type_to_rust(&field.r#type);
                format!(
                    "    pub async fn {}(&self) -> Result<{rust_type}, Error> {{\n        self.base.get_state::<{contract_name}State>().await?.{}.try_into_rust_type()\n    }}",
                    field.name, field.name
                )
            })
            .fold("".to_string(), |acc, e| format!("{acc}\n{e}"))
}

fn generate_rust_binding(contract: &Contract, contract_path: &Path) -> Result<String> {
    let contract_name = &contract.name;
    let transitions_as_fields = transitions_as_struct_fields(&contract.transitions);
    let contract_deployment_params = fields_to_parameters_of_functions_signature(&contract.init_params);
    let contract_deployment_params_for_init = fields_to_values(&contract.init_params);
    let transitions_for_new_function = transitions_to_transition_call_object(&contract.transitions);
    let contract_field_getters = to_string_for_contract_field_getters(&contract.fields, &contract_name);
    let contract_fields_for_state_struct = fields_to_contract_state_struct(&contract.fields);
    let contract_init_field_getters = get_contract_init_fields_getters(&contract.init_params);
    let contract_init_fields_for_init_struct = fields_to_contract_state_struct(&contract.init_params);
    let transitions = contract
        .transitions
        .iter()
        .map(|tr| transition_to_rust_function(tr))
        .fold("".to_string(), |acc, e| format!("{acc}{e}"));

    Ok(format!(
        r#"#[derive(Debug)]
pub struct {contract_name}<T: Middleware> {{
    pub base: BaseContract<T>,
    {transitions_as_fields}
}}

impl<T: Middleware> {contract_name}<T> {{
    pub async fn deploy(client: Arc<T> {contract_deployment_params}) -> Result<Self, Error> {{
        let factory = ContractFactory::new(client.clone());
        let init = Init(vec![
            ScillaVariable::new("_scilla_version".to_string(), "Uint32".to_string(), "0".to_value()),
            {contract_deployment_params_for_init}
        ]);

        Ok(Self::new(factory.deploy_from_file(&std::path::PathBuf::from({contract_path:?}), init, None).await?))
    }}

    pub fn address(&self) -> &ZilAddress  {{
        &self.base.address
    }}

    pub fn new(base: BaseContract<T>) -> Self {{
        Self{{
            {transitions_for_new_function}
            base,
        }}
    }}
    {transitions}{contract_field_getters}{contract_init_field_getters}
    pub async fn get_state(&self) -> Result<{contract_name}State, Error> {{
        self.base.get_state().await
    }}
}}

#[derive(serde::Deserialize, Debug)]
pub struct {contract_name}State {{{contract_fields_for_state_struct}
}}

#[derive(serde::Deserialize, Debug)]
pub struct {contract_name}Init {{{contract_init_fields_for_init_struct}
}}
"#
    ))
}

fn add_to_log(log: &str) {
    let mut file = OpenOptions::new().append(true).create(true).open("/tmp/log.txt").unwrap();
    writeln!(file, "{}", log).unwrap();
}

fn generate(contracts_path: &Path) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("scilla_contracts.rs");

    let mut file = std::fs::File::create(&dest_path).context(format!("Failed to open {}", dest_path.display()))?;
    for entry in std::fs::read_dir(contracts_path).context("Failed to read files in contracts folder")? {
        let entry = entry.context("Failed to get contract entry")?;
        let path = entry.path();
        if path.is_file() {
            match Contract::parse(&path) {
                Ok(contract) => match generate_rust_binding(&contract, &path) {
                    Ok(code) => writeln!(file, "{code}").unwrap(),
                    Err(e) => {
                        add_to_log(&format!("Failed to generate rust binding for {path:?}. {e}"));
                        continue;
                    }
                },
                Err(e) => {
                    add_to_log(&format!("Failed to parse {path:?}. {e}",));
                    continue;
                }
            }
        }
    }

    Ok(())
}

fn generate_empty() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("scilla_contracts.rs");
    std::fs::write(dest_path, "// CONTRACTS_PATH is not set, or does not exist")
        .context("Failed to create empty scilla_contracts.rs file")
}

fn main() -> Result<(), Box<dyn Error>> {
    add_to_log("Start...");
    let contracts_path = env::var("CONTRACTS_PATH").unwrap_or_default();
    if contracts_path.is_empty() {
        add_to_log("CONTRACTS_PATH is not set. Exiting...");
        generate_empty()?;
        return Ok(());
    }

    add_to_log(&format!("Contract path: {}", contracts_path));
    let contracts_path = PathBuf::from(contracts_path);
    if !contracts_path.exists() {
        add_to_log(&format!("{} does not exist. Exiting...", contracts_path.display()));
        generate_empty()?;
        return Ok(());
    }

    if let Err(x) = generate(&contracts_path) {
        add_to_log(&x.to_string())
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CONTRACTS_PATH");
    println!("cargo:rerun-if-changed={}", contracts_path.display());
    Ok(())
}
