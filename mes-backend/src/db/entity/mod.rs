use sea_orm::DatabaseConnection;

pub mod equipment;
pub mod equipment_status_log;
pub mod equipment_mqtt_config;
pub mod material_categories;
pub mod materials;
pub mod suppliers;
pub mod customers;
pub mod production_plans;
pub mod production_orders;
pub mod work_orders;
pub mod production_reports;
pub mod production_schedules;
pub mod inventory;
pub mod inbound_orders;
pub mod inbound_order_details;
pub mod departments;
pub mod users;
pub mod roles;
pub mod permissions;
pub mod user_roles;
pub mod role_permissions;

pub use equipment::Entity as Equipment;
pub use equipment_status_log::Entity as EquipmentStatusLog;
pub use equipment_mqtt_config::Entity as EquipmentMqttConfig;
pub use material_categories::Entity as MaterialCategories;
pub use materials::Entity as Materials;
pub use suppliers::Entity as Suppliers;
pub use customers::Entity as Customers;
pub use production_plans::Entity as ProductionPlans;
pub use production_orders::Entity as ProductionOrders;
pub use work_orders::Entity as WorkOrders;
pub use production_reports::Entity as ProductionReports;
pub use production_schedules::Entity as ProductionSchedules;
pub use inventory::Entity as Inventory;
pub use inbound_orders::Entity as InboundOrders;
pub use inbound_order_details::Entity as InboundOrderDetails;
pub use departments::Entity as Departments;
pub use users::Entity as Users;
pub use roles::Entity as Roles;
pub use permissions::Entity as Permissions;
pub use user_roles::Entity as UserRoles;
pub use role_permissions::Entity as RolePermissions;

// Helper type alias so DAOs can accept a generic connection
pub type ConnRef<'a> = &'a DatabaseConnection;


