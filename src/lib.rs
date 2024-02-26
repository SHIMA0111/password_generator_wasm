use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use wasm_bindgen::JsCast;
use web_sys::HtmlLiElement;
use yew::prelude::*;
use crate::components::model::{GeneratedPassword, get_value_by_id, get_value_by_name};

mod components;

const ENABLE_SYMBOLS: &str = "~!@#$%^&*()_+-={}[]|:;\"<>,.?\\/";

#[function_component(App)]
fn app() -> Html {
    let passwords = use_state(|| { GeneratedPassword::new()});
    let generate_password = use_callback(
        passwords.clone(),
        |mouse_event: MouseEvent, clone_state| {
            mouse_event.prevent_default();

            let password_length: usize = match get_value_by_id("password-length") {
                Ok(mut password_length) => {
                    if password_length < 5 {
                        console_error("'password length' should be greater than or equal 5. Automatically convert the length to 5.");
                        password_length = 5;
                    }
                    else if password_length > 1000 {
                        console_error("'password length' should be lower than or equal 1000. Automatically convert the length to 1000.");
                        password_length = 1000;
                    }
                    let password_length_usize = match convert_isize_to_usize(password_length) {
                        Ok(password_length_usize) => {
                            password_length_usize
                        },
                        Err(e) => {
                            console_error(e.as_str());
                            5
                        }
                    };
                    password_length_usize
                },
                Err(e) => {
                    console_error(e.as_str());
                    console_error("'password_length' is invalid so automatically assign 5 as 'password_length'");
                    5
                },
            };
            let number_of_passwords: usize = match get_value_by_id("password-number") {
                Ok(mut number_of_passwords) => {
                    if number_of_passwords < 1 {
                        console_error("'number of passwords' should be greater than or equal 1. Automatically convert the number to 1.");
                        number_of_passwords = 1;
                    }
                    else if password_length > 1000 {
                        console_error("'number of passwords' should be lower than or equal 1000. Automatically convert the number to 1000.");
                        number_of_passwords = 1000;
                    }

                    let number_of_passwords_usize = match convert_isize_to_usize(number_of_passwords) {
                        Ok(number_of_passwords_usize) => {
                            number_of_passwords_usize
                        },
                        Err(e) => {
                            console_error(e.as_str());
                            1
                        }
                    };
                    number_of_passwords_usize
                },
                Err(e) => {
                    console_error(e.as_str());
                    console_error("'number_of_passwords' is invalid so automatically assign 1 as 'number_of_passwords'");
                    1
                }
            };

            let enable_symbols = match get_value_by_name("symbols") {
                Ok(enable_symbols) => {
                    enable_symbols.concat()
                },
                Err(e) => {
                    console_error(e.as_str());
                    "".to_string()
                }
            };

            match GeneratedPassword::generate_password(password_length, number_of_passwords, enable_symbols) {
                Ok(passwords) => clone_state.set(passwords),
                Err(e) => console_error(e.as_str())
            }
        }
    );

    html! {
        <div class="wrapper">
            <section class="password-generator">
                <header>
                    <h1>{ "Password Generator" }</h1>
                    <form>
                        <div class="input-wrapper">
                            <div class="password-length-input">
                                <label for="password-length">{ "Password Length(Max: 1000): " }</label>
                                <input type="number" id="password-length" name="password-length" min=2 max=1000 required=true value={passwords.password_length.to_string()} />
                            </div>
                            <div class="password-number-input">
                                <label for="password-number">{ "Number of Passwords(Max: 1000): " }</label>
                                <input type="number" id="password-number" name="password-number" min=1 max=1000 required=true value={passwords.num_of_passwords.to_string()} />
                            </div>
                        </div>
                        <div class="checkbox-description">{"Include Symbols (Please check symbols you want to include the character set for generating password)"}</div>
                        <div class="checkbox-wrapper">
                            {
                                for ENABLE_SYMBOLS.chars().into_iter().map(|char| {
                                    let is_selected = passwords.enabled_symbols.contains(char.to_string().as_str());
                                    html! {
                                        <div class="symbols-checkbox">
                                            <label><input type="checkbox" name="symbols" checked={ is_selected } value={ char.to_string() } />{ char.to_string() }</label>
                                        </div>
                                    }
                                })
                            }
                        </div>
                        <div class="generate-password-button">
                            <button onclick={generate_password} type="submit">{ "Generate Password" }</button>
                        </div>
                    </form>
                </header>
                <section class="display-password">
                    <ul class="password-list">
                        { for (*passwords).passwords.iter().map(|password| {
                            let password_clone = (*password).clone();
                            html! {
                                <li onclick={Callback::from(move |e: MouseEvent| {
                                    if let Some(target) = e.target() {
                                        if let Some(element) = target.dyn_ref::<HtmlLiElement>() {
                                            select_text(&element);
                                        }
                                    }
                                })}>{password_clone}</li>
                            }
                        }) }
                    </ul>
                </section>
                <footer>
                    <p>{ "© 2024 " }<a href="https://github.com/SHIMA0111">{"SHIMA"}</a></p>
                </footer>
            </section>
        </div>
    }
}

fn select_text(element: &HtmlLiElement) {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let range = document.create_range().unwrap();
    range.select_node_contents(element).unwrap();
    let selection = document.get_selection().unwrap().unwrap();
    let _ = selection.remove_all_ranges();
    let _ = selection.add_range(&range);
}

fn convert_isize_to_usize(value: isize) -> Result<usize, String> {
    value.try_into().map_err(|_| format!("Cannot convert negative isize: {} to usize", value))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

fn console_error(s: &str) {
    error(s);
}

#[wasm_bindgen(start)]
fn rust_app() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
