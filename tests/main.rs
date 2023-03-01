#[cfg(test)]
mod tests {
    use nalgebra::{Affine2, Point2, SMatrix};

    use find_affine;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_find_affine() {
        let test = Affine2::from_matrix_unchecked(SMatrix::<f32, 3, 3>::new(
            3.07692308,
            8.46153846,
            -546.15384615,
            -1.15384615,
            -6.92307692,
            392.30769231,
            0.0,
            0.0,
            1.0,
        ));

        let from_points = vec![
            Point2::new(40.0, 50.0),
            Point2::new(100.0, 40.0),
            Point2::new(150.0, 10.0),
        ];

        let to_points: Vec<Point2<f32>> = from_points
            .iter()
            .map(|point| test.transform_point(point))
            .collect();

        let affine = find_affine::from_point_arrays(
            &from_points.try_into().unwrap(),
            &to_points.try_into().unwrap(),
            0.0001,
        )
        .unwrap();
        assert_abs_diff_eq!(test, affine, epsilon = 0.001);
    }
}
