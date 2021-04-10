local sections = {}
local figures = {}
local figure_count = 1
local equations = {}
local eq_count = 1
local tables = {}
local table_count = 1

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
function populate_figures(fig)
  if fig.identifier and #fig.caption ~= 0 then
    figures[fig.identifier] = figure_count
    figure_count = figure_count + 1
  end
end

-- Populate equations table with ID and number, and transform into Div with ID containing numbered equation
function populate_equations(para)
  if para.content[1].t == "Math" and para.content[1].mathtype == "DisplayMath" then
    for i=#para.content,1,-1 do -- Iterating backwards to limit number of loops (ID often placed after the caption)
      el = para.content[i]
      if el.t == "Str" and el.text:match("{#eq:.*}") then
        id = el.text:gsub("{#", ""):gsub("}", "") -- Strip the prefix and brackets
        if eq_surround_paren then
          equations[id] = "(" .. eq_count .. ")"
        else
          equations[id] = eq_count
        end
        eq_count = eq_count + 1
        break
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
    for i=#caption,1,-1 do -- Iterating backwards to limit number of loops (ID often placed after the caption)
      if caption[i].t == "Str" and caption[j].text:match("^{#tbl:.*}") then
        id = caption[i].text:gsub("{#", ""):gsub("}", "") -- Strip the prefix and brackets (FIXME: Other attributes should perhaps be allowed...)
        tables[id] = table_count
        table.identifier = id
        table_count = table_count + 1
        -- Remove ID definition and the space before from caption
        caption:remove(i); if i > 2 then caption:remove(i-1) end
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
  if FORMAT == "latex" and (sec or fig or eq) then
    return pandoc.RawInline("latex", "\\ref{" .. id .. "}")
  elseif sec then
    return pandoc.Link({pandoc.Str(sections[id])}, '#' .. id, "", "")
  elseif fig then
    return pandoc.Link({pandoc.Str(figures[id])}, '#' .. id, "", "")
  elseif tbl then
    return pandoc.Link({pandoc.Str(tables[id])}, '#' .. id, "", "")
  elseif eq then
    return pandoc.Link({pandoc.Str(equations[id])}, '#' .. id, "", "")
  end
end

return {
  {Pandoc = populate_sections},
  {Image = populate_figures},
  {Para = populate_equations},
  {Table = populate_tables},
  {Cite = refs},
}
