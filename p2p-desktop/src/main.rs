mod assistance;
mod user;
mod components;

use std::collections::{HashSet, VecDeque};
use std::io;
use std::io::Read;
use std::thread::sleep;
use iced::{Alignment, Application, Command, Element, executor, Renderer, Settings};
use iced::keyboard::KeyCode::Comma;
use iced_widget::Column;
use serde::de::Error;
use serde::Serialize;
use log::{error, info};
use reqwest::Client;
use tokio::task::JoinHandle;
use crate::assistance::connection::{download_file, listen};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;
use crate::assistance::metadata::METADATA;
use crate::user::user::User;
use crate::components::{home, login, page, register};
use crate::components::home::{file, search, setting, upload};

const BASE_URL: &str = "http://localhost:8080";

fn main() {
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
    client: Client,
    // pop-up
    pop_up_content: String
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
    PageMessage(page::PageMessage),
    // Show Message
    ShowPopUpMessage(String),
    HidePopUpMessage
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
                        size_text: "".to_string(),
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
                    setting_state: home::setting::SettingState{
                        ip: "".to_string(),
                        port: "".to_string(),
                    }
                },
                current_page: page::PageState::LoginPage,
                client: Client::new(),
                pop_up_content: "".to_string()
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
                        if self.login_state.name != "" && self.login_state.password != "" {
                            let this_user = User{
                                name: (self.login_state.name).clone(),
                                password: (self.login_state.password).clone()
                            };

                            info!("{} {}", self.login_state.name, self.login_state.password);
                            // ZIHAO: reuse the reqwest::Clietn, it uses an Arc internally
                            let client = reqwest::ClientBuilder::new()
                                // ZIHAO: should set cookie_store true to set cookie automatically each additional request
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
                            }, login::LoginMessage::LoginResponse
                            )
                                .map(P2PAppMessage::LoginMessage)
                        } else {
                            Command::perform(
                                async {
                                    "empty item".to_string()
                                },
                                |message| P2PAppMessage::ShowPopUpMessage(message)
                            )
                        }
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
                                    Command::perform(
                                        async {},
                                        move |_| P2PAppMessage::ShowPopUpMessage(my_http_response.message)
                                    )
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
                        if self.register_state.name != "" && self.register_state.password != "" {
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
                        } else {
                            Command::perform(
                                async {},
                                |_| P2PAppMessage::ShowPopUpMessage("Empty item".to_string())
                            )
                        }

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
                                        self.home_state.file_state.uploaded_metadata = metadata_vec.clone();
                                        // ZIHAO: 对于iced框架，执行异步代码块 只能在update方法的Command::perform第一个参数中，
                                        //      并在其中使用tokio运行异步的协程；
                                        // ZIHAO: 第二个参数实际上是一个移动捕获的闭包，将第一个参数返回的Future(T)的T作为闭包参数，
                                        //      该闭包的返回值必须为一个Message实例，用来更新App的状态，可以通过map向主app的Message映射
                                        Command::perform(
                                            async move{
                                                let mut ip_port_set: HashSet<(String, String)> = HashSet::new();
                                                for metadata in metadata_vec{
                                                    if !ip_port_set.contains(&(metadata.ip.clone(), metadata.port.clone())){
                                                        tokio::spawn(listen(metadata.ip.clone(), metadata.port.clone()));
                                                        ip_port_set.insert((metadata.ip, metadata.port));
                                                    }
                                                }
                                            },
                                            |_| P2PAppMessage::ShowPopUpMessage("get metadata success".to_string())
                                        )
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
                        let metadata_to_upload = &mut self.home_state.upload_state.metadata_to_upload;
                        metadata_to_upload.user_name = self.login_state.name.clone();
                        match upload_message{
                            upload::UploadMessage::GetUploadedMetadata => {
                                info!("current home child component: upload");
                                self.home_state.current_child_page = 1;
                                Command::none()
                            },
                            upload::UploadMessage::NAME(name) => {
                                metadata_to_upload.name = name;
                                Command::none()
                            },
                            // upload::UploadMessage::Ip(ip) => {
                            //     metadata_to_upload.ip = ip;
                            //     Command::none()
                            // },
                            upload::UploadMessage::Path(path) => {
                                metadata_to_upload.path = path;
                                Command::none()

                            },
                            // upload::UploadMessage::Port(port) => {
                            //     metadata_to_upload.port = port;
                            //     Command::none()
                            //
                            // },
                            upload::UploadMessage::Size(size) => {
                                self.home_state.upload_state.size_text = size;
                                Command::none()
                            },
                            upload::UploadMessage::SubmitUpload => {
                                match self.home_state.upload_state.size_text.parse::<f64>(){
                                    Ok(size) => {
                                        metadata_to_upload.size = size;
                                        info!("{metadata_to_upload:?}");
                                        if metadata_to_upload.name == "" || metadata_to_upload.path == ""|| metadata_to_upload.size==0.0 {
                                            error!("empty item");
                                            Command::perform(
                                                async move {
                                                    "empty items".to_string()
                                                },
                                                P2PAppMessage::ShowPopUpMessage
                                            )
                                        } else if ( metadata_to_upload.port=="" || metadata_to_upload.ip=="" ) {
                                            Command::perform(
                                                async move {
                                                    "please set ip + port firstly".to_string()
                                                },
                                                P2PAppMessage::ShowPopUpMessage
                                            )
                                        } else {
                                            let client = self.client.clone();
                                            let metadata_to_upload = metadata_to_upload.clone();
                                            Command::perform(
                                                async move{
                                                    let response = client
                                                        .post(format!("{}/metadata/upload", BASE_URL))
                                                        .json(&metadata_to_upload)
                                                        .send()
                                                        .await;
                                                    match response{
                                                        Ok(response) => {
                                                            match response.json::<MyHttpResponse>().await {
                                                                Ok(my_http_response) => {
                                                                    Ok(my_http_response)
                                                                },
                                                                Err(e) => {
                                                                    Err(MyError::ClientError {
                                                                        code:1,
                                                                        message: e.to_string()
                                                                    })
                                                                }
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
                                                upload::UploadMessage::UploadResponse
                                            )
                                                .map(home::HomeMessage::UploadMessage)
                                                .map(P2PAppMessage::HomeMessage)
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e.to_string());
                                        Command::perform(
                                            async move {
                                                e.to_string()
                                            },
                                            P2PAppMessage::ShowPopUpMessage
                                        )
                                    }
                                }
                            },
                            upload::UploadMessage::UploadResponse(response) => {
                                info!("{response:?}");
                                match response{
                                    Ok(my_httpresponse) => {
                                        if my_httpresponse.code == 0 {
                                            Command::perform(
                                                async {},
                                                |_| P2PAppMessage::ShowPopUpMessage("success".to_string())
                                            )
                                        } else {
                                            Command::perform(
                                                async {},
                                                move |_| P2PAppMessage::ShowPopUpMessage(my_httpresponse.message)
                                            )
                                        }
                                    },
                                    Err(e) => {
                                        Command::perform(
                                            async {},
                                            move |_| P2PAppMessage::ShowPopUpMessage(e.get_message())
                                        )
                                    }
                                }
                            }
                        }
                    },
                    home::HomeMessage::SearchMessage(search_message) => {
                        let self_search_state = &mut self.home_state.search_state;
                        match search_message{
                            search::SearchMessage::RouteHere => {
                                info!("current home child component: search");
                                self.home_state.current_child_page = 2;
                                Command::none()
                            },
                            search::SearchMessage::EnterFileName(file_name_to_search) => {
                                self_search_state.file_name_to_search = file_name_to_search;
                                Command::none()
                            },
                            search::SearchMessage::SubmitSearch => {
                                let client = self.client.clone();
                                let url = format!("{}/metadata/lookup?file_name={}", BASE_URL, self_search_state.file_name_to_search);
                                Command::perform(
                                    async move {
                                        match client.get(url).send().await{
                                            Ok(response) => {
                                                match response.status() {
                                                    reqwest::StatusCode::OK => {
                                                        match response.json::<Vec<METADATA>>().await{
                                                            Ok(metadata_vec) => {
                                                                let mut tasks: Vec<JoinHandle<bool>> = vec![];
                                                                for metadata in metadata_vec.clone() {
                                                                    tasks.push(tokio::spawn(search::test_online(metadata.ip, metadata.port)));
                                                                }
                                                                let mut states:VecDeque<bool> = VecDeque::new();
                                                                for task in tasks {
                                                                    states.push_back(task.await.unwrap());
                                                                }

                                                                let mut vec: Vec<(METADATA, bool)> = vec![];
                                                                for metadata in metadata_vec.clone() {
                                                                    vec.push((metadata, states.pop_front().unwrap()))
                                                                }

                                                                Ok(vec)
                                                            },
                                                            Err(e) => {
                                                                Err(MyError::ClientError {
                                                                    code: 1,
                                                                    message: e.to_string()
                                                                })
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        match response.json::<MyHttpResponse>().await{
                                                            Ok(my_httpresponse) => {
                                                                Err(MyError::ServerError {
                                                                    code: my_httpresponse.code,
                                                                    message: my_httpresponse.message
                                                                })
                                                            },
                                                            Err(e) => {
                                                                Err(MyError::ClientError {
                                                                    code: 1,
                                                                    message: e.to_string()
                                                                })
                                                            }
                                                        }
                                                    }

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
                                    search::SearchMessage::SearchResponse
                                )
                                    .map(home::HomeMessage::SearchMessage)
                                    .map(P2PAppMessage::HomeMessage)
                            },
                            search::SearchMessage::SearchResponse(response) => {
                                info!("response: {response:?}");
                                match response{
                                    Ok(metadata_vec) => {
                                        self_search_state.searched_metadata = metadata_vec;
                                        Command::perform(
                                            async {
                                                "success".to_string()
                                            },
                                            P2PAppMessage::ShowPopUpMessage
                                        )
                                    },
                                    Err(my_error) => {
                                        Command::perform(
                                            async move {
                                                my_error.get_message()
                                            },
                                            P2PAppMessage::ShowPopUpMessage
                                        )
                                    }
                                }
                            },
                            search::SearchMessage::Download(metadata) => {
                                info!("{metadata:?}");
                                let user_name = self.login_state.name.clone();
                                Command::perform(
                                    async move {
                                        // ZIHAO: async标记的函数或代码块返回Future，.await会在Future返回指定类型值之前一直阻塞该协程，
                                        download_file(user_name, metadata).await
                                    },
                                    search::SearchMessage::DownloadResponse
                                )
                                    .map(home::HomeMessage::SearchMessage)
                                    .map(P2PAppMessage::HomeMessage)
                            },
                            search::SearchMessage::DownloadResponse(result) => {
                                match result{
                                    Ok(file_name) => {
                                        info!("Downloading {file_name}");
                                        Command::perform(
                                            async {
                                                "test".to_string()
                                            },
                                            |message| P2PAppMessage::ShowPopUpMessage(message)
                                        )
                                    },
                                    Err(my_error) => {
                                        info!("{}", my_error.get_message());
                                        Command::perform(
                                            async {},
                                            move |_| P2PAppMessage::ShowPopUpMessage(my_error.get_message())
                                        )
                                    }
                                }
                            }
                        }

                    },
                    home::HomeMessage::SettingMessage(setting_message) => {
                        let setting = &mut self.home_state.setting_state;
                        match setting_message {
                            setting::SettingMessage::RouteHere => {
                                info!("current home child component: setting");
                                self.home_state.current_child_page = 3;
                                Command::none()
                            },
                            setting::SettingMessage::SetIp(set_ip) => {
                                info!("{set_ip}");
                                setting.ip = set_ip;
                                Command::none()
                            },
                            setting::SettingMessage::SetPort(set_port) => {
                                info!("{set_port}");
                                setting.port = set_port;
                                Command::none()
                            },
                            setting::SettingMessage::SetSocket => {
                                self.home_state.upload_state.metadata_to_upload.ip = self.home_state.setting_state.ip.clone();
                                self.home_state.upload_state.metadata_to_upload.port = self.home_state.setting_state.port.clone();
                                Command::none()
                            },
                            setting::SettingMessage::Test => {
                                info!("test: {setting:?}");
                                let client = self.client.clone();
                                let url = format!("{}/user/test_socket?ip={}&port={}", BASE_URL, setting.ip, setting.port);
                                let ip = setting.ip.clone();
                                let port = setting.port.clone();
                                Command::perform(
                                    async move {
                                        // ZIHAO: 必须创建新线程或者tokio协程来后台运行对指定socket的监听
                                        tokio::spawn(listen(ip, port));
                                        // ZIHAO: 测试：让服务器与被监听的socket建立tcp连接，并发送确认信息
                                        match client.get(url).send().await {
                                            Ok(response) => {
                                                match response.json::<MyHttpResponse>().await{
                                                    Ok(response) => {
                                                        Ok(response)
                                                    },
                                                    Err(e) => {
                                                        Err(
                                                            MyError::ClientError {
                                                                code: 1,
                                                                message: e.to_string()
                                                            }
                                                        )
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                Err(
                                                    MyError::ClientError {
                                                        code: 1,
                                                        message: e.to_string()
                                                    }
                                                )
                                            }
                                        }
                                    },
                                    setting::SettingMessage::TestResponse
                                )
                                    .map(home::HomeMessage::SettingMessage)
                                    .map(P2PAppMessage::HomeMessage)
                            },
                            setting::SettingMessage::TestResponse(response) => {
                                info!("{response:?}");
                                match response {
                                    Ok(my_http_response) => {
                                        if my_http_response.code == 0 {
                                            Command::perform(
                                                async move {
                                                    "success".to_string()
                                                },
                                                P2PAppMessage::ShowPopUpMessage
                                            )
                                        } else {
                                            Command::perform(
                                                async move {
                                                    "test fail: server build tcp connection with client".to_string()
                                                },
                                                P2PAppMessage::ShowPopUpMessage
                                            )
                                        }
                                    },
                                    Err(e) => {
                                        Command::perform(
                                            async move {
                                                "test fail: client socket listening error".to_string()
                                            },
                                            P2PAppMessage::ShowPopUpMessage
                                        )
                                    }
                                }

                            }
                        }
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
            },
            // Pop-Up Message
            P2PAppMessage::ShowPopUpMessage(message) => {
                self.pop_up_content = message;
                // ZIHAO: 如果第二个参数的Message不包裹内容，此时第一个参数Future包裹的类型为()，则需要转为一个参数为空的闭包
                Command::perform(
                    async {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    },
                    |_| P2PAppMessage::HidePopUpMessage
                )
            },
            P2PAppMessage::HidePopUpMessage => {
                self.pop_up_content = "".to_string();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        // ZIHAO: 通过P2PAppState::current_page控制页面
        let page = match self.current_page{
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
        };

        // ZIHAO: 实现临时弹窗
        let pop_up:Element<P2PAppMessage> = if self.pop_up_content != "" {
                iced_widget::Container::new(
                    Column::new()
                        .align_items(iced::Alignment::Center)
                        .push(
                            iced_widget::Text::new(&self.pop_up_content)
                                .size(50)
                        )
                )
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .center_x()
                    .center_y()
                    .into()

        }else{
            Column::new()
                .push(iced_widget::Text::new(""))
                .into()
        };


        Column::new()
            .align_items(Alignment::Center)
            .push(page)
            .push(pop_up)
            .into()
    }
}