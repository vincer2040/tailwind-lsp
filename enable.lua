function restart_tailwind()
    require("lsp-debug-tools").restart({
        expected = {},
        name = "tailwind-rs",
        cmd = { "./target/debug/tailwind-lsp" },
        root_dir = vim.loop.cwd(),
    });
end
