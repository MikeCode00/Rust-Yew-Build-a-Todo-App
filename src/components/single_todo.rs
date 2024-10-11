use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

use crate::{SingleTodoProps, TodoList};
#[function_component(SingleTodo)]
pub fn single_todo(props: &SingleTodoProps)-> Html {
  let todo_list_state_op = use_context::<UseStateHandle<TodoList>>();
  let todo_list_state = match todo_list_state_op {
      Some(todo_list_state) => todo_list_state,
      None => panic!("Something went wrong!")
  };
  let todo_item_op = (*todo_list_state).clone().list.into_iter().find(|todo| todo.id == props.id);
  let todo_item_view = match todo_item_op {
      Some(todo_item) => html!(
        <div class="card">
          <div class="card-header">
            {todo_item.created_at}
          </div>
          <div class="card-body">
            <blockquote class="blockquote mb-0">
              <p>{todo_item.content}</p>
              <footer class="blockquote-footer"><cite title="Source Title">{todo_item.completed}</cite></footer>
            </blockquote>
          </div>
        </div>
      ),
      None => html!(
        <h1> {"Todo id : "} {props.id} {" not found!"}</h1>
      )
  };
  html!(
    <>
      <div class="container full-height"> 
        <div class="row justify-content-center align-items-center h-100">
          <div class="col-8">
            <Link<Route> to={Route::Todos} > {"Go back to Todo List"} </Link<Route>>
            {todo_item_view}
          </div>
        </div>
      </div>
    </>
  )
}