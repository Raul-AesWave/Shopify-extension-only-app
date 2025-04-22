use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function_target(query_path = "src/run.graphql", schema_path = "schema.graphql")]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut discounts = vec![];

    for line in input.cart.lines.iter() {
        if let Some(custom_length) = &line.custom_length {
            if let Some(value_str) = &custom_length.value {
                if let Ok(length) = value_str.parse::<f64>() {
                    let discount_feet = if length < 20.0 {
                        20 - length.trunc() as i64
                    } else {
                        0
                    };
                
                    let amount = Decimal::from(discount_feet as f64); // $X.00 off
                
                    discounts.push(output::Discount {
                        message: Some(format!("{} ft selected", length)),
                        targets: vec![output::Target::CartLine(output::CartLineTarget {
                            id: line.id.to_string(),
                            quantity: None,
                        })],
                        value: output::Value::FixedAmount(output::FixedAmount {
                            amount,
                            applies_to_each_item: Some(false),
                        }),
                    });
                }
                
            }
        }
    }

    Ok(output::FunctionRunResult {
        discounts,
        discount_application_strategy: output::DiscountApplicationStrategy::ALL,
    })
}
