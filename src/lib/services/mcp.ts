import { invoke } from "@tauri-apps/api/core";

export type McpClient = "cursor" | "codex" | "claude";

export const MCP_CLIENTS: { id: McpClient; label: string; hint: string }[] = [
  {
    id: "cursor",
    label: "Cursor",
    hint: "Paste into ~/.cursor/mcp.json, then reload MCP.",
  },
  {
    id: "codex",
    label: "Codex",
    hint: "Paste into ~/.codex/config.toml.",
  },
  {
    id: "claude",
    label: "Claude Desktop",
    hint: "Paste into Claude Desktop → Settings → Developer → Edit Config.",
  },
];

export function getMcpBinaryPath(): Promise<string> {
  return invoke<string>("get_mcp_binary_path");
}

export function formatMcpSnippet(client: McpClient, command: string): string {
  switch (client) {
    case "cursor":
    case "claude":
      return `${JSON.stringify({ mcpServers: { "time-tracker": { command } } }, null, 2)}\n`;
    case "codex":
      return `[mcp_servers.time-tracker]\ncommand = ${tomlString(command)}\n`;
  }
}

function tomlString(value: string): string {
  return `"${value.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
}
