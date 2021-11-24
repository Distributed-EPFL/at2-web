use std::{borrow::Cow, mem};

use at2_ns::ThinUser;
use material_yew::{
    dialog::{ActionType, MatDialogAction},
    MatButton, MatDialog, MatFormfield, MatList, MatListItem, WeakComponentLink,
};
use yew::{prelude::*, worker::Agent};

const DEFAULT_SEND_TRANSACTION_AMOUNT: usize = 3;

use crate::agents;

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    /// User to show
    ///
    /// It's an Option to workaround a material-yew bug, so that the MatDialog can still be created
    /// even without any content
    pub user: Option<ThinUser>,
    /// Link to the MatDialog
    pub dialog_link: WeakComponentLink<MatDialog>,

    /// Where to send the transaction
    pub on_send: Callback<(ThinUser, usize)>,
}

pub struct SendTransactionDialog {
    link: ComponentLink<Self>,

    props: Properties,

    #[allow(dead_code)] // dropped on close
    get_balance_agent: Box<dyn Bridge<agents::GetBalance>>,
    current_user_balance: Option<u64>,

    amount_to_send: usize,
}

pub enum Message {
    GotBalance(<agents::GetBalance as Agent>::Output),
    UpdateAmountToSend(Option<usize>),

    SendTransaction,
    CancelDialog,
}

impl Component for SendTransactionDialog {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            props,
            get_balance_agent: agents::GetBalance::bridge(link.callback(Self::Message::GotBalance)),
            current_user_balance: None,
            amount_to_send: DEFAULT_SEND_TRANSACTION_AMOUNT,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::UpdateAmountToSend(parsed_amount) => {
                if let Some(amount) = parsed_amount {
                    self.amount_to_send = amount;
                }
                false
            }
            Self::Message::SendTransaction => {
                if let Some(recipient) = mem::take(&mut self.props.user) {
                    self.props.on_send.emit((
                        recipient,
                        mem::replace(&mut self.amount_to_send, DEFAULT_SEND_TRANSACTION_AMOUNT),
                    ));
                }
                true
            }
            Self::Message::CancelDialog => {
                self.props.user = None;
                true
            }
            Self::Message::GotBalance(ret) => match ret {
                Ok(balance) => {
                    self.current_user_balance = Some(balance);
                    true
                }
                Err(_) => {
                    if let Some(user) = self.props.user.clone() {
                        self.get_balance_agent.send(user);
                    }
                    false
                }
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            return false;
        }

        self.props = props;

        if let Some(user) = self.props.user.clone() {
            self.get_balance_agent.send(user);
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <MatDialog
                heading=self.props.user.as_ref().map(|user| Cow::from(user.name.clone()))
                dialog_link=self.props.dialog_link.clone()
                onclosed=self.link.callback(|action: String| match action.as_str() {
                    "send" => Self::Message::SendTransaction,
                    _ => Self::Message::CancelDialog,
                })
            >
                <MatList >
                    <MatListItem>
                        <MatFormfield
                            label="Balance:"
                            align_end=true
                        > { self.current_user_balance
                            .map(|balance| html! { format!("{}¤", balance) })
                            .unwrap_or(html! { <span style="color: lightgrey"> { "fetching" } </span> }) }
                        </MatFormfield>
                    </MatListItem>

                    <MatListItem>
                        <MatFormfield
                            label="Public key:"
                            align_end=true
                        > { self.props.user.as_ref().map(|user| user.public_key().to_string()).unwrap_or_else(|| "...".to_string()) } </MatFormfield>
                    </MatListItem>

                    <MatListItem>
                        <MatFormfield
                            label="Amount to send"
                            align_end=true
                        ><input
                            value=self.amount_to_send.to_string()
                            min=1
                            oninput=self.link.callback(|event: InputData|
                                Self::Message::UpdateAmountToSend(event.value.parse().ok())
                            )
                            type="number"
                        /></MatFormfield>
                    </MatListItem>
                </MatList>

                <MatDialogAction
                    action_type=ActionType::Primary
                    action=Cow::from("send")>
                    <MatButton label="Send" />
                </MatDialogAction>
                <MatDialogAction
                    action_type=ActionType::Secondary
                    action=Cow::from("cancel")>
                    <MatButton label="Cancel" />
                </MatDialogAction>
            </MatDialog>
        }
    }
}
