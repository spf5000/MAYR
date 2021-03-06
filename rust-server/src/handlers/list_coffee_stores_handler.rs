use crate::dao::coffee_store::CoffeeStoreDao;
use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;
use crate::error::ServerError;
use rust_server_model::coffee_store::{ListCoffeeStoresRequest, ListCoffeeStoresResponse};

#[post("/coffee/list")]
pub async fn list_coffee_stores(
    request: web::Bytes,
    handler: web::Data<ListCoffeeStoresHandler>,
) -> impl Responder {
    log::info!("Listing Coffee Stores");
    let request = match serde_json::from_slice(&request) {
        Result::Ok(request) => request,
        Result::Err(err) => {
            // TODO: Invalid input request?
            log::error!("Failed to deserialize request: {:?}", request);
            return ServerError::from(err).into()
        }
    };
    match handler.handle(request) {
        Ok(response) => response,
        Err(err) => {
            log::error!("Handler returned an error: {}", err);
            err.into()
        }
    }
}

#[derive(Clone)]
pub struct ListCoffeeStoresHandler {
    coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
}

impl ListCoffeeStoresHandler {
    pub fn new(
        coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
    ) -> ListCoffeeStoresHandler {
        ListCoffeeStoresHandler { coffee_store_dao }
    }

    fn handle(&self, _req: ListCoffeeStoresRequest) -> Result<HttpResponse, ServerError> {
        // // TODO: paginate DAO
        let dao_response = self.coffee_store_dao.list_stores()?;
        let handler_response = ListCoffeeStoresResponse {
            coffee_stores: dao_response,
            next_token: None
        };
        Ok(HttpResponse::Ok()
               .content_type("application/json")
               .body(serde_json::to_string(&handler_response)?),
        )
    }
}
