///
/// ## Magic Numbers
///
/// A magic number is one that can only be formed by concatenating:
/// - 1
/// - 14
/// - 144
///
/// ## Returns true if the number gave as input is magic false otherwise
///
/// ## Strategy
/// We want to get the number digit by digit, this means that the concatenation of:
/// - 1
/// - 41
/// - 441
///
/// We take the number digit by digit from the end using the remainder, and we perform
/// some conditions:
/// - if we see a four we set a flag: "previous_four"
/// - if previous_four is setted: if we see a 4: we set a flag "need_one"
/// - if we see a one, we reset all flags
///
/// When processing a number:
/// - if 1 clear all flags and continue;
/// - if need_one is true -> exit (no one resetted the flag)
/// - if 4 and previous_four then set the need_one flag
///   else set the previous_four flag  
///
/// When the remainder is 0 we have to check if the last digit seen was 1
pub fn magic_number(mut n: usize) -> bool {
    let (mut need_one, mut prev_four) = (false, false);
    while n > 0 {
        match n % 10 {
            1 => (need_one, prev_four) = (false, false),
            4 if need_one => return false,
            4 if prev_four => (prev_four, need_one) = (false, true),
            4 => prev_four = true,
            _ => break,
        }
        if n < 10 {
            return n == 1;
        }
        n /= 10;
    }

    false
}
