-- to use system clipboard install wl-clipboard
vim.opt.clipboard = "unnamedplus"
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.shiftwidth = 4
vim.opt.tabstop = 4
vim.opt.smartindent = true
vim.opt.number = true
vim.opt.wrap = true
vim.opt.relativenumber = true

-- no overwrite register on actions
vim.keymap.set('v', 'p', 'P', { noremap = true })

vim.keymap.set('n','cw', '"_cw',{noremap = true}) 
vim.keymap.set('n','cc', '"_cc',{noremap = true}) 
vim.keymap.set('n','C', '"_C',{noremap = true}) 
vim.keymap.set('v','c', '"_c',{noremap = true}) 
vim.keymap.set('v','ci{', '"_ci{',{noremap = true}) 
vim.keymap.set('v','ca{', '"_ca{',{noremap = true}) 

vim.keymap.set('v','ci"', '"_ci"',{noremap = true}) 
vim.keymap.set('v','ca"', '"_ca"',{noremap = true}) 

vim.keymap.set('v','ci[', '"_ci[',{noremap = true}) 
vim.keymap.set('v','ca[', '"_ca[',{noremap = true}) 

vim.keymap.set('v','ci(', '"_ci(',{noremap = true}) 
vim.keymap.set('v','ca(', '"_ca(',{noremap = true}) 

vim.keymap.set('n','dw', '"_dw',{noremap = true}) 
vim.keymap.set('n','dd', '"_dd',{noremap = true}) 
vim.keymap.set('n','D', '"_D',{noremap = true}) 
vim.keymap.set('v','d', '"_d',{noremap = true}) 
vim.keymap.set('v','di{', '"_di{',{noremap = true}) 
vim.keymap.set('v','da{', '"_da{',{noremap = true}) 

vim.keymap.set('v','di"', '"_di"',{noremap = true}) 
vim.keymap.set('v','da"', '"_da"',{noremap = true}) 

vim.keymap.set('v','di[', '"_di[',{noremap = true}) 
vim.keymap.set('v','da[', '"_da[',{noremap = true}) 

vim.keymap.set('v','di(', '"_di(',{noremap = true}) 
vim.keymap.set('v','da(', '"_da(',{noremap = true}) 

vim.keymap.set('v','r', '"_r',{noremap = true}) 
vim.keymap.set('n','r', '"_r',{noremap = true}) 
vim.keymap.set('v','R', '"_R',{noremap = true}) 
vim.keymap.set('n','R', '"_R',{noremap = true}) 

vim.keymap.set('v','s', '<cmd>write<cr>', {desc = 'Save'}) 
vim.keymap.set('v','S', '<cmd>write<cr>', {desc = 'Save'}) 
vim.keymap.set('n','s', '<cmd>write<cr>', {desc = 'Save'}) 
vim.keymap.set('n','S', '<cmd>write<cr>', {desc = 'Save'}) 
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


