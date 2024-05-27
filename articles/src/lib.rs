/*
DIY project no. 25007
Write a rust crate that could count the number of articles in a sentence.
Let the users be able to use it in multiple languages.
Let it help the users to determine what language type is the sentence written in.

*/

#[derive(Clone, Debug)]
pub struct Articles<'a> {
    pub sentence: String,
    pub number_of_articles: i32,
    pub language: &'a str,
    pub frequency: Vec<ArticlesMap<'a>>,
    pub mode: String,
    pub mode_count: i32,
}

#[derive(Clone, Debug)]
pub struct ArticlesMap<'a> {
    pub key: LanguageMap<'a>,
    pub value: i32,
}

#[derive(Clone, Debug)]
pub struct LanguageMap<'a> {
    pub key: String,
    pub value: &'a str,
}

impl Articles<'_> {
    pub fn count_articles(&mut self) -> (i32, &str, String, i32) {
        let sentence = self.sentence.clone();
        let len = sentence.len();
        assert!(len > 0);
        let mut sentence_articles: Vec<LanguageMap> = Vec::new();
        let mut articles_list: Vec<ArticlesMap> = Vec::new();
        let words: Vec<_> = sentence.split(' ').collect();
        for x in words {
            let x2 = x.to_string();
            if x2.len() > 0 {
                let (is_article, language_x, article) = check_if_word_is_an_article(x2);
                if is_article {
                    sentence_articles.push(LanguageMap{key: article, value: language_x});
                }
            }
        }
        let len_x = sentence_articles.len();
        if len_x > 0 {
            sort_languege_map(&mut sentence_articles);
        } else {
            return (0, "", "".to_string(), 0);
        }
        let mut count: i32 = 0;
        let mut map_x = sentence_articles[0].clone();
        let mut mode = map_x.key.clone();
        let mut language = map_x.value.clone();
        let mut mode_count = 1;
        for y in sentence_articles {
            if y.key == map_x.key {
                count += 1;
            } else {
                if count > mode_count {
                    mode = map_x.key.clone();
                    language = map_x.value.clone();
                    mode_count = count;
                }
                articles_list.push(ArticlesMap{key: map_x.clone(), value: count});
                count = 1;
                map_x = y.clone();
            }
        }
        articles_list.push(ArticlesMap{key: map_x, value: count});
        self.number_of_articles = len_x.try_into().unwrap();
        self.language = language.clone();
        self.frequency = articles_list;
        self.mode = mode.clone();
        self.mode_count = mode_count;
        (len_x.try_into().unwrap(), language, mode, mode_count)
    }
}

pub fn new(sentence: String) -> Articles<'static> {
    let mut articles_x = Articles {
        sentence: sentence,
        number_of_articles: 0,
        language: "",
        frequency: Vec::new(),
        mode: String::new(),
        mode_count: 0,
    };
    articles_x.count_articles();
    articles_x
}

pub fn check_if_word_is_an_article(word: String) -> (bool, &'static str, String) {
    let len = word.len();
    assert!(len > 0);
    let language_a = vec!["the", "an", "a"];
    let language_b = vec!["le", "la", "l'", "les", "un", "une", "des", "de", "d'", "du"];
    let language_c = vec!["u", "a", "i"];
    let word_x = word.clone();
    let article = str::to_lowercase(&word_x);
    for a in language_a {
        if a == article {
            return (true, "Language_A", article);
        }
    }
    // println!("#1 @Language_A. {}", word);
    for b in language_b {
        if b == article {
            return (true, "Language_B", article);
        } else if (b == "l'") | (b == "d'") {
            let x = b.chars().nth(0);
            let y: Vec<_> = article.split('\'').collect();
            // println!("y: {:?}", y);
            if y.len() > 1 {
                for x2 in y {
                    let x0 = x2.len();
                    // println!("x0: {} x2: {}", x0, x2);
                    if ((*x2).chars().next() == x) && (x0 == 1) {
                       return (true, "Language_B", b.to_string());
                    }
                    break;
                }
            }
        }
    }
    // println!("#2 Language_B...... {}", word);
    for c in language_c {
        if c == article {
            return (true, "Language_C", article);
        }
        let x = article.chars().nth(0);
        let x2 = (*c).chars().next();
        if x2 == x {
            // return (true, "Language_C", article);
            return (true, "Language_C", c.to_string());
        }
    }
    // println!("#3 Language_C......{}", word);
    let language = "Language_X";
    (false, language, word)
}

pub fn sort_languege_map(map: &mut Vec<LanguageMap>){
    let len = map.len();
    assert!(len > 0);
    let mut i = 0;
    while i < (len - 1) {
        let mut j = i + 1;
        while j < len {
            if *(map[j]).key < *(map[i]).key {
                let x = map[i].clone();
                let y = map[j].clone();
                map[i] = y;
                map[j] = x;
            }
            j += 1;
        }
        i += 1;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_test<T: std::fmt::Debug>(message: &str, test_x: T){
    println!("{}: {:#?}",  message, test_x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn check_if_word_is_an_article_test() {
        let word  = "Cool".to_string();
        let word_check = check_if_word_is_an_article(word.clone());
        assert_eq!(false, word_check.0);
        assert_eq!("Language_X", word_check.1);
        assert_eq!(word, word_check.2);
        //
        let word  = "The".to_string();
        let word_check = check_if_word_is_an_article(word.clone());
        assert_eq!(true, word_check.0);
        assert_eq!("Language_A", word_check.1);
        assert_eq!(word.to_lowercase(), word_check.2);
        //
        let word  = "le".to_string();
        let word_check = check_if_word_is_an_article(word.clone());
        assert_eq!(true, word_check.0);
        assert_eq!("Language_B", word_check.1);
        assert_eq!(word, word_check.2);
        //
        let word  = "u".to_string();
        let word_check = check_if_word_is_an_article(word.clone());
        assert_eq!(true, word_check.0);
        assert_eq!("Language_C", word_check.1);
        assert_eq!(word, word_check.2);
        //
    }

    #[test]
    fn count_articles_test() {
        let mut article_x = Articles {
            sentence: "Kwakukhona ikhehla elalihama lithi, lithi.".to_string(), // String,
            number_of_articles: 0, // i32,
            language: "", // &'a str,
            frequency: Vec::new(), // Vec<ArticlesMap<'a>>,
            mode: String::new(), // String,
            mode_count: 0, // i32,
        };
        let count_x = article_x.count_articles();
        assert!(count_x.0 == 1);
    }
}
