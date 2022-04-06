local sidenote_count = 0

function Note(fn)
  if FORMAT == "html" then
    -- Only convert footnotes with one block into sidenotes
    if not (#fn.content > 1) then
      sidenote_count = sidenote_count + 1
      local inlines = {}

      table.insert(inlines, pandoc.RawInline(
        "html",
        [[<label for="sn-]] ..
        sidenote_count ..
        [[" class="sidenote-toggle sidenote-number"></label><input type="checkbox" id="sn-]] ..
        sidenote_count ..
        [[" class="sidenote-toggle" /><span class="sidenote">]]
      ))
      for _, inline in ipairs(pandoc.utils.blocks_to_inlines(fn.content)) do
        table.insert(inlines, inline)
      end
      table.insert(inlines, pandoc.RawInline("html", "</span>"))

      return inlines
    end
  end
end
