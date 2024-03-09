use leptos::*;
use crate::component::{EditTodoSignal, ShowTodoModalSignal};
use crate::model::Todo;
use crate::component::todo_modal::TodoModal;
use crate::component::todo_list_item::TodoListItem;
use crate::service::todo_service::{delete_todo, edit_todo, get_todos, insert_todo};

#[component]
pub fn MainPage() -> impl IntoView {
    let (todos_refresh, set_todos_refresh) = create_signal(0);

    let show_modal: ShowTodoModalSignal = create_rw_signal(false);
    let edit_todo_item: EditTodoSignal = create_rw_signal(None);

    let todos = create_resource(todos_refresh, |_| async move {
        get_todos().await.unwrap_or_else(|_| {
            //web_sys::console::log_1(&format!("ERROR: {}",error).into());
            vec![]
        })
    });

    let callback_add_todo_event = move |todo: Todo| {
        spawn_local(async move {
            let todos_db = insert_todo(todo).await;
            show_modal.set(false);
            if todos_db.is_ok() {
                set_todos_refresh.update(|x| *x += 1);
            }
        });
    };

    let callback_edit_todo_event = move |todo: Todo| {
        spawn_local(async move {
            let todos_db = edit_todo(todo).await;
            show_modal.set(false);
            if todos_db.is_ok() {
                set_todos_refresh.update(|x| *x += 1);
            }
        });
    };

    let callback_cancel_add_event = move |_| {
        show_modal.set(false);
    };

    let on_show_modal_add_event = move |_| {
        edit_todo_item.set(None);
        show_modal.set(true);
    };

    let on_delete_todo_event = move |todo : Todo| {
        spawn_local(async move {
            let todos_db = delete_todo(todo.id.clone()).await;
            if todos_db.is_ok() {
                set_todos_refresh.update(|x| *x += 1);
            }
        });
    };

    let on_edit_todo_event = move |todo : Todo| {
        edit_todo_item.set(Some(todo));
        show_modal.set(true);
    };

    view! {
        <div class="container mx-auto m-5 p-6">
        <h1 class="mb-4 text-4xl font-extrabold text-center text-gray-600">TODO LIST</h1>

        <div class="pb-5">
            Create a Todo:
            <button on:click=on_show_modal_add_event
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mx-2">
                <i class="fa-solid fa-plus"></i>
            </button>
        </div>
        <Suspense>
         {move || match todos.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) if data.is_empty() => view! {<h2>Currently no Todos defined</h2>}.into_view(),
            Some(data) => view! {
                <h2>Start working</h2>
                <For
                    each=move || {data.clone()}
                    key=|item| (item.id.clone(), item.description.clone())
                    let:child
                >
                <TodoListItem todo=child on_edit=on_edit_todo_event on_delete=on_delete_todo_event/>
                </For>
            }.into_view()
        }}
        </Suspense>
        </div>

        <Show when = move || show_modal.get()>
            <TodoModal
                on_add=callback_add_todo_event
                on_edit=callback_edit_todo_event
                on_cancel=callback_cancel_add_event
                todo=edit_todo_item.get()>
            </TodoModal>
        </Show>
    }
}