use chrono::prelude::*;
use gloo::console::{self};
use gloo::timers::callback::Interval;
use std::{cell::RefCell, rc::Rc};

use patternfly_yew::*;
use yew::prelude::*;

fn format_timestamp(timestamp: i64) -> String {
    let now = Utc::now();
    let end_time = DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    let time_remaining = end_time - now;
    if time_remaining.num_seconds() <= 0 {
        return "Available Now".to_owned();
    }
    let days = (time_remaining.num_seconds() / 60) / 60 / 24;
    let hours = ((time_remaining.num_seconds() / 60) / 60) % 24;
    let mins = (time_remaining.num_seconds() / 60) % 60;
    let secs = time_remaining.num_seconds() % 60;
    format!(
        "{} Days {} Hours {} Minutes {} Seconds",
        days, hours, mins, secs,
    )
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub current_cost: f64,
    pub launch_time: i64,
    pub num: i64,
    pub quantity: i64,
    pub title: String,
    pub rarity: String,
}

#[function_component(NFTDetails)]
pub fn nft_details(props: &Props) -> Html {
    let time_left = use_state(|| format_timestamp(props.launch_time));
    let launch_time = props.launch_time.clone();
    let timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);
    let formatted_time = time_left.clone();
    use_effect(move || {
        let time_left = time_left.clone();
        *timer.borrow_mut() = Some(Interval::new(1000, move || {
            let time = format_timestamp(launch_time);
            time_left.set(time);
        }));
        || ()
    });

    html! {
        <Bullseye>
            <Stack gutter=true>
                <StackItem>
                    <Title level={Level::H1} size={Size::XXXXLarge}>
                        {format!(
                            "{} #{}",
                            props.title,
                            format!("{:0>3}", props.num),)}
                    </Title>
                </StackItem>
                <StackItem>
                    <Title level={Level::H2} size={Size::XXLarge}>
                        {format!(
                            "{} / {} Available / {} ETH",
                            props.rarity,
                            props.quantity,
                            props.current_cost,
                        )}
                    </Title>
                </StackItem>
                <StackItem>
                    <Title level={Level::H3} size={Size::XLarge}>
                        {"Available In:"}
                    </Title>
                </StackItem>
                <StackItem>
                    <Card>
                        <Text
                            component={TextVariant::H1}>
                            {&*formatted_time}
                        </Text>
                    </Card>
                </StackItem>
                <StackItem>
                    <Button disabled={true} label={"Coming Soon"} />
                </StackItem>
            </Stack>
        </Bullseye>
    }
}
