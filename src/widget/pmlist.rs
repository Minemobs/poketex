use tui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
};

use crate::pokemon::{DictType, Pokemon};

use super::dex::{PokemonDex, PokemonDexState};

pub struct PokemonListStatus {
    pub state: ListState,
    pub items: Vec<Pokemon>,
    pub current: Pokemon,
    pub dex: PokemonDexState,
}

fn flat_dex(pm: &Pokemon) -> PokemonDexState {
    let name = pm.name.get_name();
    let mut list = vec![PokemonDex {
        name: name.clone(),
        iv: pm.iv,
        pm_type: pm.get_type(),
    }];

    match &pm.form {
        None => (),
        Some(form) => {
            for f in form {
                let name = format!("{} {}", name, f.form.join(" "));
                list.push(PokemonDex {
                    name,
                    iv: f.iv,
                    pm_type: f.get_type(),
                });
            }
        }
    }

    let state = PokemonDexState {
        items: list,
        page: 1,
    };
    state
}

impl PokemonListStatus {
    pub fn new(mut items: Vec<Pokemon>) -> PokemonListStatus {
        // make sure items has def pokemon
        if items.len() == 0 {
            items.push(Pokemon::default());
        };

        // init position = 0
        let mut state = ListState::default();
        state.select(Some(0));

        let current = (&items).get(0).unwrap().clone();

        PokemonListStatus {
            state,
            dex: flat_dex(&current),
            current,
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.current(i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.current(i);
    }

    fn current(&mut self, index: usize) {
        self.current = match self.items.get(index) {
            Some(pm) => pm.clone(),
            None => self.items.get(0).unwrap().clone(),
        };
        self.dex = flat_dex(&self.current);
    }
}

pub struct PokemonList;

impl Default for PokemonList {
    fn default() -> Self {
        PokemonList {}
    }
}

impl StatefulWidget for PokemonList {
    type State = PokemonListStatus;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(2)
            .split(area);

        let items: Vec<ListItem> = state
            .items
            .iter()
            .map(|item| {
                let title = "#".to_string()
                    + item.no.to_string().as_str()
                    + " "
                    + item.name.get_name().as_str();

                ListItem::new(vec![Spans::from(title)])
            })
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::LEFT)
                    .title_alignment(Alignment::Center)
                    .title("Pokemon List"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .render(layout[0], buf, &mut state.state);
    }
}