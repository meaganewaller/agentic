pub fn explain(output: &crate::pipeline::PipelineOutput) {
    println!("--- Explain Mode ---\n");

    if let Some(obj) = output.merged.as_object() {
        for (key, final_value) in obj {
            println!("{key}");

            for (name, layer) in &output.layers {
                if let Some(value) = layer.get(key) {
                    println!("  {name}: {value}");
                }
            }

            println!("  -> final: {final_value}\n");
        }
    }
}