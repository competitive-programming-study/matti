pub mod holiday_planning {

    ///
    /// Computes the maximum numbers of attractions visitable given n cities
    /// and d days
    ///
    /// ## Parameters
    /// - `attractions: &[usize]`: a slice of usizes that encodes a flattened matrix
    /// - `cities: usize`: the total number of cities (rows of the matrix)
    /// - `days: usize`: the total number of days (columns of the matrix)
    ///
    /// ## Returns
    /// An usize with the maximum number of attractions
    ///
    /// ## Strategy
    /// We're given `n` itineraries, one for each city and a number of attractions
    /// visitable for each day. For each city `c_i` we `A_i[k]` with `k < days` which
    /// stores the number of attractions visitable in the city posed we stay there for
    /// `k` days.
    ///
    /// For each city we compute `A[i]` as the prefix sum array of the attractions for the city
    /// so that `A[i][k]` is the sum of the attractions until index `k < days`.
    ///
    /// Having this, we employ a memoization matrix, `M[cities + 1][days + 1]`. We compile
    /// this matrix so that every `M[i][j]` stores in the end, the maximum number of attractions
    /// considering `i < cities` cities in exactely `j < days` days.
    ///
    /// We initialize the matrix as:
    /// - `M[0][i] = 0`: we will visit 0 attractions considering 0 cities
    /// - `M[i][0] = 0`: we will visit 0 attractions considering 0 days
    ///
    /// Then we insitute the recurrence equation, by which:
    /// - `M[i][j] = max[0 <= k <= j]( M[i][j - k] + A[i][k] )`
    ///
    /// In this way we consider the maximum achievabe in each step as the max in
    /// spending `j-k` in the previous `i-1` cities and the remaining `k` days in  
    /// the `i_th` city.
    ///
    /// At the end we'll extract the result as the maximum value in the last column
    /// of the matrix. For each cell `i` in the last column represents the maximum
    /// number of attractions visitable given an holiday of the parameter `days`
    /// considering `i` cities.
    ///
    /// ## Complexity
    /// We require `O(cities * days)` additional space to store the prefix sum matrix
    /// and the memoization matrix.
    ///
    /// For the time complexity we consider that for each day `j in [0,days]` we iterate
    /// `j` times over each city (to fill the `j_th` column of the matrix) with this we
    /// can compute:
    ///
    /// `T(n) = n + 2n + 3n + ... + days(n) = O(n*days^2)`
    ///
    ///
    pub fn plan(attractions: &[usize], cities: usize, days: usize) -> usize {
        if cities == 0 || days == 0 {
            return 0;
        }

        //assert that the flatten matrix contains as many cells as we're accounting for
        assert_eq!(cities * days, attractions.len());

        //The first column is 0, for ease in the next step
        let mut prefix_sums = Vec::with_capacity(cities * (days + 1));

        //compile the prefix sum matrix
        for i in 0..cities {
            //start index of the current prefix sum (not considering the leading 0)
            let base = i * days;
            let mut sum = 0;
            prefix_sums.push(sum); //push the leading 0
            for j in 0..days {
                sum += attractions[base + j];
                prefix_sums.push(sum);
            }
        }

        // we initialize all cells to 0 (easier) even tho we could have only
        // initialized first row and column
        let mut dp = vec![0; (cities + 1) * (days + 1)];

        let mut max_attractions_sf = 0;

        for i in 1..=cities {
            //start index of the previous row (and current prefix_sum). this matrix has one more
            //row of zeroes at the start
            let prev_base = (i - 1) * (days + 1);
            //start index of the current row
            let base = i * (days + 1);

            for d in 0..=days {
                //max so far for the current cell
                let mut max_sf = 0;
                for k in 0..=d {
                    //update value for the current cell
                    max_sf = max_sf.max(dp[prev_base + (d - k)] + prefix_sums[prev_base + k]);
                }
                //update the current cell
                dp[base + d] = max_sf;
            }
            //update the current max for the full holiday [last column]
            max_attractions_sf = max_attractions_sf.max(dp[base + days]);
        }
        max_attractions_sf
    }

    pub fn parse_input(input: &str) -> (usize, usize, Vec<usize>) {
        let mut lines = input.lines();

        // First line: two numbers (e.g., "6 8")
        let first_line = lines
            .next()
            .expect("Input must contain at least one line with dimensions");

        let mut dimensions = first_line
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Invalid number in dimensions"));

        let rows = dimensions.next().expect("Missing number of rows");
        let cols = dimensions.next().expect("Missing number of columns");

        // Parse remaining lines into a flat Vec<usize>
        let matrix_flat: Vec<usize> = lines
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<usize>().expect("Invalid number in matrix"))
            })
            .collect();

        assert_eq!(
            matrix_flat.len(),
            rows * cols,
            "Matrix data does not match dimensions"
        );

        (rows, cols, matrix_flat)
    }

    pub fn parse_output(output: &str) -> usize {
        let line = output
            .lines()
            .next()
            .expect("File must contain a single unsigned number");
        line.parse().unwrap()
    }

    #[cfg(test)]
    mod test {
        use crate::holiday_planning::*;
        use std::env::current_dir;
        use std::fs::read_to_string;
        use std::path::PathBuf;

        fn test_path(name: &str) -> PathBuf {
            current_dir().unwrap().join("test_holiday").join(name)
        }

        fn test_files(input_name: &str, output_name: &str) {
            let (input_path, output_path) = (test_path(input_name), test_path(output_name));

            let (input_str, output_str) = (
                read_to_string(input_path).unwrap(),
                read_to_string(output_path).unwrap(),
            );

            let (cities, days, attractions) = parse_input(&input_str);
            let max_attractions = parse_output(&output_str);

            assert_eq!(plan(&attractions, cities, days), max_attractions);
        }

        #[test]
        fn test_io_holiday_planning() {
            for i in 0..=4 {
                let input = format!("input{i}.txt");
                let output = format!("output{i}.txt");
                test_files(&input, &output)
            }
        }
    }
}

pub mod course_design {
    //the output of both have the same signature and this function
    //is the only thing the 2 modules have in common
    pub type Topic = (i64, i64);

    ///
    /// Computes the maximum numbers of topics that match the course criteria
    ///
    /// ## Parameters
    /// - `topics: &[course_design::Topic]` a list of topics `(interest: i64, difficulty: i64)`
    ///
    /// ## Returns
    /// An `usize` with the maximum numer of topics a course can cover
    ///
    /// ## Constraints
    /// A course contains an ordered list of topics, each one having some `interest` and `difficulty`
    /// factors. For each topic `i` in the (ordered) list, `topic[i].interest > topic[i-1].interest`
    /// and `topic[i].difficulty > topic[i-1].difficulty`
    ///
    /// ## Strategy
    /// We notice that the problem requirements and constraints match with the definition of
    /// the `longest_increasing_subsequence` problem dependant on 2 dimensions.
    ///
    /// We can reduce this instance to a one parameter `LIS` instance by sorting all items in
    /// ascendent order for the interest parameter then decreasing for the difficulty parameter
    /// (if the parameter allows for duplicate interest parameters)
    /// This is because, since we're processing topics by interest order, we can incapsulate the
    /// extension of the current LIS as just checking if the current difficulty is greater than
    /// the difficulty of the last element of the LIS. Since we processed harder difficulties first
    ///  with the same interest level, any subsequence we can extend will have its last element from
    /// a previous interest level (which satisfies our interest constraint).
    ///
    /// ### Solving Longest Increasing Subsequence 1D
    /// In this particular implementation we use the quadratic approach, so for each
    /// index `i` of the sorted topic list, we look at all the previous indexes `j` updating the current
    /// solution with the max of the previous solutions + 1. When doing this we institute a custom
    /// comparator. In normal `LIS` instances when considering if updating the current solution we
    /// would check if the item at position `i` was greater than the one at `j`, while in this instance
    /// we have to check if both parameters of the topic `i` are greater than the parameters of topic `j`
    ///
    /// ## Complexity
    /// We solve the 1-D LIS instance in `O(n^2)` time with `O(n)` additional memory
    /// for the dp array.
    ///
    /// To reduce the original problem, we take `O(n(log(n)))` for sorting and `O(n)`
    /// memory
    ///
    /// So the overall complexity is `O(n^2)` time and `O(n)` space.
    ///
    pub fn design_quadratic(topics: &[Topic]) -> usize {
        let mut topics = topics.to_vec();
        topics.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));
        let n = topics.len();
        let mut dp = vec![1; n];
        let mut max_len = 1;

        for i in 0..n {
            for j in 0..i {
                if topics[j].1 < topics[i].1 {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_len = max_len.max(dp[i]);
        }

        max_len
    }

    ///
    /// Computes the maximum numbers of topics that match the course criteria
    ///
    /// ## Parameters
    /// - `topics: &[course_design::Topic]` a list of topics `(interest: i64, difficulty: i64)`
    ///
    /// ## Returns
    /// An `usize` with the maximum numer of topics a course can cover
    ///
    /// ## Constraints
    /// A course contains an ordered list of topics, each one having some `interest` and `difficulty`
    /// factors. For each topic `i` in the (ordered) list, `topic[i].interest > topic[i-1].interest`
    /// and `topic[i].difficulty > topic[i-1].difficulty`
    ///
    /// ## Strategy
    /// We notice that the problem requirements and constraints match with the definition of
    /// the `longest_increasing_subsequence` problem dependant on 2 dimensions.
    ///
    /// We can reduce this instance to a one parameter `LIS` instance by sorting all items in
    /// ascendent order for the interest parameter then decreasing for the difficulty parameter
    /// (if the parameter allows for duplicate interest parameters)
    /// This is because, since we're processing topics by interest order, we can incapsulate the
    /// extension of the current LIS as just checking if the current difficulty is greater than
    /// the difficulty of the last element of the LIS. Since we processed harder difficulties first
    ///  with the same interest level, any subsequence we can extend will have its last element from
    /// a previous interest level (which satisfies the increasing interest constraint).
    ///
    /// ### Solving Longest Increasing Subsequence 1D
    /// In this particular implementation we use an approach based on binary search, so use a support
    /// vector that at the end will contain the longest index subsequence.
    ///
    /// We start processing the list, while looking for each item we process inside the support vector,
    /// using binary search, for each item, the binary search (built-in) returns the first index where
    /// we can insert the item to keep the list sorted. If the index is internal to the array then we
    /// insert the current item at that position, while if it's out of bounds we append the new item.
    ///
    /// This results us, in updating the list with newfound elements, appending them so extending the
    /// subsequence.
    ///
    /// ## Complexity
    /// We solve the 1-D LIS instance in `O(n(log(n)))` time with `O(n)` additional memory
    /// for the dp array.
    ///
    /// To reduce the original problem, we take `O(n(log(n)))` for sorting and `O(n)`
    /// memory
    ///
    /// So the overall complexity is `O(n(log(n)))` time and `O(n)` space.
    ///
    pub fn design_binary(topics: &[Topic]) -> usize {
        let mut topics = topics.to_vec();
        topics.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

        let mut support: Vec<i64> = Vec::new();

        for (_, d) in topics {
            if let Err(idx) = support.binary_search(&d) {
                if idx >= support.len() {
                    support.push(d);
                } else {
                    support[idx] = d;
                }
            }
        }
        support.len()
    }

    pub fn parse_input(input: &str) -> Vec<(i64, i64)> {
        let mut lines = input.lines();

        // Parse the first line to get the count
        let count = lines
            .next()
            .expect("Missing count line")
            .trim()
            .parse::<usize>()
            .expect("Count is not a valid number");

        // Parse the next `count` lines into tuples
        lines
            .take(count)
            .map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().expect("Invalid number in tuple"));
                let first = nums.next().expect("Missing first number in tuple");
                let second = nums.next().expect("Missing second number in tuple");
                (first, second)
            })
            .collect()
    }

    pub fn parse_output(output: &str) -> usize {
        let line = output
            .lines()
            .next()
            .expect("File must contain a single unsigned number");
        line.parse().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::course_design::*;
    use std::env::current_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    fn test_path(name: &str) -> PathBuf {
        current_dir().unwrap().join("test_design").join(name)
    }

    fn test_files(input_name: &str, output_name: &str) {
        let (input_path, output_path) = (test_path(input_name), test_path(output_name));

        let (input_str, output_str) = (
            read_to_string(input_path).unwrap(),
            read_to_string(output_path).unwrap(),
        );

        let topics = parse_input(&input_str);
        let expected_max = parse_output(&output_str);

        assert_eq!(design_quadratic(&topics), expected_max);
        assert_eq!(design_binary(&topics), expected_max);
    }

    #[test]
    fn test_io_design_course() {
        for i in 0..=10 {
            let input = format!("input{i}.txt");
            let output = format!("output{i}.txt");
            test_files(&input, &output)
        }
    }
}
