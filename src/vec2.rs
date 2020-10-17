pub fn sum(vec2s: &[[f32; 2]]) -> [f32; 2] {
    vec2s
        .iter()
        .fold([0.0, 0.0], |v1, v2| [v1[0] + v2[0], v1[1] + v2[1]])
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum() {
        let vec2_a = [1.0, 2.0];
        let vec2_b = [3.0, 4.0];

        assert_eq!(super::sum(&[vec2_a, vec2_b]), [4.0, 6.0]);
    }
}
