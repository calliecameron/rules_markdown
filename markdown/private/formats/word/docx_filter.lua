-- Customise horizontal rules in docx to '* * *'
-- from https://gist.github.com/Merovex/05e3216f8f4f6e965cd9d564b1496719
local rule = [[<w:p>
  <w:pPr>
    <w:pStyle w:val="HorizontalRule"/>
      <w:ind w:firstLine="0"/>
      <w:jc w:val="center"/>
  </w:pPr>
  <w:r>
    <w:t>* * *</w:t>
  </w:r>
</w:p>]]

function HorizontalRule(_) -- luacheck: ignore 131
    return pandoc.RawBlock("openxml", rule)
end
