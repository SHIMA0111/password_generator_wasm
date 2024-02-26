use web_sys::{window, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::components::password_generator::{Args, generate_password};

#[derive(PartialEq)]
pub(crate) struct GeneratedPassword {
    pub(crate) passwords: Vec<String>,
    pub(crate) password_length: usize,
    pub(crate) num_of_passwords: usize,
    pub(crate) enabled_symbols: String,
}

impl GeneratedPassword {
    pub(crate) fn new() -> Self {
        Self {
            passwords: Vec::new(),
            password_length: 12,
            num_of_passwords: 5,
            enabled_symbols: "".to_string(),
        }
    }

    pub(crate) fn generate_password(password_length: usize, num_of_passwords: usize, enabled_symbols: String) -> Result<Self, String> {
        if password_length > 1000 || num_of_passwords > 1000 {
            return Err("Password length and number of passwords should be less than or equal to 1000".to_string());
        }

        let args = Args::new(password_length, num_of_passwords, enabled_symbols.clone());
        let passwords = generate_password(&args).map_err(|e| format!("Failed to generate password due to {}", e.to_string()))?;

        Ok(Self {
            passwords,
            password_length,
            num_of_passwords,
            enabled_symbols,
        })
    }
}

pub(crate) fn get_value_by_id(id: &str) -> Result<isize, String> {
    let window = window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");
    let element = document.get_element_by_id(id);

    if let Some(input_element) = element {
        let input_element: HtmlInputElement = input_element.dyn_into().unwrap();
        let input_data= input_element.value();
        match input_data.parse::<isize>() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Invalid value for {}", id)),
        }
    } else {
        Err(format!("id: {} does not exist", id))
    }
}

pub(crate) fn get_value_by_name(name: &str) -> Result<Vec<String>, String> {
    let document = window().unwrap().document().unwrap();
    let elements = document.get_elements_by_name(name);

    let mut values = Vec::new();
    if !(elements.length() == 0) {
        for i in 0..elements.length() {
            if let Some(element) = elements.item(i) {
                if let Some(input_element) = JsCast::dyn_ref::<HtmlInputElement>(&element) {
                    let value = input_element.value();
                    if input_element.checked() {
                        values.push(value);
                    }
                } else {
                    return Err("Getting empty value from checkbox. Please contact developer.".to_string())
                }
            } else {
                return Err("Element cannot extract".to_string())
            }
        }
        Ok(values)
    } else {
        Ok(values)
    }
}
