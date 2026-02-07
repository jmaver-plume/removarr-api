use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "vote_movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub voter_id: i32,
    pub movie_id: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::voter::Entity",
        from = "Column::VoterId",
        to = "super::voter::Column::Id",
        on_delete = "Cascade"
    )]
    Voter,
    #[sea_orm(
        belongs_to = "super::movie::Entity",
        from = "Column::MovieId",
        to = "super::movie::Column::Id",
        on_delete = "Cascade"
    )]
    Movie,
}

impl Related<super::voter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Voter.def()
    }
}

impl Related<super::movie::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movie.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

