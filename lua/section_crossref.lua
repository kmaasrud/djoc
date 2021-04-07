local function dump(o)
   if type(o) == 'table' then
      local s = '{ '
      for k,v in pairs(o) do
         if type(k) ~= 'number' then k = '"'..k..'"' end
         s = s .. '['..k..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
end

local make_sections = (require 'pandoc.utils').make_sections
local section_numbers = {}

function populate_section_numbers(doc)
  function populate (elements)
    for _, el in pairs(elements) do
      if el.t == 'Div' and el.attributes.number then
        section_numbers['#' .. el.attr.identifier] = el.attributes.number
        populate(el.content)
      end
    end
  end

  populate(make_sections(true, nil, doc.blocks))
end

function resolve_section_ref(link)
  if table.unpack(link.content)["text"] ~= "*" or link.target:sub(1, 1) ~= '#' then
    return nil
  end
  local section_number = pandoc.Str(section_numbers[link.target])
  return pandoc.Link({section_number}, link.target, link.title, link.attr)
end

return {
  {Pandoc = populate_section_numbers},
  {Link = resolve_section_ref}
}
