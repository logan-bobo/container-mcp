use bollard::{image::ListImagesOptions, Docker};
use rmcp::{
    handler::server::tool::ToolRouter, model::*, tool, tool_handler, tool_router,
    Error as McpError, ServerHandler,
};
use std::future::Future;

#[derive(Clone)]
pub struct DockerRouter {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl DockerRouter {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Run a docker container")]
    pub async fn run_container(&self) -> Result<CallToolResult, McpError> {
        let docker = Docker::connect_with_local_defaults().unwrap();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Container Running"
        ))]))
    }

    #[tool(description = "List all container images")]
    pub async fn list_images(&self) -> Result<CallToolResult, McpError> {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let images = &docker
            .list_images(Some(ListImagesOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await
            .unwrap();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Images: {:?}",
            images
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for DockerRouter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides the capabilities to manage docker containers".to_string(),
            ),
        }
    }
}
