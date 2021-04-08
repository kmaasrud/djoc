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

local sections = {}
local figures = {}
local figure_count = 1
local equations = {}
local eq_count = 1
local tables = {}
local table_count = 1

-- Whether to put parentheses around equation numbers or not.
-- This should be configured as a metadata thing, but hardcoding for now
local eq_surround_paren = false

-- Poplate sections table with ID and number
function populate_sections(doc)
  function populate(elements)
    for _, el in pairs(elements) do
      if el.t == "Div" and el.attributes.number then
        sections[el.attr.identifier] = el.attributes.number
        populate(el.content)
      end
    end
  end
  populate(pandoc.utils.make_sections(true, nil, doc.blocks))
end

-- Populate figures table with ID and number
function populate_figures(para)
  if para.content[1].t == "Image" then
    fig = para.content[1]
    if fig.identifier and #fig.caption ~= 0 then
      figures[fig.identifier] = figure_count
      figure_count = figure_count + 1
    end
  end
end

-- Populate equations table with ID and number, and transform into Div with ID containing numbered equation
function populate_equations(para)
  if para.content[1].t == "Math" and para.content[1].mathtype == "DisplayMath" then
    for _, v in pairs(para.content) do
      if v.t == "Str" and v.text:match("{#eq:.*}") then
        id = v.text:gsub("{#", ""):gsub("}", "") -- Strip the prefix and brackets
        if eq_surround_paren then
          equations[id] = "(" .. eq_count .. ")"
        else
          equations[id] = eq_count
        end
        eq_count = eq_count + 1
      end
    end
    if id then
      if FORMAT == "latex" then
        return pandoc.RawBlock("latex", "\\begin{equation}\n" .. para.content[1].text .. "\n\\label{" .. id .. "}\n\\end{equation}")
      else
        -- I'm mostly interested in HTML, in which this is the same behavior as pandoc-crossref.
        -- For other formats, there is probably a better solution than using \qquad
        return pandoc.Para(pandoc.Span(pandoc.Math("DisplayMath", para.content[1].text .. "\\qquad\\text{(" .. eq_count .. ")}"), {id = id}))
      end
    end
  end
end


function populate_tables(table)
  if table.caption then
    -- Caption might contain several blocks, extract ID from last block
    caption = table.caption.long[#table.caption.long].content
    for i=1,#caption do
      j = #caption - i + 1 -- This is why indices start on 0...
      if caption[j].t == "Str" and caption[j].text:match("^{#tbl:.*}") then
        id = caption[j].text:gsub("{#", ""):gsub("}", "") -- Strip the prefix and brackets
        tables[id] = table_count
        table_count = table_count + 1
        caption:remove(j); caption:remove(j-1) -- Remove ID definition and the space before
        break
      end
    end
    if id then
      return pandoc.Div(table, {id = id})
    end
  end
end


function refs(cite)
  id = cite.citations[1].id
  sec = id:match("^sec:.*"); fig = id:match("^fig:.*"); tbl = id:match("^tbl:.*"); eq = id:match("^eq:.*")
  if FORMAT == "latex" and sec and fig and tbl and eq then
    return pandoc.RawInline("latex", "\\autoref{" .. id .. "}")
  elseif sec then
    link_text = pandoc.Str(sections[id])
  elseif fig then
    link_text = pandoc.Str(figures[id])
  elseif tbl then
    link_text = pandoc.Str(tables[id])
  elseif eq then
    link_text = pandoc.Str(equations[id])
  else
    return nil
  end
  return pandoc.Link({link_text}, '#' .. id, "", "")
end

return {
  {Pandoc = populate_sections},
  {Para = populate_figures},
  {Para = populate_equations},
  {Table = populate_tables},
  {Cite = refs},
}
