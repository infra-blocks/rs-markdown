//! These tests are the examples taken from the [specification](https://spec.commonmark.org/0.31.2/).
use markdown::{ToHtml, parse};

// TODO: put all the tests.
macro_rules! test {
    ($name:ident, $markdown:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let document = parse($markdown);
            assert_eq!(document.to_html(), $expected);
        }
    };
}

mod atx_heading {
    use super::*;

    test!(
        example_62,
        r"# foo
## foo
### foo
#### foo
##### foo
###### foo",
        r"<h1>foo</h1>
<h2>foo</h2>
<h3>foo</h3>
<h4>foo</h4>
<h5>foo</h5>
<h6>foo</h6>"
    );
    test!(
        exmple_67,
        r"#                  foo                     ",
        r"<h1>foo</h1>"
    );
    test!(
        example_68,
        r" ### foo
  ## foo
   # foo",
        r"<h3>foo</h3>
<h2>foo</h2>
<h1>foo</h1>"
    );
    test!(
        example_71,
        r"## foo ##
  ###   bar    ###",
        r"<h2>foo</h2>
<h3>bar</h3>"
    );
    test!(
        example_72,
        r"# foo ##################################
##### foo ##",
        r"<h1>foo</h1>
<h5>foo</h5>"
    );
    test!(example_73, r"### foo ###     ", r"<h3>foo</h3>");
    test!(example_74, r"### foo ### b", r"<h3>foo ### b</h3>");
    test!(example_75, r"# foo#", r"<h1>foo#</h1>");

    /* test!(
        example_76,
        r"### foo \###
    ## foo #\##
    # foo \#",
        r"<h3>foo ###</h3>
    <h2>foo ###</h2>
    <h1>foo #</h1>"
    ); */

    test!(
        example_77,
        r"****
## foo
****",
        r"<hr />
<h2>foo</h2>
<hr />"
    );

    test!(
        exmple_79,
        r"##
#
### ###",
        r"<h2></h2>
<h1></h1>
<h3></h3>"
    );
}

/* mod blank_line {
    use super::*;

    test!(
        example_227,
        r"

aaa


# aaa

  ",
        r"<p>aaa</p>
<h1>aaa</h1>"
    );
} */

/* mod fenced_code {
    use super::*;

    test!(
        example_119,
        r"```
<
 >
```",
        r"<pre><code>&lt;
 &gt;
</code></pre>"
    );
    test!(
        example_120,
        r"~~~
<
 >
~~~",
        r"<pre><code>&lt;
 &gt;
</code></pre>"
    );
    test!(
        example_121,
        r"``
foo
``",
        r"<p><code>foo</code></p>"
    );
    test!(
        example_122,
        r"```
aaa
~~~
```",
        r"<pre><code>aaa
~~~
</code></pre>"
    );
    test!(
        example_123,
        r"~~~
aaa
```
~~~",
        r"<pre><code>aaa
```
</code></pre>"
    );
    test!(
        example_124,
        r"````
aaa
```
``````",
        r"<pre><code>aaa
```
</code></pre>"
    );
    test!(
        example_125,
        r"~~~~
aaa
~~~
~~~~",
        r"<pre><code>aaa
~~~
</code></pre>"
    );
    test!(example_126, r"```", r"<pre><code></code></pre>");
    test!(
        example_127,
        r"`````

```
aaa",
        r"<pre><code>
```
aaa
</code></pre>"
    );
}*/

mod html {
    use super::*;

    test!(
        example_150,
        r" <div>
  *hello*
         <foo><a>",
        r" <div>
  *hello*
         <foo><a>"
    );
    test!(
        example_151,
        r"</div>
*foo*",
        r"</div>
*foo*"
    );
    test!(
        example_153,
        r#"<div id="foo"
  class="bar">
</div>"#,
        r#"<div id="foo"
  class="bar">
</div>"#
    );
    test!(
        example_154,
        r#"<div id="foo" class="bar
  baz">
</div>"#,
        r#"<div id="foo" class="bar
  baz">
</div>"#
    );
    test!(
        example_156,
        r#"<div id="foo"
*hi*"#,
        r#"<div id="foo"
*hi*"#
    );
    test!(
        example_157,
        r#"<div class
foo"#,
        r#"<div class
foo"#
    );
    test!(
        example_158,
        r#"<div *???-&&&-<---
*foo*"#,
        r#"<div *???-&&&-<---
*foo*"#
    );
    test!(
        example_159,
        r#"<div><a href="bar">*foo*</a></div>"#,
        r#"<div><a href="bar">*foo*</a></div>"#
    );
    test!(
        example_160,
        r#"<table><tr><td>
foo
</td></tr></table>"#,
        r#"<table><tr><td>
foo
</td></tr></table>"#
    );
    test!(
        example_161,
        r#"<div></div>
``` c
int x = 33;
```"#,
        r#"<div></div>
``` c
int x = 33;
```"#
    );
    test!(
        example_162,
        r#"<a href="foo">
*bar*
</a>"#,
        r#"<a href="foo">
*bar*
</a>"#
    );
    test!(
        example_163,
        r#"<Warning>
*bar*
</Warning>"#,
        r#"<Warning>
*bar*
</Warning>"#
    );
    test!(
        example_164,
        r#"<i class="foo">
*bar*
</i>"#,
        r#"<i class="foo">
*bar*
</i>"#
    );
    test!(
        example_165,
        r#"</ins>
*bar*"#,
        r#"</ins>
*bar*"#
    );
    test!(
        example_166,
        r#"<del>
*foo*
</del>"#,
        r#"<del>
*foo*
</del>"#
    );
}

mod indented_code {
    use super::*;

    test!(
        example_107,
        r"    a simple
      indented code block
",
        r"<pre><code>a simple
  indented code block
</code></pre>"
    );
    test!(
        example_111,
        r"    chunk1

    chunk2



    chunk3
",
        r"<pre><code>chunk1

chunk2



chunk3
</code></pre>"
    );
    // Added terminating new line in input.
    test!(
        example_112,
        r"    chunk1
      
      chunk2
",
        r"<pre><code>chunk1
  
  chunk2
</code></pre>"
    );
    /* test!(
        example_115,
        r"# Heading
        foo
    Heading
    ------
        foo
    ----",
        r"<h1>Heading</h1>
    <pre><code>foo
    </code></pre>
    <h2>Heading</h2>
    <pre><code>foo
    </code></pre>
    <hr />"
    ); */
    // Added terminating new line in input.
    test!(
        example_116,
        r"        foo
    bar
",
        r"<pre><code>    foo
bar
</code></pre>"
    );
    test!(
        example_117,
        r"
    
    foo
    ",
        r"<pre><code>foo
</code></pre>"
    );
    // Added terminating new line in input.
    test!(
        example_118,
        r"    foo  
",
        r"<pre><code>foo  
</code></pre>"
    );
}

mod thematic_break {
    use super::*;

    test!(
        example_43,
        r"***
---
___",
        r"<hr />
<hr />
<hr />"
    );
    test!(
        example_47,
        r" ***
  ***
   ***",
        r"<hr />
<hr />
<hr />"
    );
    test!(
        example_50,
        r"_____________________________________",
        r"<hr />"
    );
    test!(example_51, r" - - -", r"<hr />");
    test!(example_52, r" **  * ** * ** * **", r"<hr />");
    test!(example_53, r"-     -      -      -", r"<hr />");
    test!(example_54, r"- - - -    ", r"<hr />");
}
