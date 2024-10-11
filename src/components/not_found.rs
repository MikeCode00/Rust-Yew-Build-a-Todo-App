use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
#[function_component(NotFound)]
pub fn not_found()-> Html {
  html!(
    <>
      <div class="container full-height">
        <div class = "row justify-content-center align-items-center h-100" >
          <div class= "col-4"> 
            <h1>{"404 | Not Found"}</h1>
            <Link<Route> to={Route::Home}>{"Go Home"} </Link<Route>>
          </div>
        </div>
      </div>
    </>
  )
}