use tui::{
    widgets::{
        Paragraph,
        Block,
        Borders,
        Wrap
    },
    text::Text,
    layout::Alignment,
    style::{ Style, Color }
};

pub fn banner() -> Paragraph<'static> {
    let banner =
    ".-')      ('-.             _  .-')     ('-.   .-') _                     ('-. .-.   ('-.     .-') _       \n".to_owned()+
    "( OO ).  _(  OO)           ( \\( -O )  _(  OO) (  OO) )                   ( OO )  /  ( OO ).-.(  OO) )    \n"+
    "(_)---\\_)(,------.   .-----. ,------. (,------./     '._          .-----. ,--. ,--.  / . --. //     '._  \n"+
    "/    _ |  |  .---'  '  .--./ |   /`. ' |  .---'|'--...__)        '  .--./ |  | |  |  | \\-.  \\ |'--...__)\n"+
    "\\  :` `.  |  |      |  |('-. |  /  | | |  |    '--.  .--'        |  |('-. |   .|  |.-'-'  |  |'--.  .--' \n"+
    "'..`''.)(|  '--.  /_) |OO  )|  |_.' |(|  '--.    |  |          /_) |OO  )|       | \\| |_.'  |   |  |     \n"+
    ".-._)   \\ |  .--'  ||  |`-'| |  .  '.' |  .--'    |  |          ||  |`-'| |  .-.  |  |  .-.  |   |  |    \n"+
    "\\       / |  `---.(_'  '--'\\ |  |\\  \\  |  `---.   |  |         (_'  '--'\\ |  | |  |  |  | |  |   |  |\n"+
    "`-----'  `------'   `-----' `--' '--' `------'   `--'            `-----' `--' `--'  `--' `--'   `--'      \n";

    let style = Style::default()
        .fg(Color::LightGreen);
    let txt = Text::styled(banner, style);
    Paragraph::new(
        txt
    )
        .block(
            Block::default()
                .borders(Borders::NONE)
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
