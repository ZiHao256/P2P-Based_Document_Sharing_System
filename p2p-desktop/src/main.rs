mod assistance;
mod user;
mod components;

use iced::{Application, Command, Element, executor,  Renderer, Settings};
use serde::de::Error;
use serde::Serialize;
use log::{error, info};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;
use crate::user::user::User;
use crate::components::{home, login, page, register};

const BASE_URL: &str = "http://localhost:8080";

#[tokio::main]
async fn main() {
    env_logger::init();
    P2PAppState::run(Settings::default()).expect("");
}

// ZIHAO: multi-components
pub struct P2PAppState {
    login_state: login::LoginState,
    register_state: register::RegisterState,
    // home_state: home::HomeState,
    current_page: page::PageState,
}


// ZIHAO: multi-components
#[derive(Debug, Clone)]
pub enum P2PAppMessage {
    // Login Messages
    LoginMessage(login::LoginMessage),
    // Register Messages
    RegisterMessage(register::RegisterMessage),
    // Home Messages
    HomeMessage(home::HomeMessage),
    // page Messages
    PageMessage(page::PageMessage)
}

impl Application for P2PAppState {
    type Executor = executor::Default;
    type Message = P2PAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // init
        (
            P2PAppState {
                login_state: login::LoginState {
                    name: "".to_string(),
                    password: "".to_string(),
                },
                register_state: register::RegisterState {
                    name: "".to_string(),
                    password: "".to_string(),
                    confirmed_password: "".to_string(),
                },
                // home_state: home::HomeState{
                //     file_button: button::State::new(),
                //     upload_button: button::State::new(),
                //     search_button: button::State::new(),
                //     setting_button: button::State::new()
                // },
                current_page: page::PageState::LoginPage
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("P2P-Based Document Sharing System")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // Login
            P2PAppMessage::LoginMessage(login_message) => {
                match login_message{
                    login::LoginMessage::Name(name) => {
                        self.login_state.name = name;
                        Command::none()
                    },
                    login::LoginMessage::Password(password) => {
                        self.login_state.password = password;
                        Command::none()
                    },
                    login::LoginMessage::SubmitLogin => {
                        let this_user = User{
                            name: (self.login_state.name).clone(),
                            password: (self.login_state.password).clone()
                        };

                        info!("{} {}", self.login_state.name, self.login_state.password);
                        // ZIHAO: perform asynchorneous actions
                        Command::perform(async move {
                            let client = reqwest::Client::new();
                            let res = client
                                .post(format!("{}/user/login", BASE_URL))
                                .json(&this_user)
                                .send()
                                .await;
                            return match res {
                                Ok(response) => {
                                    let my_http_response = response.json::<MyHttpResponse>().await.unwrap();
                                    info!("{:?}", my_http_response);
                                    Ok(my_http_response)
                                }
                                Err(e) => Err(MyError::ClientError {
                                    code: 1,
                                    message: e.to_string(),
                                })
                            }
                        }, login::LoginMessage::LoginResponse).map(P2PAppMessage::LoginMessage)
                    },
                    login::LoginMessage::LoginResponse(response) => {
                        match response {
                            Ok(my_http_response) => {
                                info!("{my_http_response:?}");
                                if my_http_response.code == 0 {
                                    Command::perform(
                                        async {
                                            format!("test")
                                        },
                                        page::PageMessage::SwitchToHomePage
                                    ).map(P2PAppMessage::PageMessage)
                                } else {
                                    error!("{}", my_http_response.message);
                                    Command::none()
                                }
                            },
                            Err(e) => {
                                error!("{:?}",e);
                                Command::none()
                            }
                        }
                    },
                    login::LoginMessage::EnterRegisterPage => {
                        self.current_page = page::PageState::RegisterPage;
                        Command::none()
                    }
                }
            },

            // Register
            P2PAppMessage::RegisterMessage(register_message) => {
                match register_message {
                    register::RegisterMessage::Name(name) => {
                        self.register_state.name = name;
                        Command::none()
                    }
                    register::RegisterMessage::Password(password) => {
                        self.register_state.password = password;
                        Command::none()
                    }
                    register::RegisterMessage::ConfirmedPassword(confirmed_password) => {
                        self.register_state.confirmed_password = confirmed_password;
                        Command::none()
                    }
                    register::RegisterMessage::SubmitRegister => {
                        let this_user = User{
                            name: (self.register_state.name).clone(),
                            password: (self.register_state.password).clone()
                        };

                        info!("{} {}", self.register_state.name, self.register_state.password);
                        Command::perform(async move {
                            let client = reqwest::Client::new();
                            let res = client
                                .post(format!("{}/user/register", BASE_URL))
                                .json(&this_user)
                                .send()
                                .await;
                            return match res {
                                Ok(response) => {
                                    info!("{response:?}");
                                    let my_http_response = response.json::<MyHttpResponse>().await.unwrap();
                                    info!("{:?}", my_http_response);
                                    Ok(my_http_response)
                                }
                                Err(e) => Err(MyError::ClientError {
                                    code: 1,
                                    message: e.to_string(),
                                })
                            }
                        }, register::RegisterMessage::RegisterResponse).map(P2PAppMessage::RegisterMessage)
                    }
                    register::RegisterMessage::RegisterResponse(response) => {
                        match response {
                            Ok(my_http_response) => {
                                info!("{my_http_response:?}");
                                if my_http_response.code == 0 {
                                    Command::perform(
                                        async {
                                            format!("test")
                                        },
                                        page::PageMessage::SwitchToLoginPage
                                    ).map(P2PAppMessage::PageMessage)
                                } else {
                                    error!("{}", my_http_response.message);
                                    Command::none()
                                }
                            },
                            Err(e) => {
                                error!("{:?}",e);
                                Command::none()
                            }
                        }
                    },
                    register::RegisterMessage::BackLoginPage => {
                        self.current_page = page::PageState::LoginPage;
                        Command::none()
                    }
                }
            }
            // Home
            P2PAppMessage::HomeMessage(home_message) => {
                match home_message {
                    home::HomeMessage::MenuItemSelected(home_message) => {
                        match home_message {
                            home::MenuItem::File => {

                            },

                            home::MenuItem::Upload => {

                            }
                            home::MenuItem::Search => {

                            }
                            home::MenuItem::Setting => {

                            }
                        }
                        Command::none()
                    }
                }
            }

            // Switch Page
            P2PAppMessage::PageMessage(page_message) => {
                match page_message{
                    page::PageMessage::SwitchToRegisterPage => {
                        self.current_page = page::PageState::RegisterPage;
                        Command::none()
                    },
                    page::PageMessage::SwitchToHomePage(_) => {
                        self.current_page = page::PageState::HomePage;
                        Command::none()
                    }
                    page::PageMessage::SwitchToLoginPage(_) => {
                        self.current_page = page::PageState::LoginPage;
                        Command::none()
                    }
                }


            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self.current_page{
            page::PageState::LoginPage => {
                login::login_view(&self)
                    // ZIHAO: Element<LoginMessage> -> Element<P2PAppMessage::LoginMessage>
                    .map(P2PAppMessage::LoginMessage)
            },
            page::PageState::RegisterPage => {
                register::register_view(&self)
                    .map(P2PAppMessage::RegisterMessage)
            },
            page::PageState::HomePage => {
                home::home_view(&self)
                    .map(P2PAppMessage::HomeMessage)
            }
        }

    }
}