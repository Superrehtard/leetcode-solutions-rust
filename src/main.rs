mod search_insert;
mod plus_one;
mod p5;
mod p58;
mod p209;
mod valid_palindrome_125;

use crate::plus_one::plus_one;
use p58::length_of_last_word;
use p209::min_sub_array_len;
use p5::longest_palindrome;

fn main() {
    let nums = vec![9,8,7,6,5,4,3,2,1,0];
    // println!("search insert: {}", search_insert_35(&nums, 6));

    println!("plus one: {:?}", plus_one(nums));

    length_of_last_word("hello world");

    let mut iter = "".split_ascii_whitespace();
    println!("{:?}", iter.next());

    let num_array = vec![1,2,3,4,5];

    println!("{}", min_sub_array_len(11, num_array));

    // println!("The longest palindrome: {}", longest_palindrome(String::from("kztakrekvefgchersuoiuatzlmwynzjhdqqftjcqmntoyckqfawikkdrnfgbwtdpbkymvwoumurjdzygyzsbmwzpcxcdmmpwzmeibligwiiqbecxwyxigikoewwrczkanwwqukszsbjukzumzladrvjefpegyicsgctdvldetuegxwihdtitqrdmygdrsweahfrepdcudvyvrggbkthztxwicyzazjyeztytwiyybqdsczozvtegodacdokczfmwqfmyuixbeeqluqcqwxpyrkpfcdosttzooykpvdykfxulttvvwnzftndvhsvpgrgdzsvfxdtzztdiswgwxzvbpsjlizlfrlgvlnwbjwbujafjaedivvgnbgwcdbzbdbprqrflfhahsvlcekeyqueyxjfetkxpapbeejoxwxlgepmxzowldsmqllpzeymakcshfzkvyykwljeltutdmrhxcbzizihzinywggzjctzasvefcxmhnusdvlderconvaisaetcdldeveeemhugipfzbhrwidcjpfrumshbdofchpgcsbkvaexfmenpsuodatxjavoszcitjewflejjmsuvyuyrkumednsfkbgvbqxfphfqeqozcnabmtedffvzwbgbzbfydiyaevoqtfmzxaujdydtjftapkpdhnbmrylcibzuqqynvnsihmyxdcrfftkuoymzoxpnashaderlosnkxbhamkkxfhwjsyehkmblhppbyspmcwuoguptliashefdklokjpggfiixozsrlwmeksmzdcvipgkwxwynzsvxnqtchgwwadqybkguscfyrbyxudzrxacoplmcqcsmkraimfwbauvytkxdnglwfuvehpxd")));
    println!("The longest palindrome: {}", longest_palindrome(String::from("cbbd")));
    println!("The longest palindrome: {}", longest_palindrome(String::from("racecar")));
    // println!("The longest palindrome: {}", longest_palindrome(String::from("abb")));
}