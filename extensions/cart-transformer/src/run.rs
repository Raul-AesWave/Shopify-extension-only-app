use shopify_function::prelude::*;
use shopify_function::Result;

#[allow(clippy::upper_case_acronyms)]
type URL = String;

#[shopify_function_target(query_path = "src/run.graphql", schema_path = "schema.graphql")]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut updates = vec![];

    for line in input.cart.lines.iter() {
        if let Some(price_change_attr) = line.price_change.as_ref() {
            if let Some(value_str) = price_change_attr.value.as_ref() {
                if let Ok(new_price) = value_str.parse::<f64>() {
                    eprintln!("_price_change value: {}", new_price);

                    updates.push(output::CartOperation::Update(output::UpdateOperation {
                        cart_line_id: line.id.clone(),
                        title: None,
                        price: Some(output::UpdateOperationPriceAdjustment {
                            adjustment: output::UpdateOperationPriceAdjustmentValue::FixedPricePerUnit(
                                output::UpdateOperationFixedPricePerUnitAdjustment {
                                    amount: Decimal(new_price),
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
