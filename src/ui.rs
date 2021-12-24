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
                self.bank.payout();
                
                self.history.push(self.bank.clone());
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="game">
                <h1>{ "Game theory: XY" }</h1>

                <button id="payout" onclick={ link.callback(|_| GameMsg::Payout) }>{ "Payout" }</button>

                <table>
                    <tr>
                        <th>{ "Actor" }</th>

                        { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                            <th>{ format!("{}", std::char::from_u32(i as u32 + 65).unwrap()) }</th>
                        }).collect::<Html>() }
                    </tr>

                    <tr>
                        <th>{ "Total" }</th>

                        { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                            <th>{ actor.money }</th>
                        }).collect::<Html>() }
                    </tr>

                    <tr>
                        <th>{ "Choice" }</th>

                        { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                            <th>        
                                <select
                                    name="Choice"
                                    ref={ self.refs[i].clone() }
                                    id={ format!("actor_{}", i) }
                                    onchange={ link.callback(move |_| GameMsg::Choose(i)) }
                                >
                                    <option value="X" selected=true>{ "X" }</option>
                                    <option value="Y">{ "Y" }</option>
                                </select>
                            </th>
                        }).collect::<Html>() }
                    </tr>

                    { self.history.iter().enumerate().map(|(round, bank)| html! {
                        <tr>
                            <th>{ round + 1 }</th>

                            { bank.0.iter().enumerate().map(|(i, actor)| html! {
                                <th>{ match actor.choice {
                                    game::Choice::X => "X",
                                    game::Choice::Y => "Y",
                                } }</th>
                            }).collect::<Html>() }
                        </tr>
                    }).collect::<Html>() }
                </table>
            </div>
        }
    }
}