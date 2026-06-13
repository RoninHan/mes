use crate::db::entity::{self, user_roles, ConnRef};
use sea_orm::{prelude::*, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

pub async fn list_roles_for_user(
    conn: ConnRef<'_>,
    user_id: i64,
) -> Result<Vec<user_roles::Model>, DbErr> {
    user_roles::Entity::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .all(conn)
}

pub async fn replace_user_roles(
    conn: ConnRef<'_>,
    user_id: i64,
    role_ids: &[i64],
    operator_id: i64,
) -> Result<(), DbErr> {
    let txn = conn.begin().await?;
    user_roles::Entity::delete_many()
        .filter(user_roles::Column::UserId.eq(user_id))
        .exec(&txn)
        .await?;

    for rid in role_ids {
        let active = user_roles::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(*rid),
            created_by: Set(Some(operator_id)),
            ..Default::default()
        };
        user_roles::Entity::insert(active).exec(&txn).await?;
    }
    txn.commit().await
}


