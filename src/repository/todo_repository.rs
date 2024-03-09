use std::collections::HashMap;

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use leptos::logging;

use crate::model::{Error, Todo};
use crate::repository::dynamo_connection::create_db;

static TABLE_NAME: &str = "TodoTable";

pub async fn get_all() -> Result<Vec<Todo>, Error> {
    logging::log!("[REPOSITORY][get_all]");
    let config = create_db().await;
    let client = Client::from_conf(config);

    let req = client
        .scan()
        .table_name(TABLE_NAME)
        .limit(20);

    let result = req.send()
        .await
        .map(|result|
            result.items
                .map(|item| todo_list_mapper(item)))
        .map_err(|error| {
            logging::error!("{:?}", error);
            error
        })?;

    Ok(result.unwrap())
}


pub async fn get_todo(id: &str) -> Result<Option<Todo>, Error> {
    logging::log!("[REPOSITORY][get_todo]");
    let config = create_db().await;
    let client = Client::from_conf(config);

    // Example for Shadowing
    let id = AttributeValue::S(id.to_string());

    let req = client
        .query()
        .table_name(TABLE_NAME)
        .key_condition_expression("id = :id")
        .expression_attribute_values(":id", id)
        .limit(2);

    let result = req.send()
        .await
        .map(|result|
            result.items
                .map(|item| todo_list_mapper(item)))?;
    let todos = result.unwrap();
    if todos.len() > 1 {
        panic!("More than one item found");
    }

    Ok(todos.get(0).map(|x| x.to_owned()))
}


pub async fn insert_todo(todo: Todo) -> Result<(), Error> {
    logging::log!("[REPOSITORY][insert_todo]");
    let config = create_db().await;
    let client = Client::from_conf(config);

    let id = AttributeValue::S(todo.id);
    let title = AttributeValue::S(todo.title);
    let description = AttributeValue::S(todo.description);
    let created = AttributeValue::N(todo.created.timestamp_millis().to_string());

    let request = client
        .put_item()
        .table_name(TABLE_NAME)
        .item("id", id)
        .item("title", title)
        .item("description", description)
        .item("created", created);

    request.send().await?;

    Ok(())
}


pub async fn update_todo(todo: Todo) -> Result<(), Error> {
    logging::log!("[REPOSITORY][update_todo]");
    let config = create_db().await;
    let client = Client::from_conf(config);

    let id = AttributeValue::S(todo.id);
    let title = AttributeValue::S(todo.title);
    let description = AttributeValue::S(todo.description);

    let request = client
        .update_item()
        .table_name(TABLE_NAME)
        .key("id", id)
        .update_expression("set title = :title, description = :description")
        .expression_attribute_values(":title", title)
        .expression_attribute_values(":description", description);

    request.send().await?;

    Ok(())
}


pub async fn delete_todo(id: &str) -> Result<(), Error> {
    logging::log!("[REPOSITORY][delete_todo]");
    let config = create_db().await;
    let client = Client::from_conf(config);

    let id = AttributeValue::S(id.to_string());

    let req = client
        .delete_item()
        .table_name(TABLE_NAME)
        .key("id", id);

    req.send().await?;

    Ok(())
}


fn todo_mapper(data: HashMap<String, AttributeValue>) -> Todo {
    let id = data.get("id").unwrap().as_s().unwrap().clone();
    let title = data.get("title").unwrap().as_s().unwrap().clone();
    let description = data.get("description").unwrap().as_s().unwrap().clone();
    let created = data.get("created").unwrap().as_n().unwrap().clone();
    let created_n = created.parse::<i64>().expect(&*format!("unparsable DATE/TIME for {}", id));
    Todo { id, title, description, created: chrono::DateTime::from_timestamp_millis(created_n).unwrap() }
}


fn todo_list_mapper(data: Vec<HashMap<String, AttributeValue>>) -> Vec<Todo> {
    data.iter().map(|item| todo_mapper(item.clone())).collect()
}