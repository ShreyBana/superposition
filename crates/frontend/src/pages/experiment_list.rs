pub mod utils;

use futures::join;
use leptos::*;

use chrono::{prelude::Utc, TimeZone};
use serde::{Deserialize, Serialize};

use crate::components::drawer::{close_drawer, Drawer, DrawerBtn};
use crate::components::skeleton::Skeleton;
use crate::components::table::types::TablePaginationProps;
use crate::components::{experiment_form::ExperimentForm, stat::Stat, table::Table};

use crate::providers::condition_collapse_provider::ConditionCollapseProvider;
use crate::providers::editor_provider::EditorProvider;
use crate::types::{ExpListFilters, ExperimentResponse, ListFilters, PaginatedResponse};
use crate::utils::update_page_direction;

use self::utils::experiment_table_columns;
use crate::{
    api::{fetch_default_config, fetch_dimensions, fetch_experiments},
    types::{DefaultConfig, Dimension},
};
use serde_json::{json, Map, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CombinedResource {
    experiments: PaginatedResponse<ExperimentResponse>,
    dimensions: Vec<Dimension>,
    default_config: Vec<DefaultConfig>,
}

#[component]
pub fn experiment_list() -> impl IntoView {
    // acquire tenant
    let tenant_rs = use_context::<ReadSignal<String>>().unwrap();
    let (exp_filters, _set_exp_filters) = create_signal(ExpListFilters {
        status: None,
        from_date: Utc.timestamp_opt(0, 0).single(),
        to_date: Utc.timestamp_opt(4130561031, 0).single(),
    });
    let (pagination_filters, set_pagination_filters) = create_signal(ListFilters {
        page: Some(1),
        count: Some(10),
        all: None,
    });

    let (reset_exp_form, set_exp_form) = create_signal(0);
    let table_columns = store_value(experiment_table_columns());

    let combined_resource: Resource<
        (String, ExpListFilters, ListFilters),
        CombinedResource,
    > = create_blocking_resource(
        move || (tenant_rs.get(), exp_filters.get(), pagination_filters.get()),
        |(current_tenant, exp_filters, pagination_filters)| async move {
            // Perform all fetch operations concurrently
            let experiments_future = fetch_experiments(
                exp_filters,
                pagination_filters,
                current_tenant.to_string(),
            );
            let empty_list_filters = ListFilters {
                page: None,
                count: None,
                all: Some(true),
            };
            let dimensions_future =
                fetch_dimensions(empty_list_filters.clone(), current_tenant.to_string());
            let config_future =
                fetch_default_config(empty_list_filters, current_tenant.to_string());

            let (experiments_result, dimensions_result, config_result) =
                join!(experiments_future, dimensions_future, config_future);
            // Construct the combined result, handling errors as needed
            CombinedResource {
                experiments: experiments_result.unwrap_or(PaginatedResponse::default()),
                dimensions: dimensions_result
                    .unwrap_or(PaginatedResponse::default())
                    .data
                    .into_iter()
                    .filter(|d| d.dimension != "variantIds")
                    .collect(),
                default_config: config_result
                    .unwrap_or(PaginatedResponse::default())
                    .data,
            }
        },
    );

    let handle_submit_experiment_form = move || {
        combined_resource.refetch();
        set_exp_form.update(|val| {
            *val += 1;
        });
        close_drawer("create_exp_drawer");
    };

    let handle_next_click = Callback::new(move |total_pages: i64| {
        set_pagination_filters.update(|f| {
            f.page = update_page_direction(f.page, total_pages, true);
        });
    });

    let handle_prev_click = Callback::new(move |_| {
        set_pagination_filters.update(|f| {
            f.page = update_page_direction(f.page, 1, false);
        });
    });

    // TODO: Add filters
    view! {
        <div class="p-8">
            <Suspense fallback=move || view! { <Skeleton/> }>
                <div class="pb-4">

                    {move || {
                        let value = combined_resource.get();
                        let total_items = match value {
                            Some(v) => v.experiments.total_items.to_string(),
                            _ => "0".to_string(),
                        };
                        view! {
                            <Stat
                                heading="Experiments"
                                icon="ri-test-tube-fill"
                                number=total_items
                            />
                        }
                    }}

                </div>
                <div class="card rounded-xl w-full bg-base-100 shadow">
                    <div class="card-body">
                        <div class="flex justify-between">
                            <h2 class="card-title">Experiments</h2>
                            <div>
                                <DrawerBtn drawer_id="create_exp_drawer"
                                    .to_string()>
                                    Create Experiment <i class="ri-edit-2-line ml-2"></i>
                                </DrawerBtn>
                            </div>
                        </div>
                        {move || {
                            let value = combined_resource.get();
                            let pagination_filters = pagination_filters.get();
                            match value {
                                Some(v) => {
                                    let data = v
                                        .experiments
                                        .data
                                        .iter()
                                        .map(|ele| {
                                            let mut ele_map = json!(ele)
                                                .as_object()
                                                .unwrap()
                                                .to_owned();
                                            ele_map
                                                .insert(
                                                    "created_at".to_string(),
                                                    json!(ele.created_at.format("%v").to_string()),
                                                );
                                            ele_map
                                                .insert(
                                                    "last_modified".to_string(),
                                                    json!(ele.last_modified.format("%v").to_string()),
                                                );
                                            ele_map
                                        })
                                        .collect::<Vec<Map<String, Value>>>()
                                        .to_owned();
                                    let pagination_props = TablePaginationProps {
                                        enabled: true,
                                        count: pagination_filters.count.unwrap_or_default(),
                                        current_page: pagination_filters.page.unwrap_or_default(),
                                        total_pages: v.experiments.total_pages,
                                        on_next: handle_next_click,
                                        on_prev: handle_prev_click,
                                    };
                                    view! {
                                        <ConditionCollapseProvider>
                                            <Table
                                                cell_class="min-w-48 font-mono".to_string()
                                                rows=data
                                                key_column="id".to_string()
                                                columns=table_columns.get_value()
                                                pagination=pagination_props
                                            />
                                        </ConditionCollapseProvider>
                                    }
                                }
                                None => view! { <div>Loading....</div> }.into_view(),
                            }
                        }}

                    </div>
                </div>

                {move || {
                    let dim = combined_resource
                        .get()
                        .unwrap_or(CombinedResource {
                            experiments: PaginatedResponse::default(),
                            dimensions: vec![],
                            default_config: vec![],
                        })
                        .dimensions;
                    let def_conf = combined_resource
                        .get()
                        .unwrap_or(CombinedResource {
                            experiments: PaginatedResponse::default(),
                            dimensions: vec![],
                            default_config: vec![],
                        })
                        .default_config;
                    let _ = reset_exp_form.get();
                    view! {
                        <Drawer
                            id="create_exp_drawer".to_string()
                            header="Create New Experiment"
                            handle_close=move || {
                                close_drawer("create_exp_drawer");
                                set_exp_form.update(|i| *i += 1);
                            }
                        >

                            <EditorProvider>
                                <ExperimentForm
                                    name="".to_string()
                                    context=vec![]
                                    variants=vec![]
                                    dimensions=dim.clone()
                                    default_config=def_conf.clone()
                                    handle_submit=handle_submit_experiment_form
                                />
                            </EditorProvider>
                        </Drawer>
                    }
                }}

            </Suspense>
        </div>
    }
}
