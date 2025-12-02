use crate::db::entity::{self, permissions, role_permissions, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[derive(Debug)]
pub struct PermissionNode {
    pub id: i64,
    pub permission_code: String,
    pub permission_name: String,
    pub permission_type: i8,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub icon: Option<String>,
    pub children: Vec<PermissionNode>,
}

pub async fn list_all(conn: ConnRef<'_>) -> Result<Vec<permissions::Model>> {
    Ok(
        entity::Permissions::find()
            .filter(permissions::Column::IsDeleted.eq(0))
            .all(conn)
            .await?,
    )
}

pub async fn list_by_role_ids(
    conn: ConnRef<'_>,
    role_ids: &[i64],
) -> Result<Vec<permissions::Model>> {
    if role_ids.is_empty() {
        return Ok(Vec::new());
    }
    let perms = entity::Permissions::find()
        .inner_join(entity::RolePermissions)
        .filter(role_permissions::Column::RoleId.is_in(role_ids.iter().cloned()))
        .all(conn)
        .await?;
    Ok(perms)
}

pub fn build_permission_tree(all: Vec<permissions::Model>) -> Vec<PermissionNode> {
    fn build_children(
        parent_id: i64,
        all: &[permissions::Model],
    ) -> Vec<PermissionNode> {
        all.iter()
            .filter(|p| p.parent_id == parent_id && p.is_deleted == 0 && p.status == 1)
            .map(|p| PermissionNode {
                id: p.id,
                permission_code: p.permission_code.clone(),
                permission_name: p.permission_name.clone(),
                permission_type: p.permission_type,
                route_path: p.route_path.clone(),
                component_path: p.component_path.clone(),
                icon: p.icon.clone(),
                children: build_children(p.id, all),
            })
            .collect()
    }
    build_children(0, &all)
}


