//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use super::sea_orm_active_enums::UserPreference;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "user_publications_preference_interaction"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub profile: i64,
    pub post: Option<i64>,
    pub comment: Option<i64>,
    pub r#type: UserPreference,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Profile,
    Post,
    Comment,
    Type,
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
    Comment,
    Post,
    Profile,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Profile => ColumnType::BigInteger.def(),
            Self::Post => ColumnType::BigInteger.def().null(),
            Self::Comment => ColumnType::BigInteger.def().null(),
            Self::Type => UserPreference::db_type().def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Comment => Entity::belongs_to(super::comment::Entity)
                .from(Column::Comment)
                .to(super::comment::Column::Id)
                .into(),
            Self::Post => Entity::belongs_to(super::post::Entity)
                .from(Column::Post)
                .to(super::post::Column::Id)
                .into(),
            Self::Profile => Entity::belongs_to(super::profile::Entity)
                .from(Column::Profile)
                .to(super::profile::Column::Id)
                .into(),
        }
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
