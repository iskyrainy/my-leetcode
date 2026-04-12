fn part(nums: &mut [i32], left: usize, right: usize) {
    let rand_i = left + rand::random_range(0..=(right - left));
    let pivot = nums[rand_i];
    nums.swap(left, rand_i);
    let (mut i, mut j) = (left + 1, right);
    loop {
        while i <= j && nums[i] < pivot {
            i += 1;
        }
        while i <= j && nums[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }
    nums.swap(left, j);
    if j > left {
        part(nums, left, j - 1);
    }
    if j < right {
        part(nums, j + 1, right);
    }
}

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() - 1;
    part(&mut nums, 0, n);
    nums.sort_unstable();
    nums
}

#[cfg(test)]
mod test {
    use crate::lc::lc_912::sort_array;

    #[test]
    fn test_sort_array_1() {
        assert_eq!(vec![1, 2, 3, 5], sort_array(vec![5, 2, 3, 1]));
    }
}
