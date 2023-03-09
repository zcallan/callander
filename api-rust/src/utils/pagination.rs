use diesel::{
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment},
    query_dsl::LoadQuery,
    sql_types::BigInt,
    PgConnection, QueryResult, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

const DEFAULT_PER_PAGE: i64 = 10;

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

#[derive(Deserialize, Serialize, Clone, Copy, TS, QueryId, Debug)]
#[ts(export)]
pub struct Paginated<T> {
    pub items: T,

    #[ts(type = "number")]
    pub current_page: i64,

    #[ts(type = "number")]
    pub per_page: i64,

    #[ts(type = "number")]
    pub total_items: i64,

    #[ts(type = "number")]
    pub total_pages: i64,

    #[ts(type = "number")]
    pub offset: i64,

    pub has_next_page: bool,
    pub has_prev_page: bool,
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated {
            per_page,
            offset: (self.current_page - 1) * per_page,
            ..self
        }
    }

    pub fn load_and_count_pages<'a, U>(
        self,
        conn: &mut PgConnection,
    ) -> QueryResult<(Vec<U>, i64, i64)>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total_items = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_items, total_pages))
    }
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            items: self,
            current_page: page,
            per_page: DEFAULT_PER_PAGE,
            offset: 0,
            total_items: 0,
            total_pages: 0,
            has_next_page: false,
            has_prev_page: false,
        }
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.items.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
