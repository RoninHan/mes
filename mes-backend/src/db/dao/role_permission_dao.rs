use crate::db::entity::{self, role_permissions, ConnRef};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

pub async fn list_permissions_for_role(
    conn: ConnRef<'_>,
    role_id: i64,
) -> sea_orm::DbResult<Vec<role_permissions::Model>> {
    entity::RolePermissions::find()
        .filter(role_permissions::Column::RoleId.eq(role_id))
        .all(conn)
}

pub async fn replace_role_permissions(
    conn: ConnRef<'_>,
    role_id: i64,
    perm_ids: &[i64],
    operator_id: i64,
) -> sea_orm::DbResult<()> {
    let txn = conn.begin().await?;
    entity::RolePermissions::delete_many()
        .filter(role_permissions::Column::RoleId.eq(role_id))
        .exec(&txn)
        .await?;
    for pid in perm_ids {
        let active = role_permissions::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(*pid),
            created_by: Set(Some(operator_id)),
            ..Default::default()
        };
        entity::RolePermissions::insert(active).exec(&txn).await?;
    }
    txn.commit().await
}


