use shopify_function::prelude::*;
use shopify_function::Result;

#[allow(clippy::upper_case_acronyms)]
type URL = String;

#[shopify_function_target(query_path = "src/run.graphql", schema_path = "schema.graphql")]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut updates = vec![];
    let base_length = 10.0;
    let price_per_foot = 1.00;

    for line in input.cart.lines.iter() {
        let base_price = line.cost.amount_per_quantity.amount.0;
        if let Some(length_attr) = line.custom_length.as_ref() {
            if let Some(value_str) = length_attr.value.as_ref() {
                if let Ok(slider_value) = value_str.parse::<f64>() {
                    let length_diff = slider_value - base_length;
                    let adjusted_price = (base_price + (length_diff * price_per_foot)).max(0.0);

                    updates.push(output::CartOperation::Update(output::UpdateOperation {
                        cart_line_id: line.id.clone(),
                        title: None,
                        price: Some(output::UpdateOperationPriceAdjustment {
                            adjustment: output::UpdateOperationPriceAdjustmentValue::FixedPricePerUnit(
                                output::UpdateOperationFixedPricePerUnitAdjustment {
                                    amount: Decimal(adjusted_price),
                                },
                            ),
                        }),
                        image: None,
                    }));
                }
            }
        }
    }
    Ok(output::FunctionRunResult {
        operations: updates,
    })
}
