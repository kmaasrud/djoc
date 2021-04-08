local sections = {}

function find_sections(doc)
  function populate(elements)
    for _, el in pairs(elements) do
      if el.t == 'Div' and el.attributes.number then
        sections[el.attr.identifier] = el.attributes.number
        populate(el.content)
      end
    end
  end

  populate(pandoc.utils.make_sections(true, nil, doc.blocks))
end

function resolve_section_ref(cite)
  id = cite.citations[1].id:match("^sec:.*")
  if id then
    link_text = pandoc.Str(sections[id])
    return pandoc.Link({link_text}, '#' .. id, "", "")
  end
  return nil
end

return {
  {Pandoc = find_sections},
  {Cite = resolve_section_ref}
}
