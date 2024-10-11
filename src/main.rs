
use components::{home::Home, not_found::NotFound, single_todo::SingleTodo, todos::Todos};
use util::get_current_time;
use yew::{function_component, html, use_state, Html, Properties, ContextProvider, UseStateHandle };
use yew_router::{Routable, BrowserRouter, Switch};
mod components;
mod util;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let todo_list_state = use_state(|| {
        TodoList {
            list: vec![
                TodoItem {
                    id: 0,
                    content: String::from("Drink water"),
                    completed: false,
                    created_at: get_current_time(),
                },
                TodoItem {
                    id: 1,
                    content: String::from("Take a walk"),
                    completed: true,
                    created_at: get_current_time()
                }
            ]
        }
    });
    html!(
        <ContextProvider<UseStateHandle<TodoList>> context = {todo_list_state.clone()}>
            <BrowserRouter>
                <Switch<Route> render = {switch} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<TodoList>>>
    )
}

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/todos")]
    Todos,
    #[at("/todo/:id")]
    SingleTodo {id: usize},
    #[not_found]
    #[at("/*")]
    NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<Home />),
        Route::Todos => html!(<Todos />),
        Route::SingleTodo { id } => html!(<SingleTodo id={id}/>),
        Route::NotFound => html!(<NotFound />),
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct SingleTodoProps {
    id: usize
}

#[derive(PartialEq, Clone)]
pub struct TodoItem {
    id: usize,
    content: String,
    completed: bool,
    created_at: String
}

#[derive(PartialEq, Clone)]
pub struct TodoList {
    list: Vec<TodoItem>
}
