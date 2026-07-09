pub enum TOK {
    FirstBlock, // %{...%}
    Name, // DIGIT
    Pattern, // [0-9] or {DIGIT}+
    SectionSep, // %%
    Action, // { printf(...); }
    StartCond, // %s / %x decla
    EOF
}

pub struct Token {
   typ: TOK,
   s: String,
   line: usize,
   column: usize,
}
