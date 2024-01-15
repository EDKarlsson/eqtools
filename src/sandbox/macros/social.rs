/*
[Socials]
Page2Button1Name=Follow Tank.
Page2Button1Color=18
Page2Button1Line1=/follow Aaden

Page2Button2Name=Assist Cast 1
Page2Button2Color=13
Page2Button2Line1=/pause 5, /assist Aaden
Page2Button2Line2=/pause 10, /cast 3

Page2Button3Name=Assist Cast 2
Page2Button3Color=13
Page2Button3Line1=/pause 5, /assist Aaden
Page2Button3Line2=/pause 10, /cast 3
 */

pub struct SocialMacro {
    name: String,
    color: i16,
    lines: [String; 5],
}

impl SocialMacro {
    pub fn new(name: &String, color: i16, lines: [String; 5]) -> SocialMacro {
        return SocialMacro {
            name: name.parse().unwrap(),
            color,
            lines,
        };
    }
}