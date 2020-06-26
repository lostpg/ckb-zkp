// The following code is from (scipr-lab's zexe)[https://github.com/scipr-lab/zexe] and thanks for their work

use crate::tests::fields::{field_test, primefield_test};
use math::test_rng;
use rand::Rng;

use crate::edwards_bls12::{Fq, Fr};

#[test]
fn test_fr() {
    let mut rng = test_rng();
    let a: Fr = rng.gen();
    let b: Fr = rng.gen();
    field_test(a, b);
    primefield_test::<Fr>();
}

#[test]
fn test_fq() {
    let mut rng = test_rng();
    let a: Fq = rng.gen();
    let b: Fq = rng.gen();
    field_test(a, b);
    primefield_test::<Fq>();
}
