use crate::domain::chat::{DMThread, DirectMessage};
use secrecy::Secret;
use sqlx::{postgres::PgQueryResult, query, Error, PgPool};

pub const DM_THREADS_TABLE_NAME: &str = "dm_threads";
pub const DIRECT_MESSAGES_TABLE_NAME: &str = "direct_messages";

pub const GROUP_CHATS: &str = "group_chats";
pub const GROUP_CHAT_MEMBERS_TABLE_NAME: &str = "group_chat_members";
pub const GROUP_CHAT_MESSAGES_TABLE_NAME: &str = "group_chat_messages";
pub const GROUP_CHAT_MESSAGE_READS_TABLE_NAME: &str = "group_chat_message_reads";
pub const GROUP_CHAT_MESSAGE_REACTIONS_TABLE_NAME: &str = "group_chat_message_reactions";

pub const SERVER_THREADS_TABLE_NAME: &str = "server_threads";
pub const SERVER_MESSAGES_TABLE_NAME: &str = "server_messages";
pub const SERVER_MESSAGE_READS_TABLE_NAME: &str = "server_message_reads";
pub const SERVER_MESSAGE_REACTIONS_TABLE_NAME: &str = "server_message_reactions";

pub async fn upsert_dm_thread(
    db_pool: &PgPool,
    dm_thread: DMThread,
) -> Result<PgQueryResult, Error> {
    query(
        r#"
        INSERT INTO dm_threads (id, first_user_id, second_user_id, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id)
        DO
            UPDATE SET
                first_user_id = EXCLUDED.first_user_id,
                second_user_id = EXCLUDED.second_user_id,
                updated_at = now(),
                deleted_at = EXCLUDED.deleted_at
        WHERE
            (dm_threads.first_user_id, dm_threads.second_user_id, dm_threads.deleted_at) IS DISTINCT FROM
            (EXCLUDED.first_user_id, EXCLUDED.second_user_id, EXCLUDED.deleted_at);
        "#)
        .bind(dm_thread.id())
        .bind(dm_thread.first_user_id())
        .bind(dm_thread.second_user_id())
        .bind(dm_thread.created_at())
        .bind(dm_thread.updated_at())
        .bind(dm_thread.deleted_at())
    .execute(db_pool)
    .await
}

pub async fn upsert_direct_message(db_pool: &PgPool, direct_message: DirectMessage) -> Result<PgQueryResult, Error> {
    query(
        r#"
        INSERT INTO direct_messages (id, thread_id, sender_id, message, is_read, reaction, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT
        DO
            UPDATE SET
                thread_id = EXCLUDED.thread_id,
                sender_id = EXCLUDED.sender_id,
                message = EXCLUDED.message,
                is_read = EXCLUDED.is_read,
                reaction = EXCLUDED.reaction,
                updated_at = now(),
                deleted_at = EXCLUDED.deleted_at
        WHERE
            (direct_messages.thread_id, direct_messages.sender_id, direct_messages.message, direct_messages.is_read, direct_messages.reaction, direct_messages.deleted_at) IS DISTINCT FROM
            (EXCLUDED.thread_id, EXCLUDED.sender_id, EXCLUDED.message, EXCLUDED.is_read, EXCLUDED.reaction, EXCLUDED.deleted_at)
        "#
    )
    .bind(direct_message.id())
    .bind(direct_message.thread_id())
    .bind(direct_message.sender_id())
    .bind(direct_message.message())
    .bind(direct_message.is_read())
    .bind(direct_message.reaction())
    .bind(direct_message.created_at())
    .bind(direct_message.updated_at())
    .bind(direct_message.deleted_at())
    .execute(db_pool)
    .await
}