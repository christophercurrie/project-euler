fn euler015(dim: usize) -> u64 {
    let vertexes = dim + 1;
    let mut data = vec![vec![1; vertexes]; vertexes];

    for x in (0..vertexes).rev() {
        for y in (0..vertexes).rev() {
            if x + 1 == vertexes && y + 1 == vertexes {
                continue;
            }

            let mut paths = 0;
            if x + 1 < vertexes {
                paths += data[x + 1][y];
            }
            if y + 1 < vertexes {
                paths += data[x][y + 1];
            }
            data[x][y] = paths;
        }
    }

    data[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler015() {
        assert_eq!(euler015(2), 6);
        assert_eq!(euler015(20), 137_846_528_820);
    }
}
