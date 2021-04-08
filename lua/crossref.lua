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
        equations[id] = eq_count
        eq_count = eq_count + 1
      end
    end
    if id then
      if FORMAT == "latex" then
        return pandoc.RawBlock("latex", "\\begin{equation}\n" .. para.content[1].text .. "\n\\label{" .. id .. "}\n\\end{equation}")
      else
        -- I'm mostly interested in HTML, in which this is the same behavior as pandoc-crossref.
        -- For some other formats, there is probably a better solution than this
        return pandoc.Para(pandoc.Span(pandoc.Math("DisplayMath", para.content[1].text .. "\\qquad\\text{(" .. eq_count .. ")}"), {id = id}))
      end
    end
  end
end


function refs(cite)
  id = cite.citations[1].id
  if id:match("^sec:.*") then
    link_text = pandoc.Str(sections[id])
  elseif id:match("^fig:.*") then
    link_text = pandoc.Str(figures[id])
  elseif id:match("^eq:.*") then
    if FORMAT == "latex" then
      return pandoc.RawInline("latex", "\\ref{" .. id .. "}")
    end
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
  {Cite = refs},
}
