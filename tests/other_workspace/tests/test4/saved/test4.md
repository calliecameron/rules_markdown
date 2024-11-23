---
author:
- The Author
date: 1 January 2022
lang: en-GB
source-hash: eabaf7703065d2a44ba59d5fd9fc06a5cf8d455a21cd9395c960c68916375315
title: Test 4
version: reproducible
---

-   [The Title](#__h1_1){#toc-__h1_1}
-   [The Title](#__h1_2){#toc-__h1_2}
-   [The Title Is ‘Baz’](#__h1_3){#toc-__h1_3}
-   [Title](#__h1_4){#toc-__h1_4}

# The Title {#__h1_1}

This is some text before a section. It shouldn’t be indented.

## This is a section {#__h2_1}

This is some test text. This is formatted in *italics* and **bold**, with - various – dashes—, and trailing dots…

This is a bullet list:

-   This is the first paragraph of the first item.

    And the second paragraph of the first item.

-   The second item only has one paragraph.

This is a numbered list:

1.  This is the first paragraph of the first item.

    And the second paragraph of the first item.

2.  The second item only has one paragraph.

‘These quotes should be curly,’ and “so should these.” There should be a blank line before the next paragraph:

 

And then there should be some text ^in superscript^ and ~in subscript~, and a footnote[^1] with a star, a footnote[^2] with a dagger, and this should be `monospace`.

### Subsection {#__h3_1}

Test text test text test text.

> This is a quote block. It should be indented slightly and shouldn’t contain a line break.
>
> This is a second paragraph in the same quote block.

> | This is a quoted line block. It should be indented slightly
> | and have a *line break* after ‘slightly’, and **formatting**.

> | “These literal double curly quotes, used where smart
> | quotes gets it wrong, curl the right way even though
> | they’re on different lines.”

> | ‘These literal single curly quotes, used where smart
> | quotes gets it wrong, curl the right way even though
> | they’re on different lines.’

After this line there should be stars.

------------------------------------------------------------------------

This is a new paragraph after the stars. This text is [Small Caps]{.smallcaps}. Here is a pound sign (£), a euro sign (€), and three letters with accents: ëóû.

# The Title {#__h1_2}

**An Author**

This is some text before a section. It shouldn’t be indented. Each section should start on a new page (but subsections shouldn’t).

## This is a section {#__h2_2}

This is some test text. This is formatted in *italics* and **bold**, with - various – dashes—, and trailing dots…

‘These quotes should be curly,’ and “so should these.” There should be a blank line before the next paragraph:

 

And then there should be some text ^in superscript^ and ~in subscript~, and a footnote[^3] with a star, a footnote[^4] with a dagger, and this should be `monospace`.

### Subsection {#__h3_2}

Test text test text test text.

> This is a quote block. It should be indented slightly and shouldn’t contain a line break.

> | This is a quoted line block. It should be indented slightly
> | and have a *line break* after ‘slightly’, and **formatting**.

> | “These literal double curly quotes, used where smart
> | quotes gets it wrong, curl the right way even though
> | they’re on different lines.”

> | ‘These literal single curly quotes, used where smart
> | quotes gets it wrong, curl the right way even though
> | they’re on different lines.’

After this line there should be stars.

------------------------------------------------------------------------

This is a new paragraph after the stars. This text is [Small Caps]{.smallcaps}. Here is a pound sign (£), a euro sign (€), and three letters with accents: ëóû.

## This is a second section {#__h2_3}

![foo](tests/test2/image.jpg "bar"){.quux width="50%"}

There is an image here.

And this is *even more italic text*.

# The Title Is ‘Baz’ {#__h1_3}

**An Author, 23 February 2019 baz**

## This is a section {#__h2_4}

This is some test text. This is formatted in *italics* and **bold**, with - various – dashes—, and trailing dots…

‘These quotes should be curly,’ and “so should these.” There should be a blank line before the next paragraph:

 

And then we do a simple include:

Text before a section in simple include.

## Section in simple include {#__h2_5}

> This is a quote block. It should be indented slightly and shouldn’t contain a line break.

> | This is a quoted line block. It should be indented slightly
> | and have a *line break* after ‘slightly’, and **formatting**.

Text before recursive include, with *italic*, **bold**, “curly quotes,” and— an em dash.

Text before section in recursive include.

## Section in recursive include {#__h2_6}

Text in recursive include, with *italic*, **bold**, “curly quotes,” and— an em dash.

### Subsection in recursive include {#__h3_3}

> | “These literal double curly quotes, used where smart
> | quotes gets it wrong, curl the right way even though
> | they’re on different lines.”

> | ‘These literal single curly quotes, used where smart
> | quotes gets it wrong, curl the right way even though
> | they’re on different lines.’

Test text test text test text. After this line there should be stars.

------------------------------------------------------------------------

And then there should be some text ^in superscript^ and ~in subscript~, and a footnote[^5] with a star, a footnote[^6] with a dagger, and this should be `monospace`.

Text after recursive include. Here is a pound sign (£), a euro sign (€), and three letters with accents: ëóû.

## Second section in simple include {#__h2_7}

Test text.

Text after simple include.

### Subsection {#__h3_4}

This is a new paragraph. This text is [Small Caps]{.smallcaps}.

## This is a second section {#quux-not-spellchecked}

And this is *even more italic text*. Foo.

``` python
# Code blocks aren't spellchecked: quux
```

Inline code isn’t spellchecked: `quux`.

::: quux
`div` classes aren’t spellchecked.
:::

Anything in a nospellcheck div isn’t spellchecked: quux.

Automatic links aren’t spellchecked: <http://quux.com>. Neither are the targets or attributes of inline links: [Foo](http://quux.com "Foo"){.quux}. Neither are [span classes]{.quux}. Anything in a nospellcheck span isn’t spellchecked.

# Title {#__h1_4}

**4 January 2022**

This is text before a section. It shouldn’t be indented.

[^1]: This is a footnote. It should appear at the bottom of the page.

[^2]: Another footnote.

[^3]: This is a footnote. It should appear at the bottom of the page.

[^4]: Another footnote.

[^5]: This is a footnote. It should appear at the bottom of the page.

[^6]: Another footnote.
