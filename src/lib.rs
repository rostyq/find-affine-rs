use nalgebra::{Affine2, Point2, RealField, SMatrix, Vector3};

#[inline]
pub fn from_smatrices<T: RealField>(
    from: SMatrix<T, 3, 3>,
    to: SMatrix<T, 2, 3>,
    eps: T,
) -> Result<Affine2<T>, &'static str> {
    let mut transform = (to * from.pseudo_inverse(eps)?).fixed_resize::<3, 3>(T::zero());

    transform[(2, 2)] = T::one();

    Ok(Affine2::from_matrix_unchecked(transform))
}

#[inline]
pub fn from_point_slices<T: RealField>(
    from: &[Point2<T>],
    to: &[Point2<T>],
    eps: T,
) -> Result<Affine2<T>, &'static str> {
    from_point_arrays(
        from.try_into()
            .map_err(|_| "Cannot get three points from `from` slice")?,
        to.try_into()
            .map_err(|_| "Cannot get three points from `to` slice")?,
        eps,
    )
}

#[inline]
pub fn from_point_arrays<T: RealField>(
    from: &[Point2<T>; 3],
    to: &[Point2<T>; 3],
    eps: T,
) -> Result<Affine2<T>, &'static str> {
    let points: [Vector3<T>; 3] = unsafe {
        from.iter()
            .map(|p| p.to_homogeneous())
            .collect::<Vec<Vector3<T>>>()
            .try_into()
            .unwrap_unchecked()
    };

    let from_matrix =
        SMatrix::<T, 3, 3>::from_iterator(points.iter().flatten().map(|v| v.to_owned()));

    let to_matrix = SMatrix::<T, 2, 3>::from_iterator(
        to.iter()
            .map(|p| p.coords.iter())
            .flatten()
            .map(|v| v.to_owned()),
    );

    from_smatrices(from_matrix, to_matrix, eps)
}
