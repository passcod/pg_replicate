use sqlx::PgPool;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PipelineConfig {
    pub config: BatchConfig,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct BatchConfig {
    /// maximum batch size in number of events
    pub max_size: usize,

    /// maximum duration, in seconds, to wait for a batch to fill
    pub max_fill_secs: u64,
}

pub struct Pipeline {
    pub id: i64,
    pub tenant_id: i64,
    pub source_id: i64,
    pub sink_id: i64,
    pub publication_id: i64,
    pub config: serde_json::Value,
}

pub async fn create_pipeline(
    pool: &PgPool,
    tenant_id: i64,
    source_id: i64,
    sink_id: i64,
    publication_id: i64,
    config: &PipelineConfig,
) -> Result<i64, sqlx::Error> {
    let config = serde_json::to_value(config).expect("failed to serialize config");
    let record = sqlx::query!(
        r#"
        insert into pipelines (tenant_id, source_id, sink_id, publication_id, config)
        values ($1, $2, $3, $4, $5)
        returning id
        "#,
        tenant_id,
        source_id,
        sink_id,
        publication_id,
        config
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn read_pipeline(
    pool: &PgPool,
    tenant_id: i64,
    pipeline_id: i64,
) -> Result<Option<Pipeline>, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        select id, tenant_id, source_id, sink_id, publication_id, config
        from pipelines
        where tenant_id = $1 and id = $2
        "#,
        tenant_id,
        pipeline_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| Pipeline {
        id: r.id,
        tenant_id: r.tenant_id,
        source_id: r.source_id,
        sink_id: r.sink_id,
        publication_id: r.publication_id,
        config: r.config,
    }))
}

pub async fn update_pipeline(
    pool: &PgPool,
    tenant_id: i64,
    pipeline_id: i64,
    source_id: i64,
    sink_id: i64,
    publication_id: i64,
    config: &PipelineConfig,
) -> Result<Option<i64>, sqlx::Error> {
    let config = serde_json::to_value(config).expect("failed to serialize config");
    let record = sqlx::query!(
        r#"
        update pipelines
        set source_id = $1, sink_id = $2, publication_id = $3, config = $4
        where tenant_id = $5 and id = $6
        returning id
        "#,
        source_id,
        sink_id,
        publication_id,
        config,
        tenant_id,
        pipeline_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| r.id))
}

pub async fn delete_pipeline(
    pool: &PgPool,
    tenant_id: i64,
    pipeline_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        delete from pipelines
        where tenant_id = $1 and id = $2
        returning id
        "#,
        tenant_id,
        pipeline_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| r.id))
}

pub async fn read_all_pipelines(
    pool: &PgPool,
    tenant_id: i64,
) -> Result<Vec<Pipeline>, sqlx::Error> {
    let mut record = sqlx::query!(
        r#"
        select id, tenant_id, source_id, sink_id, publication_id, config
        from pipelines
        where tenant_id = $1
        "#,
        tenant_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(record
        .drain(..)
        .map(|r| Pipeline {
            id: r.id,
            tenant_id: r.tenant_id,
            source_id: r.source_id,
            sink_id: r.sink_id,
            publication_id: r.publication_id,
            config: r.config,
        })
        .collect())
}
