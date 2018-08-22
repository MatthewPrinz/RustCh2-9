use std::fs::File;
use std::io::prelude::*;

fn readFile(fileName: String) -> String {
    //let mut file = File::open(fileName).unwrap();
    let mut file = File::open(fileName).expect("Failed to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

fn fruitytest() {
    println!("******* Starting Base Test #1 (fruity) *******");
    let dictionary: String = String::from("apple\nbanana\norange\npear");
    let article: String = String::from("I ate an apple and a pear");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}

fn dogtest() {
    println!("******* Starting Base Test #2 (dogs) *******");
    let dictionary: String = String::from("Beagle\nBulldog\nCollie\nPoodle\nretriever\n");
    let article: String = String::from("bulldog dalmation beagle Retriever poodles");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}



fn punctuationTests() {
    println!("******* Starting Base Test #3 (punctuation) *******");
    let dictionary: String = String::from("but\ncan\ncan't\ndo\ndon't\nthink\n"); // the dictionary can contain punctuation
    let article: String = String::from("I think I can, but I can't. I think I do, but I don't");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");

}
fn testcase_1() {
    println!("\n\n******* test case 1 *******\n");
    let dictionary: String = String::from("but\ncan\ncan't\ndo\ndon't\nthink\n"); // the dictionary can contain punctuation
    let article: String = String::from("");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}

fn testcase_2() {
    println!("\n\n******* test case 2 *******\n");
    let dictionary: String = String::from("\n\n\n\n\ncat\n"); // the dictionary can contain punctuation
    let article: String = String::from("I think I cat, but I cat. I think I do, but I cat");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}

fn testcase_3() {
    println!("\n\n******* test case 3 *******\n");
    let dictionary: String = String::from("\n"); // the dictionary can contain punctuation
    let article: String = String::from("************* h********* hAo()os(so2");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}

fn testcase_4() {
    println!("\n\n******* test case 4 *******\n");
    let dictionary: String = String::from("*****\ncan\ncan't\ndo\ndon't\nthink\n23"); // the dictionary can contain punctuation
    let article: String = String::from("I think ***** can, but I can't. I th23ink I do, but I don't");
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}

fn easytest() {
    println!("\n\n******* Easy Test Case *******\n");
    let dictionary: String = String::from("happy\nhappier\nhappiest"); // the dictionary can contain punctuation
    let article: String = String::from("happier happy sad");
    spellCheck(&article, &dictionary); //should print sad
    println!("****DONE****\n");
}

fn capitaltest() {
    println!("\n\n******* Capital Test Case *******\n");
    let dictionary: String = String::from("HAPPY\nhappier\nhappiest\nhappy"); // the dictionary can contain punctuation
    let article: String = String::from("happier happy sad HAPPIER");
    spellCheck(&article, &dictionary); //should print sad
    println!("****DONE****\n");
}

fn fmltest() {
    println!("\n\n******* FML Test Case *******\n");
    let dictionary: String = String::from("firetruck"); // the dictionary can contain punctuation
    let article: String = String::from("firetruck fire fi re");
    spellCheck(&article, &dictionary); //should print: fire fi re
    println!("****DONE****\n");
}
fn robusttest() {
    println!("\n\n******* Robust Test Case *******\n");
    let dictionary: String = String::from("nope\nwhups"); // the dictionary can contain punctuation
    let article: String = String::from("-nope wait wtf whups");
    spellCheck(&article, &dictionary); //should print: wait wtf
    println!("****DONE****\n");
}

fn generalTest1() {
    println!("\n\n******* Article Test #1 *******\n");
    let dictionary:String = readFile(String::from("american-english.txt"));
    let article:String = readFile(String::from("bobsledding.txt"));
    spellCheck(&article, &dictionary);
    println!("****DONE****\n");
}

fn main() {
    fruitytest();
    /*
    dogtest();
    punctuationTests();
    testcase_1();
    testcase_2();
    testcase_3();
    testcase_4();
    fmltest();
    easytest();
    capitaltest();
    robusttest();
    */
    generalTest1();

}

fn spellCheck(article: &String, dictionary: &String) {

    let mut split_article:Vec<String> = Vec::new();
    let mut split_dictionary:Vec<String> = Vec::new();
    let mut result:Vec<String> = Vec::new();
    let mut article_word = String::new();
    let mut flag:bool = false;

    // Every word of the dictionary is given to the vector split_dictionary
    let dict = dictionary.split('\n');
    for word in dict {
        split_dictionary.push(word.to_string());
    }

    // This following code parses the article, and puts words into the split_article vector one by one.
    for letter in article.chars() {
        if letter.is_alphabetic() {
            article_word.push(letter);
            // Since dictionary words have to be alphabetic characters
            if letter == article.chars().last().unwrap() {
                // This handles a specific edge case where the article_word is the last word.
                if article_word.len() > 1 {
                    let mut article_word_copy = article_word.clone();
                    split_article.push(article_word_copy);
                }
            }
        } else {
            if article_word.len() > 1 {
                // Since dictionary words have to be greater than 1 character
                let mut article_word_copy = article_word.clone();
                split_article.push(article_word_copy);
            }
            article_word.clear();
        }
    }
    // The following code iterates through the split_article and the split_dictionary vectors, and sees
    // if it finds the word in the dictionary. If it finds the word, it sets a flag. If the flag is not set,
    // that indicates the word is not in the dictionary, thus, it should be added to the result vector.
    for word0 in split_article.iter() {
        for word1 in split_dictionary.iter() {
            if *word1.to_ascii_lowercase() == *word0.to_ascii_lowercase() {
                //Since case doesn't matter
                flag = true;
                break;
            }
        }
        if !flag {
            let mut copy_word = word0.clone();
            result.push(copy_word);
        }
        flag = false;
    }
    result.dedup(); //removes duplicates
    for word in result {
        println!("{}", word);
    }
}


