use yew::{html, use_context, Callback, UseStateHandle};
use yew::{function_component, Html};
use yew_router::prelude::*;
use crate::Route;
use crate::TodoItem;
use crate::TodoList;

#[function_component(List)]
pub fn list()->Html {
  let todo_list_state_option = use_context::<UseStateHandle<TodoList>>();

  let todo_list_state = match todo_list_state_option {
      Some(todo_list_state) => todo_list_state,
      None => panic!("Something went wrong!"),
  };

  let delete_todo = {
    let todo_list_state = todo_list_state.clone();
    Callback::from(move |id: usize| {
      let current_list = (*todo_list_state).clone().list;
      let new_list = current_list.into_iter().filter(|todo| todo.id != id).collect::<Vec<TodoItem>>();
      todo_list_state.set(TodoList { list: new_list });
    })
  };

  let toggle_complete = {
    let todo_list_state = todo_list_state.clone();
    Callback::from(move |id: usize| {
      let current_list = (*todo_list_state).clone().list;
      let new_list = current_list.into_iter().map(|todo| {
        if todo.id == id {
          TodoItem{
            id: todo.id,
            content: todo.content,
            created_at: todo.created_at,
            completed: !todo.completed
          }
        } else {
          todo
        }
      }).collect::<Vec<TodoItem>>();
      todo_list_state.set(TodoList { list: new_list });
    })
  };

  let list = if (*todo_list_state).list.len() == 0 {
    html!(
      <li class="list-group-item" > {"No todo"} </li>
    )
  } else {
    (*todo_list_state).clone().list.into_iter().map(|todo| {
      let delete_todo = delete_todo.clone();
      let toggle_complete = toggle_complete.clone();
      html!(
        <li class="list-group-item d-flex justify-content-between align-items-center" >
        <Link<Route> to={Route::SingleTodo {id: todo.id}} classes ={if todo.completed {"completed"} else {""}}> {todo.content} </Link<Route>>
        <div>
        {
          if todo.completed {
            html!(
              <button type="button" class="btn btn-success" onclick = {move |_| toggle_complete.emit(todo.id)} >{"Complete"} </button>
            )
          }else {
            html! (
              <button type="button" class="btn btn-warning" onclick = {move |_| toggle_complete.emit(todo.id)} >{"Uncomplete"}</button>
            )
          }
        }
        <button type="button" class="btn btn-danger" onclick = {move |_| delete_todo.emit(todo.id)}> {"remove"}</button>
        </div>
        </li>
      )
    }).collect::<Html>()
  };
  html!(
    {
      list
    }
  )
}