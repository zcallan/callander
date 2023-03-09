/**
 * https://stackoverflow.com/a/62029781
 */
use diesel::expression_methods::ExpressionMethods;
use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::AppearsOnTable;

use crate::diesel::QueryDsl;
use crate::schema::friends;
use crate::schema::friends::BoxedQuery;

pub fn sort_by_column<U: 'static>(
    query: BoxedQuery<'static, Pg>,
    column: U,
    sort_dir: String,
) -> BoxedQuery<'static, Pg>
where
    U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<friends::table> + std::marker::Send,
{
    match sort_dir.as_ref() {
        "asc" => query.order_by(column.asc()),
        "desc" => query.order_by(column.desc()),
        _ => query,
    }
}
