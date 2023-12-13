//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "publication"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub author: i64,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub payload: Json,
    pub pinned: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Author,
    CreatedAt,
    EditedAt,
    Payload,
    Pinned,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Attachment,
    Profile,
    Reaction,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Author => ColumnType::BigInteger.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::EditedAt => ColumnType::DateTime.def().null(),
            Self::Payload => ColumnType::JsonBinary.def(),
            Self::Pinned => ColumnType::Boolean.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Attachment => Entity::has_many(super::attachment::Entity).into(),
            Self::Profile => Entity::belongs_to(super::profile::Entity)
                .from(Column::Author)
                .to(super::profile::Column::Id)
                .into(),
            Self::Reaction => Entity::has_many(super::reaction::Entity).into(),
        }
    }
}

impl Related<super::attachment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attachment.def()
    }
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl Related<super::reaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reaction.def()
    }
}

impl Related<super::asset::Entity> for Entity {
    fn to() -> RelationDef {
        super::attachment::Relation::Asset.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::attachment::Relation::Publication.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
