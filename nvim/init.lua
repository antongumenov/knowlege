vim.opt.number = true
vim.opt.mouse = 'a'
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.hlsearch = false
vim.opt.wrap = true
vim.opt.breakindent = true
vim.opt.tabstop = 4
vim.opt.shiftwidth = 4
vim.opt.expandtab = false
vim.opt.cursorline = true
vim.g.mapleader = ' '


-- vim.keymap.set({mode}, {lhs}, {rhs}, {opts})
-- {mode}
--n: Normal mode.
--i: Insert mode.
--x: Visual mode.
--s: Selection mode.
--v: Visual + Selection.
--t: Terminal mode.
--o: Operator-pending.
--'': Yes, an empty string. Is the equivalent of n + v + o.
vim.keymap.set('n', '<space>w', '<cmd>write<cr>', {desc = 'Save'})


