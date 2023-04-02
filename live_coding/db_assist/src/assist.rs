use pgx::prelude::*;

#[pg_schema]
pub mod db_assist {
    use askama::Template;
    use async_openai::{
        types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
        Client,
    };
    use db_schema::PgSchema;
    use futures::StreamExt;
    use pgx::prelude::*;
    use tokio::runtime::Builder;

    #[derive(Template)]
    #[template(path = "prompt.txt")]
    pub struct PromptTemplate<'a> {
        context: &'a str,
        question: &'a str,
    }

    #[pg_extern]
    fn all(namespace: &str) -> String {
        get_all_schema(namespace).unwrap()
    }

    #[pg_extern]
    fn h(question: &str, namespace: &str) -> String {
        assist(question, namespace).unwrap()
    }

    fn assist(question: &str, namespace: &str) -> anyhow::Result<String> {
        let rt = Builder::new_current_thread().enable_all().build()?;
        let result = rt.block_on(async move {
            let client = Client::new();
            let mut answer = String::new();
            let context = get_all_schema(namespace)?;
            let tpl = PromptTemplate {
                context: &context,
                question,
            };
            let prompt = tpl.render()?;
            info!("{}", question);
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

            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        response.choices.iter().for_each(|chat_choice| {
                            if let Some(ref content) = chat_choice.delta.content {
                                answer.push_str(content);
                            }
                        });
                    }
                    Err(err) => {
                        error!("error: {err}");
                    }
                }
            }
            Ok::<_, anyhow::Error>(answer)
        });

        result
    }

    fn get_all_schema(namespace: &str) -> Result<String, spi::Error> {
        let schema = PgSchema::new(namespace);
        let mut items = vec![];

        items.push(get_spi_result(&schema.types())?);
        items.push(get_spi_result(&schema.enums())?);
        items.push(get_spi_result(&schema.tables())?);
        items.push(get_spi_result(&schema.views())?);
        items.push(get_spi_result(&schema.mviews())?);
        items.push(get_spi_result(&schema.functions())?);
        items.push(get_spi_result(&schema.triggers())?);
        items.push(get_spi_result(&schema.indexes())?);

        Ok(items.join("\n"))
    }

    fn get_spi_result(query: &str) -> Result<String, spi::Error> {
        let items = Spi::connect(|client| {
            let tbl = client.select(query, None, None)?;

            let mut items = Vec::new();
            for row in tbl {
                let sql: String = row.get_by_name("sql")?.unwrap();
                items.push(sql);
            }
            Ok::<_, spi::Error>(items)
        })?;

        Ok(items.join("\n"))
    }
}
