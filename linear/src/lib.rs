//! Linear issue tracking integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::linear AS f"bucket:/linear.surli";`
//! and call `mod::linear::create_issue(...)` and `mod::linear::get_issue(...)`.
//! Every request POSTs a GraphQL query to `https://api.linear.app/graphql`
//! with an `Authorization` header set to the raw API key (no `Bearer` prefix).

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_graphql(api_key: &str, query: &str, variables: Value) -> Result<Value> {
	surrealism::run(
		"http::post".to_string(),
		None,
		(
			"https://api.linear.app/graphql".to_string(),
			json!({ "query": query, "variables": variables }),
			json!({ "Authorization": api_key }),
		),
	)
	.context("Call to host 'http::post' failed")
}

/// Creates an issue on a team.
#[surrealism]
fn create_issue(
	api_key: String,
	team_id: String,
	title: String,
	description: String,
) -> Result<Value, String> {
	post_graphql(
		&api_key,
		"mutation($teamId: String!, $title: String!, $description: String!) { issueCreate(input: { teamId: $teamId, title: $title, description: $description }) { success issue { id identifier url } } }",
		json!({ "teamId": team_id, "title": title, "description": description }),
	)
	.map_err(|e| e.to_string())
}

/// Fetches an issue by id.
#[surrealism]
fn get_issue(api_key: String, issue_id: String) -> Result<Value, String> {
	post_graphql(
		&api_key,
		"query($id: String!) { issue(id: $id) { id identifier title url state { name } } }",
		json!({ "id": issue_id }),
	)
	.map_err(|e| e.to_string())
}
