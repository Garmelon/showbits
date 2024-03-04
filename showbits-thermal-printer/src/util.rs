const CODE_PAGE_437_SECOND_HALF: &str = concat!(
    "ÇüéâäàåçêëèïîìÄÅ",
    "ÉæÆôöòûùÿÖÜ¢£¥₧ƒ",
    "áíóúñÑªº¿⌐¬½¼¡«»",
    "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐",
    "└┴┬├─┼╞╟╚╔╩╦╠═╬╧",
    "╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀",
    "αßΓπΣσµτΦΘΩδ∞φε∩",
    "≡±≥≤⌠⌡÷≈°∙·√ⁿ²■",
);

fn is_safe(c: char) -> bool {
    c.is_ascii_graphic() || c == ' ' || c == '\n' || CODE_PAGE_437_SECOND_HALF.contains(c)
}

pub fn sanitize(text: &str) -> String {
    text.chars().filter(|&c| is_safe(c)).collect::<String>()
}
