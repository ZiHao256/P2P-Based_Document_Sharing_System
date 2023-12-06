mod assistance;
mod user;
mod components;

use iced::{Application, Command, Element, executor,  Renderer, Settings};
use serde::de::Error;
use serde::Serialize;
use log::{error, info};
use reqwest::Client;
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;
use crate::assistance::metadata::METADATA;
use crate::user::user::User;
use crate::components::{home, login, page, register};
use crate::components::home::file;
use crate::P2PAppMessage::HomeMessage;

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
    home_state: home::HomeState,
    current_page: page::PageState,
    // ZIHAO: reuse client, reqwest::Client uses an Arc internally
    client: Client
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
                home_state: home::HomeState{
                    current_child_page: 0,
                    file_state: home::file::FileState {
                        uploaded_metadata: vec![],
                    },
                    upload_state: home::upload::UploadState {
                        metadata_to_upload: METADATA {
                            id: 0,
                            user_name: "".to_string(),
                            name: "".to_string(),
                            size: 0.0,
                            path: "".to_string(),
                            ip: "".to_string(),
                            port: "".to_string(),
                        },
                    },
                    search_state: home::search::SearchState {
                        file_name_to_search: "".to_string(),
                        searched_metadata: vec![],
                    },
                },
                current_page: page::PageState::LoginPage,
                client: Client::new()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("P2P-Based Document Sharing System")
    }

    /*
    // ZIHAO: Self::Message & update如何交互：
    //      封装消息：两种方式，都是首先创建一个Message变体实例，然后映射为主App的Message实例，再调用update方法
    //          1. view方法中：通过用户触发widget组件，来获得消息实例；
                    通过Element中的map方法，将子组件Message类型映射为主App的Message类型
    //          2. update方法中：Command::perform将异步代码块执行结果，封装为消息实例
    //      触发update：
    //          App将封装后的消息实例作为参数，调用update方法
    */
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
                        // ZIHAO: reuse the reqwest::Clietn, it uses an Arc internally
                        let client = reqwest::ClientBuilder::new()
                            .cookie_store(true)
                            .build()
                            .unwrap()
                            ;
                        self.client = client.clone();
                        // ZIHAO: perform asynchorneous actions
                        Command::perform(async move {

                            let res = client
                                .post(format!("{}/user/login", BASE_URL))
                                .json(&this_user)
                                .send()
                                .await;
                            info!("{res:?}");
                            return match res {
                                Ok(response) => {
                                    let my_http_response = response.json::<MyHttpResponse>().await.unwrap();
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
                // ZIHAO: 通过改变P2PAppState::HomeState::current_child_page来控制子组件的展示
                match home_message {
                    home::HomeMessage::FileMessage(file_message) => {
                        info!("current home child component: file");
                        self.home_state.current_child_page = 0;
                        match file_message{
                            home::file::FileMessage::GetMetadata => {
                                let client = self.client.clone();
                                // ZIHAO: perform async action
                                Command::perform(
                                    async move {
                                        let response = client
                                            .get(format!("{}/user/show_metadata", BASE_URL))
                                            .send()
                                            .await
                                            ;
                                        match response{
                                            Ok(response) => {
                                                info!("{response:?}");
                                                if response.status() == reqwest::StatusCode::OK {
                                                    Ok(response.json::<Vec<METADATA>>().await.unwrap())
                                                } else {
                                                    let my_httresponse = response.json::<MyHttpResponse>().await.unwrap();
                                                    Err(MyError::ServerError {
                                                        code: my_httresponse.code,
                                                        message: my_httresponse.message
                                                    })
                                                }
                                            },
                                            Err(e) => {
                                                Err(MyError::ClientError {
                                                    code: 1,
                                                    message: e.to_string()
                                                })
                                            }
                                        }
                                    },
                                    // ZIHAO: 将子模块内部的消息类型，映射到主App的消息类型，即具体的variant的映射
                                    // map home::file::FileMessage::GetMetadataResponse
                                    // -> home::HomeMessage::FileMessage
                                    // -> P2PAppMessage::HomeMessage
                                    home::file::FileMessage::GetMetadataResponse
                                )
                                    .map(home::HomeMessage::FileMessage)
                                    .map(P2PAppMessage::HomeMessage)
                            },
                            home::file::FileMessage::GetMetadataResponse(response) => {
                                info!("response: {response:?}");
                                match response{
                                    Ok(metadata_vec) => {
                                        self.home_state.file_state.uploaded_metadata = metadata_vec;
                                        Command::none()
                                    },
                                    Err(my_error) => {
                                        error!("{my_error:?}");
                                        Command::none()
                                    }
                                }

                            },
                            home::file::FileMessage::DeleteMetadata(metadata_id) => {
                                info!("metadata_id: {metadata_id}");
                                let client = self.client.clone();
                                Command::perform(
                                    async move {
                                        let response = client.delete(format!("{}/user/delete_metadata?metadata={}", BASE_URL, metadata_id))
                                            .send()
                                            .await
                                            .unwrap();
                                        match response.json::<MyHttpResponse>().await {
                                            Ok(my_http_res) => {
                                                Ok(my_http_res)
                                            },
                                            Err(e) => {
                                                Err(MyError::ClientError {
                                                    code: 1,
                                                    message: e.to_string()
                                                })
                                            }
                                        }
                                    },
                                    file::FileMessage::DeleteMetadataResponse
                                ).map(home::HomeMessage::FileMessage)
                                    .map(P2PAppMessage::HomeMessage)
                            },
                            home::file::FileMessage::DeleteMetadataResponse(response) => {
                                match response{
                                    Ok(my_http_res) => {
                                        info!("{my_http_res:?}");
                                        Command::perform(
                                            async {
                                                file::FileMessage::GetMetadata
                                            },
                                            home::HomeMessage::FileMessage
                                        ).map(P2PAppMessage::HomeMessage)
                                    },
                                    Err(my_error) => {
                                        info!("{my_error:?}");
                                        Command::none()
                                    }
                                }
                            }
                        }
                    },
                    home::HomeMessage::UploadMessage(upload_message) => {
                        info!("current home child component: upload");
                        self.home_state.current_child_page = 1;
                        Command::none()
                    },
                    home::HomeMessage::SearchMessage(search_message) => {
                        info!("current home child component: search");
                        self.home_state.current_child_page = 2;
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
                        // ZIHAO: 第一个异步代码块中返回的结果，必须能被第二个参数Message变体包裹，以此创建一个变体实例
                        //      可以将该变体实例映射为主App的Message变体实例，须要主AppMessage变体包含该子Message类型
                        Command::perform(
                            async {
                                file::FileMessage::GetMetadata
                            },
                            home::HomeMessage::FileMessage
                        )
                            .map(P2PAppMessage::HomeMessage)
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
        // ZIHAO: 通过P2PAppState::current_page控制页面
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