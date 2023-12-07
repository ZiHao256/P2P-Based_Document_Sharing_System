use iced::{Alignment, Element, Length};
use iced::alignment::Vertical;
use iced_widget::{Column, Container, Row};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;
use crate::assistance::metadata::METADATA;
use crate::components::home::file;
use crate::P2PAppState;

pub struct FileState{
    pub uploaded_metadata: Vec<METADATA>
}

#[derive(Clone, Debug)]
pub enum FileMessage{
    // get
    GetMetadata,
    GetMetadataResponse(Result<Vec<METADATA>, MyError>),
    // delete
    DeleteMetadata(i64),
    DeleteMetadataResponse(Result<MyHttpResponse, MyError>)
}

pub fn file_view(app_state: &P2PAppState) -> Element<FileMessage> {

    let header = Row::new()
        .push(iced_widget::text("User Name").width(Length::Fill))
        .push(iced_widget::text("File Name").width(Length::Fill))
        .push(iced_widget::text("Size").width(Length::Fill))
        .push(iced_widget::text("Path").width(Length::Fill))
        .push(iced_widget::text("IP").width(Length::Fill))
        .push(iced_widget::text("Port").width(Length::Fill))
        .push(iced_widget::text("Operation").width(Length::Fill))
        .padding(10)
        ;

    /*
    ZIHAO：两种方式应对未知数量的行数
    */
    // ZIHAO: 第一种 由于iced中的Element都是不可Clone，为了由vec创建一个运行时未知行数的表，需要使用fold方法，
    let content: Column<FileMessage> = app_state.home_state.file_state.uploaded_metadata
        .iter()
        .fold(
            Column::new().spacing(10),
            |col, metadata|{
            col.push(
                Row::new()
                    .push(iced_widget::text(&metadata.user_name).width(Length::Fill))
                    .push(iced_widget::text(&metadata.name).width(Length::Fill))
                    .push(iced_widget::text(&metadata.size).width(Length::Fill))
                    .push(iced_widget::text(&metadata.path).width(Length::Fill))
                    .push(iced_widget::text(&metadata.ip).width(Length::Fill))
                    .push(iced_widget::text(&metadata.port).width(Length::Fill))
                    .push(
                        iced_widget::button("Delete")
                            // ZIHAO: 触发widget时，创建一个子组件Message的变体实例，
                            //  接着在view方法中回调时，映射为主App的Message变体实例
                            //  并调用update方法
                            .width(Length::Fill)
                            .on_press(file::FileMessage::DeleteMetadata(metadata.id))
                    )
            )
        });

    // ZIHAO：第二种 iced_widget::column函数参数为vec类型
    // let content: Vec<Element<FileMessage>> = app_state.home_state.file_state.uploaded_metadata
    //     .iter()
    //     .map(|metadata| {
    //         Row::new()
    //             .push(iced_widget::text(&metadata.user_name))
    //             .push(iced_widget::text(&metadata.name))
    //             .push(iced_widget::text(&metadata.size))
    //             .push(iced_widget::text(&metadata.path))
    //             .push(iced_widget::text(&metadata.ip))
    //             .push(iced_widget::text(&metadata.port))
    //             .push(
    //                 iced_widget::button("Delete")
    //                     // ZIHAO: 触发widget时，创建一个子组件Message的变体实例，
    //                     //  接着在view方法中回调时，映射为主App的Message变体实例
    //                     //  并调用update方法
    //                     .on_press(file::FileMessage::DeleteMetadata(metadata.id))
    //             ).into()
    //     }).collect();

    Container::new(
        iced_widget::Column::new()
            .push(header)
            .push(content)
    )
        .into()
}
