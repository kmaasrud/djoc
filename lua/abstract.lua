--[[
abstract-to-meta – move an "abstract" section into document metadata
Copyright: © 2017–2021 Albert Krewinkel
License:   MIT – see LICENSE file for details
]]
local abstract = {}

--- Extract abstract from a list of blocks.
function abstract_from_blocklist (blocks)
  local body_blocks = {}
  local looking_at_abstract = false

  for _, block in ipairs(blocks) do
    if block.t == 'Header' and block.level == 1 then
      if block.identifier == 'abstract' then
        looking_at_abstract = true
      else
        looking_at_abstract = false
        body_blocks[#body_blocks + 1] = block
      end
    elseif looking_at_abstract then
      if block.t == 'HorizontalRule' then
        looking_at_abstract = false
      else
        abstract[#abstract + 1] = block
      end
    else
      body_blocks[#body_blocks + 1] = block
    end
  end

  return body_blocks
end

return {{
  Blocks = abstract_from_blocklist,
  Meta = function (meta)
    if not meta.abstract and #abstract > 0 then
      meta.abstract = pandoc.MetaBlocks(abstract)
    end
    return meta
  end
}}
