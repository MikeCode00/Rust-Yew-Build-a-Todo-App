use yew::prelude::*;
use crate::{components::list::List, util::get_current_time, TodoItem, TodoList};


#[function_component(Todos)]
pub fn todos()-> Html {
  let todo_list_state_op = use_context::<UseStateHandle<TodoList>>();
  let todo_list_state = match todo_list_state_op {
      Some(todo_list_state) => todo_list_state,
      None => panic!("Something went wrong!")
  };

  let input_state = use_state(|| String::from(""));
  let input_callback = {
    let input_state = input_state.clone();
    Callback::from(move|e: InputEvent| {
      if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
        input_state.set(input.value());
      }
    })
  };

  let add_new_todo_callback = {
    let input_state = input_state.clone();
    let todo_list_state = todo_list_state.clone();
    Callback::from(move |_| {
      let mut current_todo_list = (*todo_list_state).clone().list;
      let new_todo_item = TodoItem {
        id: if current_todo_list.len() == 0 {0} else { current_todo_list[current_todo_list.len() - 1].id + 1},
        content: (*input_state).clone(),
        completed: false,
        created_at: get_current_time(),
      };
      current_todo_list.push(new_todo_item);
      todo_list_state.set(TodoList { list: current_todo_list });
      input_state.set(String::from(""));
    })
  };

  html!(
    <>
      <div class="container">
        <div class = "row justify-content-center " >
          <div class= "col-8 mt-5">
            <div class="mb-3">
              <textarea class = "form-control" id="exampleFormControlTextArea1" row="3" value={(*input_state).clone()} oninput = {input_callback} />
              <button type="button" class="btn btn-primary" onclick={add_new_todo_callback} disabled = {(*input_state).clone().trim() == ""}>{"add"}</button>
            </div>
            <ul class="list-group">
              <List />
            </ul>
          </div>
        </div>
      </div>
    </>
  )
}