use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function_target(query_path = "src/run.graphql", schema_path = "schema.graphql")]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let mut discounts = vec![];
    
    for line in input.cart.lines.iter() {
        let default_price = line.cost.subtotal_amount.amount.to_string().parse::<f64>()?;
        
        if let Some(price_change) = &line.price_change {
            
            if let Some(price_change_value) = &price_change.value {
                
                if let Ok(new_price) = price_change_value.parse::<f64>() {
                    
                    if default_price != new_price {
                        let discount_amount = default_price - new_price;

                        discounts.push(output::Discount {
                            message: Some(format!("${}.00 off", discount_amount.trunc() as i64)),
                            targets: vec![output::Target::CartLine(output::CartLineTarget {
                                id: line.id.to_string(),
                                quantity: None,
                            })],
                            value: output::Value::FixedAmount(output::FixedAmount {
                                amount: Decimal::from(discount_amount as f64),

                                applies_to_each_item: Some(false),
                            }),
                        });
                    }
                }
            }
        }
    }

    Ok(output::FunctionRunResult {
        discounts,
        discount_application_strategy: output::DiscountApplicationStrategy::ALL,
    })
}
