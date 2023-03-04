use crate::error::DaftError;
use crate::{error::DaftResult, series::Series, with_match_comparable_daft_types};

use crate::array::BaseArray;

impl Series {
    pub fn argsort(&self, descending: bool) -> DaftResult<Series> {
        with_match_comparable_daft_types!(self.data_type(), |$T| {
            let downcasted = self.downcast::<$T>()?;
            Ok(downcasted.argsort::<UInt64Type>(descending)?.into_series())
        })
    }

    pub fn argsort_multikey(sort_keys: &[Series], descending: &[bool]) -> DaftResult<Series> {
        if sort_keys.len() != descending.len() {
            return Err(DaftError::ValueError(format!(
                "sort_keys and descending length must match, got {} vs {}",
                sort_keys.len(),
                descending.len()
            )));
        }

        if sort_keys.len() == 1 {
            return sort_keys
                .first()
                .unwrap()
                .argsort(*descending.first().unwrap());
        }

        let first = sort_keys.first().unwrap();
        with_match_comparable_daft_types!(first.data_type(), |$T| {
            let downcasted = first.downcast::<$T>()?;
            let result = downcasted.argsort_multikey::<UInt64Type>(&sort_keys[1..], descending)?;
            Ok(result.into_series())
        })
    }

    pub fn sort(&self, descending: bool) -> DaftResult<Self> {
        with_match_comparable_daft_types!(self.data_type(), |$T| {
            let downcasted = self.downcast::<$T>()?;
            Ok(downcasted.sort(descending)?.into_series())
        })
    }
}
