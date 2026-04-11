//! MCP tool definitions for the billing extension.

use convergio_types::extension::McpToolDef;
use serde_json::json;

pub fn billing_tools() -> Vec<McpToolDef> {
    vec![McpToolDef {
        name: "cvg_cost_summary".into(),
        description: "Get spending overview: total cost, active and total plans.".into(),
        method: "GET".into(),
        path: "/api/cost/summary".into(),
        input_schema: json!({"type": "object", "properties": {}}),
        min_ring: "community".into(),
        path_params: vec![],
    }]
}
