use leptos::*;
use crate::model::Todo;
#[cfg(feature = "ssr")]
use crate::repository::todo_repository;

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    todo_repository::get_all()
        .await
        .map_err(|error| {
            logging::error!("{:?}", error);
            ServerFnError::ServerError(error.to_string())
        })
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: String) -> Result<(), ServerFnError> {
    todo_repository::delete_todo(id.as_str())
        .await
        .map_err(|error| {
            logging::error!("{:?}", error);
            ServerFnError::ServerError(error.to_string())
        })
}

#[server(EditTodo, "/api")]
pub async fn edit_todo(todo: Todo) -> Result<(), ServerFnError> {
    todo_repository::update_todo(todo)
        .await
        .map_err(|error| {
            logging::error!("{:?}", error);
            ServerFnError::ServerError(error.to_string())
        })
}

#[server(InsertTodo, "/api")]
pub async fn insert_todo(todo: Todo) -> Result<(), ServerFnError> {
    todo_repository::insert_todo(todo)
        .await
        .map_err(|error| {
            logging::error!("{:?}", error);
            ServerFnError::ServerError(error.to_string())
        })
}