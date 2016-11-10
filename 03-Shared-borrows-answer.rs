pub fn main() {
    let string = format!("my friend");
    greet(&string);
    greet(&string);
}

fn greet(name: &String) {
    println!("Hello, {}!", &name[3..]);
}

// Goal #1: Convert `greet` to use borrowing, not ownership, so that
// this program executes without doing any cloning.
//
// Ans: `greet(&string)` and change the greet parameter type to `&String`
//
// Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// "Hello, my friend".
//
// Ans: `&name[3..]`. Notice that you need to add the `&` instead of just `name[3..]`
//
/*
shinglyu> Shing Lyu Hi there, Why do I need to write `&dog[0..5]` instead of `dog[0..5] in this example: https://doc.rust-lang.org/1.5.0/book/strings.html#slicing
6:01 PM ⇐ rschiffl1n quit (randy@moz-ipg5t8.g6pa.kjt5.8801.2600.IP) Ping timeout: 121 seconds
6:02 PM
<bluss> shinglyu: It's in line with the single element case for a slice. if cat[0] is a T, then &cat[0] is a &T
6:02 PM
<Quxxy2> shinglyu: Because the result of `dog[0..5]` is of type `str`, which is unsized, and you can't have values of that type
6:02 PM → LLBlumire1 joined (Thunderbird@moz-hlf.2ld.175.109.IP)
6:02 PM
<bluss> shinglyu: now that's extended to the str type
6:02 PM
<Quxxy2> &str is (a pointer to the start of the string, length of string)
6:03 PM ⇐ eibwen quit (eibwen@moz-a9r1hc.wlan.hu-berlin.de) Ping timeout: 121 seconds
6:03 PM
<Quxxy2> So str is what a *str points to, which is... some bytes floating around in memory of unknown size
6:03 PM The compiler *really* doesn't like not knowing the size of values
6:03 PM It makes copying them kinda hard :)
6:04 PM Sorry: "So str is what a &str points to, ..."
6:04 PM
<shinglyu> Shing Lyu bluss: Quxxy2: &str it self is sized, because it's the size of the memory address, right?
6:04 PM
<Quxxy2> Well, it's the size of (pointer, usize), but yes
6:04 PM ⇐ LLBlumire quit  •  LLBlumire1 → LLBlumire
6:04 PM
<shinglyu> Shing Lyu bluss: Quxxy2: Ya, that what I'm trying to say
6:04 PM
<Havvy> playbot: ::std::mem::sizeof::<&str>()
6:04 PM
<•playbot> error: `sizeof` is a reserved keyword
6:04 PM   --> <anon>:10:21
6:04 PM    |
6:04 PM (output truncated; full output at http://bit.ly/2euVPIv)
6:04 PM → junqed joined (junqed@moz-3ia.d39.70.178.IP)
6:05 PM
<Quxxy2> Heh :P
6:05 PM playbot: ::std::mem::size_of::<str>()
6:05 PM
<•playbot> error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
6:05 PM   --> <anon>:10:9
6:05 PM    |
6:05 PM (output truncated; full output at http://bit.ly/2dXW82d)
6:05 PM
<shinglyu> Shing Lyu But then, how is the size of the str determined then?
6:05 PM → shellac joined (pldms@moz-lngnj4.bris.ac.uk)
6:05 PM
<Quxxy2> It's part of the &str
6:06 PM Here's a way to look at it:
6:06 PM ⇐ mvyskocil and lenwood quit
6:06 PM
<Quxxy2> There are no `str`s
6:06 PM Actually, let's change slightly and talk about [u8], which is *almost* the same thing
6:06 PM
<shinglyu> Shing Lyu Ohoh
6:06 PM " A &str is made up of two components: a pointer to some bytes, and a length. "
6:06 PM Got it
6:06 PM
<Quxxy2> (str is just [u8] with the added invariant of "must be valid UTF-8")
6:06 PM There are no [u8]s
6:06 PM There are, however, fixed-sized [u8; N]s
6:07 PM However, every [u8; N] with a different N is a totally different, incompatible type
6:07 PM This would make passing arrays around a huge pain
6:07 PM So you can "erase" the N from the type, turning it into [u8]
6:07 PM However, in order for this to still make sense, the length can't just go away, it has to be maintained somewhere
6:07 PM So it goes from being part of the type at compile-time, to being part of the value
6:08 PM → rembo10 joined (rembo10@moz-9vjc0f.ab2q.2uic.04f8.2a01.IP)
6:08 PM
<shinglyu> Shing Lyu Quxxy2: Wow
6:08 PM
<Quxxy2> But in order for all [u8]s to be the same, and to support cheap slicing, the length can't actually be stored anywhere as part of the [u8]
6:08 PM So *instead*, it gets promoted to part of the pointer *itself*
6:08 PM This means you can go &[u8; N] -> &[u8]
6:09 PM ⇐ junqed quit (junqed@moz-3ia.d39.70.178.IP) Ping timeout: 121 seconds
6:09 PM
<Quxxy2> &[u8; N] is a single pointer, &[u8] is the same pointer *plus* the N that got erased
6:09 PM
<shinglyu> Shing Lyu Quxxy2: So can I  use [u8]  myself? or I must use &[u8]
6:09 PM
<Quxxy2> The same thing happens with &str, except there's no "fixed length string" type in Rust
6:09 PM → mvyskocil joined (mvyskocil@moz-1fo.p0p.7.31.IP)
6:09 PM
<Quxxy2> [u8] is a valid type, you just can't *have* one
6:09 PM You can't store one in a variable, or an argument, or return it from a function
6:10 PM
<shinglyu> Shing Lyu Quxxy2: I see
6:10 PM
<Quxxy2> You can *only* ever deal with unsized types behind a pointer of some kind
6:10 PM &[u8], Box<[u8]>, Rc<[u8]>, etc.
6:10 PM There has to be something that knows the missing "N"
6:10 PM
<shinglyu> Shing Lyu Quxxy2: So if I try to assign it to a variable, it will generated the unsized error too?
6:10 PM
<Quxxy2> Yup
Havvy> What's the size of Box<[u8]> ?
<shinglyu> Shing Lyu Quxxy2: Cool! Now I understand
6:10 PM
<Quxxy2> Havvy: 2 pointers
6:11 PM → allengeorge joined (allengeorge@moz-4748b7.res.rr.com)
6:11 PM
<Quxxy2> shinglyu: The same is also true of trait objects, except the thing being erased there isn't N, it's the trait vtable
*/
