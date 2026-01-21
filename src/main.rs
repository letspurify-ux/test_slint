use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};

#[derive(Debug, Clone)]
struct QueryResult {
    column1: String,
    column2: String,
    column3: String,
}

#[derive(Default)]
struct OracleQueryApp {
    host: String,
    port: String,
    service: String,
    user: String,
    query: String,
    status: String,
    results: Vec<QueryResult>,
}

#[derive(Debug, Clone)]
enum Message {
    HostChanged(String),
    PortChanged(String),
    ServiceChanged(String),
    UserChanged(String),
    QueryChanged(String),
    RunQuery,
}

impl Application for OracleQueryApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            OracleQueryApp {
                host: "db.company.local".to_string(),
                port: "1521".to_string(),
                service: "ORCL".to_string(),
                user: "system".to_string(),
                query: "select * from employees where status = 'ACTIVE';".to_string(),
                status: "Ready".to_string(),
                results: vec![QueryResult {
                    column1: "EMP_ID".to_string(),
                    column2: "NAME".to_string(),
                    column3: "STATUS".to_string(),
                }],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Oracle Query Console".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::HostChanged(value) => self.host = value,
            Message::PortChanged(value) => self.port = value,
            Message::ServiceChanged(value) => self.service = value,
            Message::UserChanged(value) => self.user = value,
            Message::QueryChanged(value) => self.query = value,
            Message::RunQuery => {
                let mut mock_rows = Vec::new();
                for idx in 0..6 {
                    mock_rows.push(QueryResult {
                        column1: format!("EMP-{}", 1000 + idx),
                        column2: format!("User {:02}", idx + 1),
                        column3: "ACTIVE".to_string(),
                    });
                }
                self.results = mock_rows;
                self.status = format!("Ran query (mock): {}", self.query);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let connection_panel = column![
            text("Connection").size(20),
            row![
                column![text("Host"), text_input("Host", &self.host)
                    .on_input(Message::HostChanged)
                    .padding(6),]
                .spacing(6),
                column![text("Port"), text_input("Port", &self.port)
                    .on_input(Message::PortChanged)
                    .padding(6),]
                .spacing(6),
            ]
            .spacing(12),
            row![
                column![text("Service"), text_input("Service", &self.service)
                    .on_input(Message::ServiceChanged)
                    .padding(6),]
                .spacing(6),
                column![text("User"), text_input("User", &self.user)
                    .on_input(Message::UserChanged)
                    .padding(6),]
                .spacing(6),
            ]
            .spacing(12),
        ]
        .spacing(12);

        let query_panel = column![
            text("SQL Query").size(20),
            text_input("Enter SQL query", &self.query)
                .on_input(Message::QueryChanged)
                .padding(8),
            button("Run Query").on_press(Message::RunQuery),
            text(format!("Status: {}", self.status)).size(16),
        ]
        .spacing(12);

        let header_row = row![
            text("Column 1").width(Length::FillPortion(1)),
            text("Column 2").width(Length::FillPortion(1)),
            text("Column 3").width(Length::FillPortion(1)),
        ]
        .spacing(12);

        let mut rows = column![header_row].spacing(8);
        for row_item in &self.results {
            rows = rows.push(
                row![
                    text(&row_item.column1).width(Length::FillPortion(1)),
                    text(&row_item.column2).width(Length::FillPortion(1)),
                    text(&row_item.column3).width(Length::FillPortion(1)),
                ]
                .spacing(12),
            );
        }

        let results_panel = column![
            text("Results").size(20),
            scrollable(rows).height(Length::Fill),
        ]
        .spacing(12);

        let content = column![connection_panel, query_panel, results_panel]
            .spacing(24)
            .padding(20)
            .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    OracleQueryApp::run(Settings::default())
}
