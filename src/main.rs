//#![feature(collections)]
extern crate docopt;

use std::char;
use std::string::String;

use std::io::BufRead;

use std::collections::HashMap;

use docopt::Docopt;


static DICC256 : &'static str = "\
🌀🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏😐😑😒😓😔😕😖😗😘😙😚\
😛😜😝😞😟😠😡😢😣😤😥😦😧😨😩😪😫😬😭😮😯😰😱😲😳😴😵😶😷😸😹😺😻😼😽😾😿🙀🙅🙆🙇🙈🙉\
🙊🙋🙌🙍🙎🙏🚀🚁🚂🚃🚄🚅🚆🚊🚍🚑🚒🚕🚖🚗🚘🚙🚜🚝🚞🚟🚠🚡🚣🚥🚧🚴🚵🚶🚸🚹🚺🚻🚽🚿🛀🛁⚪\
⚫⚽⛄⛅⛔⛪⛳⛵🌝🌞🌟🌲🌳🌴🌵🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁🍂🍃🍄🍅🍆🍇🍈🍉🍊🍋🍌🍍🍎🍏🍐🍑🍒\
🍓🍔🍕🍖🍗🍘🍙🍚🍛🍜🍝🍞🍟🍠🍡🍢🍣🍤🍥🍦🍧🍨🍩🍪🍫🍬🍭🍮🍯🍰🍱🍲🍳🍴🍵🍶🍷🍸🍹🍺🍻🍼🎀\
🎁👍👎👏👐👑👒👓👔👕👖👗👘👙👚👛👜👝👞👟👠👡👢👤👥👦👧👨👩👪👫👬👭👮👯👰👱👲👳👴👵";

//XXX: use vecMap when stable
fn build_emoji_map() -> (HashMap<u32,u8>, HashMap<u8,u32>) {
    let mut emoji_to_byte = HashMap::new();
    let mut byte_to_emoji = HashMap::new();
    let mut byte : u8 = 0;
    for ch in DICC256.chars() {
        let emoji = ch as u32;
        //println!("{} {} {}", byte, &emoji, &ch);
        emoji_to_byte.insert(emoji, byte);
        byte_to_emoji.insert(byte, emoji);
        if byte<255 {
            byte = byte + 1;
        }
    }
    return (emoji_to_byte, byte_to_emoji);
}

fn encode(map : &HashMap<u8,u32>, bytes :&[u8]) -> String {
    let mut st = String::new();
    for byte in bytes {
        match map.get(byte) {
            Some(value) => st.push(char::from_u32(*value).unwrap()),
            None        => st.push('?')
        }
    }
    return st;
}

fn decode(map :&HashMap<u32,u8>, st :&String) -> Vec<u8> {
    let mut res = vec!();
    for ch in st.chars() {
        let no = ch as u32;
        match map.get(&no) {
            Some(value) => res.push(*value),
            None => ()
        }
    }
    return res;
}

fn encode_stdin_stdout(byte_to_emoji : &HashMap<u8,u32>) {
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    loop {
      let mut consumed : usize = 0;
      match reader.fill_buf() {
          Ok(buf) => {
                 print!("{}", &encode(&byte_to_emoji, buf) );
                 consumed = buf.len();
                }
         Err(e)  => println!("error {:?}", e)
     }
     if consumed == 0 { break; };
     reader.consume(consumed);
    }
}

fn decode_stdin_stdout(emoji_to_byte :&HashMap<u32,u8>) {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => print!("{}", String::from_utf8_lossy(&decode(emoji_to_byte, &line))),
            Err(e)   => println!("error {:?}", e)
        }
    }

}

static USAGE: &'static str = "
Usage: basemoji [options]
Emoji encode or decode standard input to standard output 😍

Options: -d    decode basemoji
         -e    encode basemoji
";
/* SOON
    -i    input file (default is stdin)
    -o    output file (default is stdout)
";
*/

fn main(){

    let args = Docopt::new(USAGE).unwrap().parse()
                  .unwrap_or_else(|e| e.exit());

    let (emoji_to_byte, byte_to_emoji) = build_emoji_map();
    if args.get_bool(&"-e") {
        encode_stdin_stdout(&byte_to_emoji);
    } else if args.get_bool(&"-d") {
        decode_stdin_stdout(&emoji_to_byte);
    } else {
        println!("{}", &USAGE);
    }

}
