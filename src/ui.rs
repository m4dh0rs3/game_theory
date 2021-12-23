use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::game;

pub struct Game {
    bank: game::Bank,
    refs: [NodeRef; 4],
    history: Vec<game::Bank>,
}

pub enum GameMsg {
    Choose(usize),
    Payout,
}

impl Component for Game {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            bank: game::Bank::default(),
            refs: [NodeRef::default(), NodeRef::default(), NodeRef::default(), NodeRef::default()],
            history: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::Choose(i) => {
                let select = self.refs[i].cast::<HtmlInputElement>().unwrap();
                self.bank.0[i].choice = match select.value().as_str() {
                    "X" => game::Choice::X,
                    "Y" => game::Choice::Y,
                    _ => unreachable!()
                };
            },
            GameMsg::Payout => {
                self.history.push(self.bank.clone());

                self.bank.payout();
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="game">
                <h1>{ "Game theory: XY" }</h1>
                { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                    <div class="actor">
                        <p>{ format!("Budget of {}: {}", std::char::from_u32(i as u32 + 65).unwrap(), actor.money) }</p>
                        <select
                            name="Choice"
                            ref={ self.refs[i].clone() }
                            id={ format!("actor_{}", i) }
                            onchange={ link.callback(move |_| GameMsg::Choose(i)) }
                        >
                            <option value="X" selected=true>{ "X" }</option>
                            <option value="Y">{ "Y" }</option>
                        </select>
                        <label for={ format!("actor_{}", i) }>{ "Choice" }</label>
                    </div>
                }).collect::<Html>() }
                <button onclick={ link.callback(|_| GameMsg::Payout) }>{ "Payout" }</button>
            </div>
        }
    }
}