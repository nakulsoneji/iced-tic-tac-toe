use iced::widget::{button, Column, Button, Text, Row, Container};
use iced::{Alignment, Element, Sandbox, Settings, theme, Theme, Length};
use iced::alignment::{Horizontal, Vertical};

const BASE_SIZE: u16 = 6;
pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

struct ButtonColor {
    color: iced::Color,
    border: iced::Color
}

impl ButtonColor {
    fn new(color: iced::Color, border: iced::Color) -> Self {
        ButtonColor {
            color,
            border
        }
    }
}

impl button::StyleSheet for ButtonColor {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.color)),
            border_width: 10.0,
            border_color: self.border,
            ..Default::default()
        }
    }
}

struct Game {
    board: [[char; 3]; 3],
    turn: char,
}


#[derive(Clone, Debug)]
enum Message {
    Move(usize, usize)
}

impl Game {
    fn check_win(&self) -> Option<(char, [[usize; 2]; 3])> {
        let diagonals = [
            [[0, 0], [1, 1], [2, 2]], 
            [[0, 2], [1, 1], [2, 0]]
        ];

        let board = self.board;
        for (index, row) in self.board.iter().enumerate() {
            // checking rows
            if row.iter().all(|x| *x == 'X') || row.iter().all(|x| *x == 'O') {
                return Some((row[0], [[index, 0], [index, 1], [index, 2]]));
            }
            
            // checking columns
            let column = [board[0][index], board[1][index], board[2][index]];
            if column.iter().all(|x| *x == 'X') || column.iter().all(|x| *x == 'O') {
                return Some((column[0], [[0, index], [1, index], [2, index]]));
            }
        }
        for diagonal in diagonals {
            let values = diagonal.iter().map(|&[a, b]| board[a][b]).collect::<Vec<char>>();
            if values.iter().all(|x| *x == 'X') || values.iter().all(|x| *x == 'X') {
                return Some((values[0], diagonal));
            }
        }
        None
    }

    fn is_winner(&self) -> bool {
        if let Some(_) = self.check_win() {
            return true;
        }
        false
    }
}

impl Sandbox for Game {
    type Message = Message;

    fn new() -> Self {
        Game {
            board:[[' '; 3]; 3],
            turn: 'X',
        }
    }

    fn title(&self) -> String {
        String::from("Iced Tic Tac Toe")
    }

    fn update(&mut self, message: Message) {
        let Message::Move(a, b) = message;
        if self.board[a][b] == ' ' {
            self.board[a][b] = self.turn;    
            self.turn = if self.turn == 'X' { 'O' } else { 'X' };
        }
    }

    fn view(&self) -> Element<Message> {
        let mut display = Column::new().align_items(Alignment::Center);
        
        for i in 0..self.board.len() {
            let mut row = Row::new().align_items(Alignment::Center);
            for j in 0..self.board[i].len() {
                row = row.push(make_button([i, j], self));
            }
            display = display.push(row);
        }

        Container::new(display).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }

}

fn make_button<'a>(index: [usize; 2], game: &Game) -> Button<'a, Message>{
    let size = BASE_SIZE * 16;
    let color;
    let (_, indices) = game.check_win().unwrap_or((Default::default(), Default::default()));

    if game.is_winner() && indices.iter().any(|&x| x == index) {
        color = ButtonColor::new(iced::Color::from_rgb(255.0, 255.0, 0.0), Default::default());
    }

    else {
        match game.board[index[0]][index[1]] {
            'X' => {
                color = ButtonColor::new(iced::Color::from_rgb(255.0, 0.0, 0.0), Default::default())
            }
            'O' => {
                color = ButtonColor::new(iced::Color::from_rgb(0.0, 0.0, 255.0), Default::default());
            }
            _ => {
                color = ButtonColor::new(iced::Color::from_rgb(0.8, 0.8, 0.8), Default::default());
            }
        }
    }

    let text = Text::new(game.board[index[0]][index[1]].to_string())
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(size);

    Button::new(text)
        .on_press(Message::Move(index[0], index[1]))
        .height(200)
        .width(200)
        .style(theme::Button::Custom(Box::new(color)))
}