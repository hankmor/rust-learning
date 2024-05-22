// add 方法将两个参数想家，返回一个 usize 结果
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 定义一个模块，用来测试 add 方法
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
