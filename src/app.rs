use patternfly_yew::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::Render;

use crate::components::*;
use crate::index::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/"]
    Index,
}

pub struct Model {}

impl yew::Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <Router<AppRoute, ()>
                    redirect={Router::redirect(|_|AppRoute::Index)}
                    render={Self::switch_main()}
                />
            </>
        }
    }
}

impl Model {
    fn switch_main() -> Render<AppRoute, ()> {
        Router::render(|switch: AppRoute| match switch {
            AppRoute::Index => Self::page(html! {<Index/>}),
        })
    }

    fn page(_html: Html) -> Html {
        let logo = html_nested! {
            <Logo
                src="https://via.placeholder.com/250x75?text=Overengineered"
                alt="Overengineered"
            />
        };

        let about_toggle = html! { <DropdownToggle text="Info"/> };
        let collections_toggle = html! { <DropdownToggle text="Collections"/> };
        let toolbar = Children::new(vec![
            html! {
                <Dropdown position={Position::Left} toggle={about_toggle} plain=true>
                    <DropdownItem>{"Info & Roadmap"}</DropdownItem>
                    <DropdownItem>{"Perks"}</DropdownItem>
                    <DropdownItem>{"Staff"}</DropdownItem>
                </Dropdown>
            },
            html! {
                <Dropdown position={Position::Left} toggle={collections_toggle} plain=true>
                    <DropdownItem>{"First Collection"}</DropdownItem>
                </Dropdown>
            },
            html! {
                <Flex>
                    <FlexItem><Text component={TextVariant::H4}>{"Store"}</Text></FlexItem>
                    <FlexItem><Text component={TextVariant::H4}>{"Github"}</Text></FlexItem>
                    <FlexItem><Text component={TextVariant::H4}>{"Discord"}</Text></FlexItem>
                </Flex>
            },
        ]);

        html! {
            <Page
                tools={toolbar}
                logo={logo}
                >
                    <Stack>
                        <StackItem fill={true}>
                            <NFTDetails
                                current_cost={0.22}
                                launch_time={1646844302}
                                num={1}
                                quantity={1}
                                title={"Jerry"}
                                rarity={"Principle Engineer"}
                            />
                        </StackItem>
                        <StackItem>
                            <Bullseye>
                                <Title
                                    level={Level::H1}
                                    size={Size::XXXXLarge}>
                                    {"Week 1 NFTs"}
                                </Title>
                            </Bullseye>
                        </StackItem>
                        <StackItem fill={true}>
                            <NFTDetails
                                current_cost={0.22}
                                launch_time={1646890610}
                                num={2}
                                quantity={10}
                                title={"Tom"}
                                rarity={"Jr Engineer"}
                            />
                        </StackItem>
                    </Stack>
            </Page>
        }
    }
}
