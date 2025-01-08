#![warn(missing_docs)]

/*!
This crate implements a simple but powerful profanity filter.

While this filter can still be technically subverted, the goal is that by
the time a profanity gets past the filter, it barely resembles the original word.
This is done by subverting common profanity filter workarounds, such as inserting
spaces or special characters in between letters (`F_U_C_K`) or using similar-looking
characters in the place of others (`SH!T`).

Keep in mind though, that this filter is far from perfect. If people *really* want
to swear, they can get through this filter.

# Usage
The [`Censor`] enum is the main object used for censoring strings.
It is essentially a set of words to be filtered out. The [`Standard`]
variant contains words that most people consider to be swear words, and is meant to be a good
baseline for a filter. More sets and individual words can be added with the `+` and `+=`
operators, and sets and words can be removed with the `-` and `-= operators`.

```
use censor::*;

let censor = Censor::Standard;

// Use `Censor::check` to check if a string contains a profanity
assert!(censor.check("fuck"));
assert!(censor.check("FUCK"));
assert!(censor.check("FuCk"));
assert!(censor.check("fμ¢κ"));
assert!(censor.check("f!u!c!k"));
assert!(censor.check("F_u c_K"));
assert!(censor.check("fuuuuuuuck"));

assert!(!censor.check("fluff truck"));
assert!(!censor.check("fukushima"));

// Use `Censor::censor` to censor a string with asterisks
assert_eq!("*_*_*_*_*", censor.censor("₱_û_$_$_¥"));
assert_eq!("**** that ****, dude", censor.censor("fuck that shit, dude"));
assert_eq!("******* yoouuu", censor.censor("fuuuuck yoouuu"));

// Use `Censor::replace` to replace censored words with any grawlix string
assert_eq!("What the !@#$?", censor.replace("What the fuck?", "!@#$%"));

// You can combine `Censor`s and add your own words
let censor = Standard + Zealous + Sex + "dong";

assert_eq!(
    "Woops, I dropped my monster ******, that I use for my magnum ****",
    censor.censor("Woops, I dropped my monster condom, that I use for my magnum dong")
);

// You can remove words from `Censor`s too
let censor = Standard - "ass";
assert!(!censor.check("I don't care if people say 'ass'"));

// Overlapping censored words are fully censored
let censor = Standard + Sex;
assert_eq!("**********", censor.censor("shititties"));
assert_eq!("*************", censor.censor("blowjoboobies"))
```
 */

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use once_cell::sync::Lazy;

///
pub static CENSOR: Lazy<Censor> = Lazy::new(|| Standard + Sex);

static CHAR_ALIASES: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    const CASE_DIFF: u8 = b'a' - b'A';
    for c in b'A'..=b'Z' {
        map.insert(c as char, (c + CASE_DIFF) as char);
    }
    macro_rules! alias {
        ($reduced:literal => $($alias:literal),*) => {
            $(map.insert($alias, $reduced);)*
        };
    }
    alias!('a' => '4', '@', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'à', 'á', 'â', 'ã', 'ä', 'å', 'α', 'Α');
    alias!('b' => 'ß', 'Β', '฿');
    alias!('c' => '¢', 'ç', 'Ç', '©');
    alias!('d' => 'Ð', '₫');
    alias!('e' => '3', '£', '€', 'È', 'É', 'Ê', 'Ë', 'è', 'é', 'ê', 'ë', 'ε', 'Ε', 'Ξ', 'Σ');
    alias!('g' => '6');
    alias!('h' => 'Η');
    alias!('k' => 'κ', 'Κ');
    alias!('i' => '1', '|', '!', 'Ì', 'Í', 'Î', 'Ï', 'ì', 'í', 'î', 'ï', 'Ι');
    alias!('m' => 'Μ');
    alias!('n' => 'ñ', 'Ñ', 'η', 'Ν', 'Π');
    alias!('o' => '0', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', 'ò', 'ó', 'ô', 'õ', 'ö', 'Ø', 'ø', 'θ', 'ο', 'σ', 'Θ', 'Ο', 'Φ');
    alias!('p' => 'ρ', 'Ρ', '₱', '℗', 'Þ', 'þ');
    alias!('r' => '®');
    alias!('s' => '5', '$');
    alias!('t' => 'τ', 'Τ');
    alias!('u' => 'Ù', 'Ú', 'Û', 'Ü', 'ù', 'ú', 'û', 'ü', 'μ', 'υ');
    alias!('v' => 'ν');
    alias!('w' => 'ω', '₩');
    alias!('x' => '×', 'χ', 'Χ');
    alias!('y' => '¥', 'Ý', 'ý', 'ÿ', 'γ', 'Υ');
    alias!('z' => '2', 'Ζ');
    map
});

macro_rules! word_set {
    ($doc:literal, $name:ident, $($word:literal),*) => {
        #[doc = $doc]
        #[doc = ""]
        #[doc = "#### List"]
        $(
            #[doc = $word]
            #[doc = ""]
        )*
        pub static $name: Lazy<HashSet<String>> = Lazy::new(|| {
            let mut set = HashSet::new();
            let words = [$($word),*];
            for i in 0..words.len() {
                set.insert(String::from(words[i]));
            }
            set
        });
    };
}

word_set!(
    "Words that are profanities by most people's definition",
    STANDARD_WORDS,
    "ass",
    "asshole",
    "bitch",
    "cock",
    "cunt",
    "fag",
    "fagot",
    "faggot",
    "fuck",
    "nigger",
    "piss",
    "pussy",
    "shit",
    "twat",
    "whore",
    "屁股",
    "混蛋",
    "婊子",
    "鸡巴",
    "阴户",
    "阴道",
    "基佬",
    "他妈的",
    "你妈的",
    "妈的",
    "黑鬼",
    "小便",
    "阴户",
    "屎",
    "娘们",
    "妓女"
);
word_set!(
    "Words that are profanities only to the zealous",
    ZEALOUS_WORDS,
    "crap",
    "damn",
    "goddamn",
    "hell",
    "suck"
);
word_set!(
    "Words related to sex",
    SEX_WORDS,
    "ass",
    "asshole",
    "blowjob",
    "boob",
    "boobie",
    "boobies",
    "boobjob",
    "breast",
    "clitoris",
    "cock",
    "condom",
    "cunnilingus",
    "cunt",
    "dick",
    "doggystyle",
    "ejaculate",
    "felate",
    "felatio",
    "fetish",
    "foreskin",
    "handjob",
    "labia",
    "masturbate",
    "masturbation",
    "masterbate",
    "masterbation",
    "penis",
    "pussy",
    "rimjob",
    "semen",
    "sex",
    "tits",
    "tittie",
    "titties",
    "titty",
    "twat",
    "vagina",
    "vulva",
    "屁股",
    "屁眼",
    "口交",
    "乳房",
    "乳交",
    "乳房",
    "鸡巴",
    "避孕套",
    "舔阴",
    "阴户",
    "阴茎",
    "后入式",
    "射精",
    "口交",
    "恋物癖",
    "包皮",
    "手淫",
    "阴唇",
    "手淫",
    "阴户",
    "舔肛",
    "精液",
    "性",
    "乳房",
    "乳头",
    "阴道",
    "外阴"
);

/**
A collection of words to censor
 */
#[derive(Debug, Clone, Eq)]
pub enum Censor {
    /**
    Standard swear words

    For more information, see [`STANDARD_WORDS`]
     */
    Standard,
    /**
    Words related to sex

    Not usually used by itself

    For more information, see [`SEX_WORDS`]
     */
    Sex,
    /**
    Words that are profanities only to the zealous

    Not usually used by itself

    For more information, see [`ZEALOUS_WORDS`]
     */
    Zealous,
    /// A custom set of words
    Custom(HashSet<String>),
}

pub use Censor::*;

impl Default for Censor {
    fn default() -> Self {
        Standard
    }
}

impl Censor {
    /// Create an empty `Censor`
    pub fn empty() -> Self {
        Custom(HashSet::new())
    }
    /// Create a `Censor::Custom`
    pub fn custom<I, W>(words: I) -> Self
    where
        I: IntoIterator<Item = W>,
        W: Into<String>,
    {
        Custom(words.into_iter().map(Into::into).collect())
    }
    /// Check if a string contains censored words
    pub fn check(&self, text: &str) -> bool {
        !self.bad_chars(text, 0, 0).is_empty()
    }
    /// Count the number of censored words in a string
    ///
    /// # Example
    /// ```
    /// use censor::*;
    ///
    /// let censor = Censor::Standard;
    ///
    /// assert_eq!(0, censor.count("dog"));
    /// assert_eq!(1, censor.count("motherfucker"));
    /// assert_eq!(2, censor.count("bitch ass guy"));
    /// ```
    pub fn count(&self, text: &str) -> usize {
        let bad_chars = self.bad_chars(text, 0, 0);
        let mut count = 0;
        let mut in_censored = false;
        for i in 0..text.chars().count() {
            if bad_chars.contains(&i) {
                if !in_censored {
                    in_censored = true;
                    count += 1;
                }
            } else {
                in_censored = false;
            }
        }
        count
    }
    /// Replace censored words in the string with asterisks (`*`s)
    pub fn censor(&self, text: &str) -> String {
        self.replace(text, "*")
    }
    /**
    Replace censored words in the string with characters from a 'grawlix' string (#?!@$)

    # Panics
    Panics if the grawlix string is empty
     */
    #[track_caller]
    pub fn replace(&self, text: &str, grawlix: &str) -> String {
        self.replace_with_offsets(text, grawlix, 0, 0)
    }
    /**
    Replace censored words in the string with characters from a 'grawlix' string (#?!@$)

    Characters at indices within the given offsets from the start and end of words will not be censored

    # Panics
    Panics if the grawlix string is empty
     */
    #[track_caller]
    pub fn replace_with_offsets(
        &self,
        text: &str,
        grawlix: &str,
        start_offset: usize,
        end_offset: usize,
    ) -> String {
        if grawlix.is_empty() {
            panic!("grawlix is empty");
        }
        let graw_chars: Vec<char> = grawlix.chars().collect();
        let mut graw_offset: usize = 0;

        let bad_chars = self.bad_chars(text, start_offset, end_offset);
        text.chars()
            .enumerate()
            .map(|(i, c)| {
                if bad_chars.contains(&i) {
                    let graw = graw_chars[graw_offset];
                    graw_offset = (graw_offset + 1) % graw_chars.len();
                    graw
                } else {
                    c
                }
            })
            .collect()
    }
    /// Get a set of the indices of characters in the given string that
    /// are part of censored words
    pub fn bad_chars(&self, text: &str, start_offset: usize, end_offset: usize) -> HashSet<usize> {
        let lowercase = text.to_lowercase();
        let sizes: BTreeSet<usize> = self.list().map(|s| s.len()).collect();
        // Check just alpha
        let (alphanum_only, alphanum_map) = remove_non_alpha(&lowercase);
        let bad_alphanum_chars = self._bad_chars(
            &alphanum_only,
            &alphanum_map,
            &sizes,
            start_offset,
            end_offset,
        );
        // Check aliased then without whitespace
        let (alias_ws, alias_ws_map) = remove_whitespace(&alias(&lowercase));
        let bad_alias_ws_chars =
            self._bad_chars(&alias_ws, &alias_ws_map, &sizes, start_offset, end_offset);
        // Check aliased then just alpha
        let (alias_alphanum, alias_alphanum_map) = remove_non_alpha(&alias(&lowercase));
        let bad_alias_alphanum_chars = self._bad_chars(
            &alias_alphanum,
            &alias_alphanum_map,
            &sizes,
            start_offset,
            end_offset,
        );
        // Union sets
        bad_alphanum_chars
            .into_iter()
            .chain(bad_alias_ws_chars)
            .chain(bad_alias_alphanum_chars)
            .collect()
    }
    fn _bad_chars(
        &self,
        text: &str,
        map: &HashMap<usize, usize>,
        sizes: &BTreeSet<usize>,
        start_offset: usize,
        end_offset: usize,
    ) -> HashSet<usize> {
        let (deduped, dd_map) = dedup_string(text);
        let mut set = HashSet::new();
        for &size in sizes.iter().rev() {
            for word in self.list().filter(|s| s.len() == size) {
                for (i, _) in text.match_indices(word.as_str()) {
                    for j in start_offset..word.len().saturating_sub(end_offset) {
                        let k = i + j;
                        if let Some(k) = map.get(&k) {
                            set.insert(*k);
                        }
                    }
                }
                for (i, _) in deduped.match_indices(word.as_str()) {
                    for j in start_offset..word.len().saturating_sub(end_offset) {
                        let k = i + j;
                        if let Some(ls) = dd_map.get(&k) {
                            for l in ls {
                                if let Some(k) = map.get(l) {
                                    set.insert(*k);
                                }
                            }
                        }
                    }
                }
            }
        }
        set
    }
    /// Get a reference to the set used by the `Censor`
    pub fn set(&self) -> &HashSet<String> {
        match self {
            Standard => &STANDARD_WORDS,
            Zealous => &ZEALOUS_WORDS,
            Sex => &SEX_WORDS,
            Custom(words) => words,
        }
    }
    /// Get an iterator over all censored words
    pub fn list(&self) -> std::collections::hash_set::Iter<String> {
        self.set().iter()
    }
    /// Find a censored word in the `Censor`. Applies character aliases
    pub fn find(&self, word: &str) -> Option<&str> {
        let word = alias(word);
        self.set().get(&word).map(|w| w.as_str())
    }
    /// Check if the `Censor` contains a word. Applies character aliases
    pub fn contains(&self, word: &str) -> bool {
        self.find(word).is_some()
    }
}

impl AddAssign for Censor {
    fn add_assign(&mut self, other: Self) {
        *self = Censor::Custom(self.set().union(other.set()).cloned().collect());
    }
}

impl PartialEq for Censor {
    fn eq(&self, other: &Self) -> bool {
        self.set() == other.set()
    }
}

impl<S> AddAssign<S> for Censor
where
    S: Into<String>,
{
    fn add_assign(&mut self, other: S) {
        *self = Censor::Custom(self.list().cloned().chain(Some(other.into())).collect());
    }
}

impl SubAssign for Censor {
    fn sub_assign(&mut self, other: Self) {
        *self = Censor::Custom(self.set().difference(other.set()).cloned().collect());
    }
}

impl<S> SubAssign<S> for Censor
where
    S: Into<String>,
{
    fn sub_assign(&mut self, other: S) {
        let other = other.into();
        *self = Censor::Custom(self.list().filter(|&s| s != &other).cloned().collect());
    }
}

impl Add for Censor {
    type Output = Censor;
    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl<S> Add<S> for Censor
where
    S: Into<String>,
{
    type Output = Censor;
    fn add(mut self, other: S) -> Self::Output {
        self += other;
        self
    }
}

impl Sub for Censor {
    type Output = Censor;
    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl<S> Sub<S> for Censor
where
    S: Into<String>,
{
    type Output = Censor;
    fn sub(mut self, other: S) -> Self::Output {
        self -= other;
        self
    }
}

fn alias(text: &str) -> String {
    text.chars()
        .map(|c| CHAR_ALIASES.get(&c).copied().unwrap_or(c))
        .collect()
}

fn remove_whitespace(text: &str) -> (String, HashMap<usize, usize>) {
    let mut output = String::new();
    let mut map = HashMap::new();
    for (i, (j, c)) in text
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .enumerate()
    {
        output.push(c);
        map.insert(i, j);
    }
    (output, map)
}

fn remove_non_alpha(text: &str) -> (String, HashMap<usize, usize>) {
    let mut output = String::new();
    let mut map = HashMap::new();
    for (i, (j, c)) in text
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .enumerate()
    {
        output.push(c);
        map.insert(i, j);
    }
    (output, map)
}

fn dedup_string(s: &str) -> (String, HashMap<usize, Vec<usize>>) {
    let mut last = None;
    let mut res = String::new();
    let mut map = HashMap::new();
    let mut j = 0;
    for (i, c) in s.chars().enumerate() {
        if last.map(|l| l != c).unwrap_or(true) {
            res.push(c);
            map.entry(j).or_insert_with(Vec::new).push(i);
            j += 1;
        } else {
            map.entry(j).or_insert_with(Vec::new).push(i);
        }
        last = Some(c);
    }
    (res, map)
}

#[test]
fn test() {
    let data = CENSOR.censor("you 他妈的");
    println!("{data}");
}
