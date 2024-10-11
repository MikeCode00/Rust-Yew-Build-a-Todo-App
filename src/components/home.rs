use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
#[function_component(Home)]
pub fn home()-> Html {
  html!(
    <>
      <div class="container full-height">
        <div class = "row justify-content-center align-items-center h-100" >
          <div class= "col-8"> 
            <h1> {"Welcome to the world best "} <Link<Route> to={Route::Todos} > {"To-do"} </Link<Route>> {" app"} </h1>
          </div>
        </div>
      </div>
    </>
  )
}