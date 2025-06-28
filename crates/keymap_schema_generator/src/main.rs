use anyhow::Result;
use clap::{Arg, Command};
use gpui::Application;
use settings::KeymapFile;
use std::fs;
use std::sync::{Arc, Mutex};

fn make_app() -> Command {
    Command::new("keymap_schema_generator")
        .about("Generates JSON schema for Zed's keymap.json files")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path for the JSON schema")
                .value_name("FILE")
                .default_value("keymap-schema.json"),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty-print the JSON schema")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("metadata")
                .short('m')
                .long("metadata")
                .help("Also output metadata about actions to a separate file")
                .action(clap::ArgAction::SetTrue),
        )
}

fn main() -> Result<()> {
    let matches = make_app().get_matches();

    // Call a zed:: function so everything in `zed` crate is linked and
    // all actions in the actual app are registered
    zed::stdout_is_a_pty();

    let output_path = matches.get_one::<String>("output").unwrap().clone();
    let pretty = matches.get_flag("pretty");
    let include_metadata = matches.get_flag("metadata");

    // Store the results in a shared container
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();

    // Create a minimal GPUI application to generate the schema
    Application::new().run(move |cx| {
        let schema = KeymapFile::generate_json_schema_for_registered_actions(cx);

        let json_output = if pretty {
            serde_json::to_string_pretty(&schema).unwrap()
        } else {
            serde_json::to_string(&schema).unwrap()
        };

        fs::write(&output_path, &json_output).unwrap();

        println!(
            "✅ Successfully generated keymap JSON schema at: {}",
            &output_path
        );

        // Print some metadata about the schema
        if let Some(definitions) = schema.get("definitions") {
            if let Some(definitions_obj) = definitions.as_object() {
                println!(
                    "📊 Schema contains {} type definitions",
                    definitions_obj.len()
                );
            }
        }

        // Generate metadata if requested
        if include_metadata {
            let actions = gpui::generate_list_of_all_registered_actions();
            let mut action_metadata = serde_json::json!({
                "total_actions": actions.len(),
                "actions": []
            });

            let mut action_list = Vec::new();
            for action in &actions {
                action_list.push(serde_json::json!({
                    "name": action.name,
                    "deprecated_aliases": action.deprecated_aliases,
                    "humanized_name": command_palette::humanize_action_name(action.name)
                }));
            }

            action_metadata["actions"] = serde_json::Value::Array(action_list);

            let metadata_path = output_path.replace(".json", "-metadata.json");
            let metadata_output = if pretty {
                serde_json::to_string_pretty(&action_metadata).unwrap()
            } else {
                serde_json::to_string(&action_metadata).unwrap()
            };

            fs::write(&metadata_path, metadata_output).unwrap();
            println!("📋 Action metadata saved to: {}", metadata_path);
        }

        // Show sample actions for verification
        let actions = gpui::generate_list_of_all_registered_actions();
        println!("🎯 Total actions registered: {}", actions.len());

        println!("\n📝 Sample actions:");
        for (i, action) in actions.iter().take(8).enumerate() {
            println!("  {}. {}", i + 1, action.name);
        }

        if actions.len() > 8 {
            println!("  ... and {} more", actions.len() - 8);
        }

        // Store success in the result container
        *result_clone.lock().unwrap() = Some(());

        // Exit the application immediately
        cx.quit();
    });

    // Check if the operation was successful
    if result.lock().unwrap().is_some() {
        println!("\n🎉 Schema generation completed successfully!");
    } else {
        anyhow::bail!("Failed to generate schema");
    }

    Ok(())
}
