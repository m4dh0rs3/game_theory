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
            <div class="xy-game">
                <h1>{ "Game theory: XY" }</h1>

                <table id="table-rules">
                    <tr><th>{ "Set" }</th><th>{ "X" }</th><th>{ "Y" }</th></tr>

                    <tr><td>{ "XXXX" }</td><td>{ "-10" }</td><td>{ "–" }</td></tr>

                    <tr><td>{ "XXXY" }</td><td>{ "+10" }</td><td>{ "-30" }</td></tr>

                    <tr><td>{ "XXYY" }</td><td>{ "+20" }</td><td>{ "-20" }</td></tr>

                    <tr><td>{ "XYYY" }</td><td>{ "+30" }</td><td>{ "-10" }</td></tr>
                        
                    <tr><td>{ "YYYY" }</td><td>{ "–" }</td><td>{ "+10" }</td></tr>
                </table>

                <p>{ "Each round, all `n` actors choose `X` or `Y`. Rules on the set of the choice of all actors decide, how much each actor earns or looses." }</p>

                <button id="button-payout" onclick={ link.callback(|_| GameMsg::Payout) }>{ "Payout" }</button>

                <table id="table-game">
                    <tr>
                        <th>{ "Actor" }</th>

                        { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                            <td class="td-actor">{ format!("{}", std::char::from_u32(i as u32 + 65).unwrap()) }</td>
                        }).collect::<Html>() }
                    </tr>

                    <tr>
                        <th>{ "Total" }</th>

                        { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                            <td class="td-money">{ actor.money }</td>
                        }).collect::<Html>() }
                    </tr>

                    <tr>
                        <th>{ "Choice" }</th>

                        { self.bank.0.iter().enumerate().map(|(i, actor)| html! {
                            <td>        
                                <select
                                    ref={ self.refs[i].clone() }
                                    class="select-choice"
                                    onchange={ link.callback(move |_| GameMsg::Choose(i)) }
                                >
                                    <option value="X" selected=true>{ "X" }</option>
                                    <option value="Y">{ "Y" }</option>
                                </select>
                            </td>
                        }).collect::<Html>() }
                    </tr>

                    { self.history.iter().enumerate().map(|(round, bank)| html! {
                        <tr>
                            <th class="td-round">{ round + 1 }</th>

                            { bank.0.iter().enumerate().map(|(i, actor)| match actor.choice {
                                game::Choice::X => html! { <td class="td-x">{ "X" }</td> },
                                game::Choice::Y => html! { <td class="td-y">{ "Y" }</td> },
                            }).collect::<Html>() }
                        </tr>
                    }).rev().collect::<Html>() }
                </table>
            </div>
        }
    }
}