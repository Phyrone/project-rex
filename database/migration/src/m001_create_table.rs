use sea_orm_migration::prelude::*;

const DOMAIN_LENGTH: u32 = 253;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Node::Table)
                    .col(
                        ColumnDef::new(Node::Id)
                            .big_integer()
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Node::Domain)
                            .string_len(DOMAIN_LENGTH)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Node::AliasDomains)
                            .array(ColumnType::String(Some(DOMAIN_LENGTH)))
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Node::FirstSeen)
                            .timestamp()
                            .not_null()
                            .default("now()"),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .nulls_not_distinct()
                            .table(Node::Table)
                            .col(Node::Domain),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .table(Node::Table)
                            .col(Node::AliasDomains),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .col(
                        ColumnDef::new(Profile::Id)
                            .big_unsigned()
                            .big_integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Profile::NodeID)
                            .big_unsigned()
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Profile::DisplayName).not_null().string())
                    .col(ColumnDef::new(Profile::Meta).json().default("{}"))
                    .primary_key(
                        Index::create()
                            .table(Profile::Table)
                            .col(Profile::Id)
                            .col(Profile::NodeID),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Profile::FkNode.to_string())
                            .from_tbl(Profile::Table)
                            .from_col(Profile::NodeID)
                            .to_tbl(Node::Table)
                            .to_col(Node::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(
                        ColumnDef::new(Post::Id)
                            .big_unsigned()
                            .big_integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Post::NodeID)
                            .big_unsigned()
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Post::SenderId)
                            .big_unsigned()
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default("now()"),
                    )
                    .col(ColumnDef::new(Post::Title).not_null().string())
                    .col(ColumnDef::new(Post::Content).not_null().json())
                    .col(ColumnDef::new(Post::Meta).json().default("{}").not_null())
                    .primary_key(
                        Index::create()
                            .table(Post::Table)
                            .col(Post::Id)
                            .col(Post::NodeID),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Post::FkSender.to_string())
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_tbl(Profile::Table)
                            .from_col(Post::SenderId)
                            .to_col(Profile::Id)
                            .from_col(Post::NodeID)
                            .to_col(Profile::NodeID),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Post::FkNode.to_string())
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_tbl(Node::Table)
                            .from_col(Post::NodeID)
                            .to_col(Node::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Node::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Node {
    Table,
    Id,
    Domain,
    AliasDomains,
    FirstSeen,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    NodeID,
    SenderId,
    CreatedAt,
    Title,
    Content,
    Meta,
    #[sea_orm(iden = "fk_post_sender")]
    FkSender,
    #[sea_orm(iden = "fk_post_node")]
    FkNode,
}

#[derive(DeriveIden)]
enum Profile {
    Table,
    Id,
    NodeID,
    DisplayName,
    Meta,

    #[sea_orm(iden = "fk_user_node")]
    FkNode,
}

#[derive(DeriveIden)]
enum PostAttachment {
    Table,
    NodeID,
    PostID,
    Position,
}
