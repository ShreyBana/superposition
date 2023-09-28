use actix_web::web::Data;
use experimentation_platform::{
  api::experiments::types::{VariantType, Variant}, 
  db::models::Experiment
};

use serde_json::{Value, Map};
use service_utils::{
  service::types::AppState,
  types as app, errors::types::Error as err,
};

pub fn fetch_variant_id(
  experiment: &Experiment,
  variant: VariantType,
) -> app::Result<String> {
  let variants = &experiment.variants;
  let experiment_variants: Vec<Variant> = serde_json::from_value(variants.clone())
      .map_err(|e| {
      log::error!("parsing to variant type failed with err: {e}");
      err::InternalServerErr("".to_string())
  })?;

  for ele in experiment_variants {
    if ele.variant_type == variant {
        return Ok(ele.id);
    }
  }
  log::info!("Failed to fetch variant {:?} id for exp {}", variant, experiment.id);
  return Err(err::InternalServerErr("".to_string()));
}

pub fn get_resolved_config(
  state: &Data<AppState>,
  dimension_ctx : &Map<String, Value>,
) -> app::Result<Value> {
  let http_client = reqwest::Client::new();
  let url = format!("{}/config/resolve", state.cac_host);
  let resp = http_client
      .get(&url)
      .bearer_auth(&state.admin_token)
      .query(dimension_ctx)
      .send()
      .and_then(|mut resp| resp.json())
      .map_err(|e| err::InternalServerErr(e.to_string()))?;
  Ok(resp)
}
