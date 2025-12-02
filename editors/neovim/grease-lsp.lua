-- Grease language configuration for Neovim LSP
-- Add this to your Neovim configuration (e.g., init.lua)

local lspconfig = require('lspconfig')

-- Grease LSP configuration
lspconfig.grease.setup {
  cmd = { 'grease', 'lsp' },
  filetypes = { 'grease' },
  root_dir = lspconfig.util.root_pattern('.git', vim.fn.getcwd()),
  single_file_support = true,
  settings = {},
  on_attach = function(client, bufnr)
    -- Standard LSP keybindings
    local opts = { noremap = true, silent = true, buffer = bufnr }
    
    -- Navigation
    vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, opts)
    vim.keymap.set('n', 'gd', vim.lsp.buf.definition, opts)
    vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts)
    vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, opts)
    vim.keymap.set('n', '<C-k>', vim.lsp.buf.signature_help, opts)
    vim.keymap.set('n', '<space>wa', vim.lsp.buf.add_workspace_folder, opts)
    vim.keymap.set('n', '<space>wr', vim.lsp.buf.remove_workspace_folder, opts)
    vim.keymap.set('n', '<space>wl', function()
      print(vim.inspect(vim.lsp.buf.list_workspace_folders()))
    end, opts)
    vim.keymap.set('n', '<space>D', vim.lsp.buf.type_definition, opts)
    vim.keymap.set('n', '<space>rn', vim.lsp.buf.rename, opts)
    vim.keymap.set('n', '<space>ca', vim.lsp.buf.code_action, opts)
    vim.keymap.set('n', 'gr', vim.lsp.buf.references, opts)
    vim.keymap.set('n', '<space>e', vim.diagnostic.open_float, opts)
    vim.keymap.set('n', '[d', vim.diagnostic.goto_prev, opts)
    vim.keymap.set('n', ']d', vim.diagnostic.goto_next, opts)
    vim.keymap.set('n', '<space>q', vim.diagnostic.setloclist, opts)
    vim.keymap.set('n', '<space>f', function() vim.lsp.buf.format { async = true } end, opts)
  end,
}

-- File type detection
vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, {
  pattern = { "*.grease" },
  callback = function()
    vim.bo.filetype = 'grease'
  end,
})

-- Syntax highlighting (basic)
vim.api.nvim_create_autocmd("FileType", {
  pattern = "grease",
  callback = function()
    -- Basic syntax highlighting
    vim.cmd([[
      syntax match greaseComment /\v#.*/
      syntax match greaseKeyword /\v<(def|if|elif|else|while|for|in|return|use|as)>/
      syntax match greaseLogical /\v<(and|or|not)>/
      syntax match greaseBoolean /\v<(true|false)>/
      syntax match greaseNull /\v<null>/
      syntax match greaseString /\v"([^"]|\\")*"/
      syntax match greaseString /\v'([^']|\\')*'/
      syntax match greaseNumber /\v<[0-9]+(\.[0-9]+)?>/
      syntax match greaseOperator /\v[+\-*/%=<>!]=?/
      syntax match greaseOperator /\v[+\-*/%<>]/
      syntax match greasePunctuation /\v[,:.]/
      
      highlight link greaseComment Comment
      highlight link greaseKeyword Keyword
      highlight link greaseLogical Keyword
      highlight link greaseBoolean Boolean
      highlight link greaseNull Constant
      highlight link greaseString String
      highlight link greaseNumber Number
      highlight link greaseOperator Operator
      highlight link greasePunctuation Delimiter
    ]])
  end,
})