use salvo::prelude::*;
use routes::collection;

use crate::routes::document;

mod routes;
mod api_error;
mod api_result;

#[tokio::main]
async fn main() {
    let collection_router =
        Router::with_path("collection")
            .push(
                Router::with_path("<collection_id>")
                    .get(collection::info)
                    .put(collection::create)
                    .patch(collection::update)
                    .push(
                        Router::with_path("search")
                            .get(collection::search)
                    )
                    .push(
                        Router::with_path("commit")
                            .get(collection::commit)
                    )
                    .push(
                        Router::with_path("documents")
                            .put(document::create)
                            .push(
                                Router::with_path("<document_id>")
                                    .get(document::info)
                                    .post(document::update)
                                    .delete(document::remove)
                            )
                    ),
            )
        ;

    Server::new(TcpListener::bind("127.0.0.1:1551"))
        .serve(collection_router)
        .await;
}
