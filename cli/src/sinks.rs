use rustyline::DefaultEditor;

use crate::{
    api_client::{
        ApiClient, CreateSinkRequest, CreateSinkResponse, SinkConfig, SinkResponse,
        UpdateSinkRequest,
    },
    get_id, get_string,
    tenants::get_tenant_id,
    CliError,
};

pub async fn create_sink(
    api_client: &ApiClient,
    editor: &mut DefaultEditor,
) -> Result<CreateSinkResponse, CliError> {
    let tenant_id = get_tenant_id(editor)?;
    let config = get_sink_config(editor)?;
    let sink = api_client
        .create_sink(tenant_id, &CreateSinkRequest { config })
        .await?;

    Ok(sink)
}

pub async fn show_sink(
    api_client: &ApiClient,
    editor: &mut DefaultEditor,
) -> Result<SinkResponse, CliError> {
    let tenant_id = get_tenant_id(editor)?;
    let sink_id = get_sink_id(editor)?;

    let sink = api_client.read_sink(tenant_id, sink_id).await?;

    Ok(sink)
}

pub async fn update_sink(
    api_client: &ApiClient,
    editor: &mut DefaultEditor,
) -> Result<(), CliError> {
    let tenant_id = get_tenant_id(editor)?;
    let sink_id = get_sink_id(editor)?;
    let config = get_sink_config(editor)?;

    let sink = UpdateSinkRequest { config };
    api_client.update_sink(tenant_id, sink_id, &sink).await?;

    Ok(())
}

pub async fn delete_sink(
    api_client: &ApiClient,
    editor: &mut DefaultEditor,
) -> Result<(), CliError> {
    let tenant_id = get_tenant_id(editor)?;
    let sink_id = get_sink_id(editor)?;

    api_client.delete_sink(tenant_id, sink_id).await?;

    Ok(())
}

pub async fn list_sinks(
    api_client: &ApiClient,
    editor: &mut DefaultEditor,
) -> Result<Vec<SinkResponse>, CliError> {
    let tenant_id = get_tenant_id(editor)?;
    let tenants = api_client.read_all_sinks(tenant_id).await?;

    Ok(tenants)
}

fn get_sink_config(editor: &mut DefaultEditor) -> Result<SinkConfig, CliError> {
    let project_id = get_string(editor, "enter project_id: ")?;
    let dataset_id = get_string(editor, "enter dataset_id: ")?;
    let service_account_key = get_string(editor, "enter service_account_key: ")?;
    Ok(SinkConfig::BigQuery {
        project_id,
        dataset_id,
        service_account_key,
    })
}

pub fn get_sink_id(editor: &mut DefaultEditor) -> Result<i64, CliError> {
    get_id(editor, "enter sink id: ")
}
