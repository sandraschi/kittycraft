# craft.config.psd1 - per-repo settings for start.ps1 and scripts\release.ps1.
# Standard: mcp-central-docs\standards\rust-artcraft\CRAFT_START_AND_RELEASE.md
@{
    Name        = 'kittycraft'
    Display     = 'KittyCraft'
    AppPackage  = 'kittycraft-app'
    AppBin      = 'kittycraft-app'
    AppSmoke    = @()
    CliPackage  = 'kittycraft-mcp'
    CliBin      = 'kittycraft-mcp'
    CliSmoke    = @('status', '--json')
    McpArgs     = @('mcp')
    McpSmoke    = @('mcp')
    Installer   = $false
    Mcpb        = $false
}
