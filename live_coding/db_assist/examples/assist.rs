use anyhow::Result;
use askama::Template;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use db_schema::PgSchema;
use futures::StreamExt;
use sqlx::PgPool;
use std::io::{stdout, Write};

#[derive(Template)]
#[template(path = "prompt.txt")]
struct PromptTemplate<'a> {
    context: &'a str,
    question: &'a str,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let conn_str = "postgres://postgres:postgres@localhost:5432/reservation";
    let context = query_schema(conn_str, "rsvp").await?;
    let question = "10月后对于 hotel-315 这个 resource_id 的预订情况如何？";
    let tpl = PromptTemplate {
        context: &context,
        question,
    };
    let prompt = tpl.render()?;
    println!("{}", question);
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(1024u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content(prompt)
            .role(Role::User)
            .build()?])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    // For reasons not documented in OpenAI docs / OpenAPI spec,
    // the response of streaming call is different and doesn't include all the same fields.

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}

async fn query_schema(conn_str: &str, namespace: &str) -> Result<String> {
    let schema = PgSchema::new(namespace);
    // get pg pool
    let pool = PgPool::connect(conn_str).await?;
    let types = schema.get_types(&pool).await?.join("\n");
    let enums = schema.get_enums(&pool).await?.join("\n");
    let tables = schema.get_tables(&pool).await?.join("\n");
    let functions = schema.get_functions(&pool).await?.join("\n");
    let views = schema.get_views(&pool).await?.join("\n");
    let mviews = schema.get_mviews(&pool).await?.join("\n");
    let triggers = schema.get_triggers(&pool).await?.join("\n");
    let indexes = schema.get_indexes(&pool).await?.join("\n");

    // concat all these to a single vec
    let mut all = String::new();
    all.push_str(&types);
    all.push_str("\n");
    all.push_str(&enums);
    all.push_str("\n");
    all.push_str(&tables);
    all.push_str("\n");
    all.push_str(&functions);
    all.push_str("\n");
    all.push_str(&views);
    all.push_str("\n");
    all.push_str(&mviews);
    all.push_str("\n");
    all.push_str(&triggers);
    all.push_str("\n");
    all.push_str(&indexes);
    all.push_str("\n");

    Ok(all)
}
