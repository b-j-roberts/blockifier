use std::ops::Sub;

use crate::bouncer::{BouncerWeights, BuiltinCount};

#[test]
fn test_block_weights_sub_checked() {
    let max_bouncer_weights = BouncerWeights {
        gas: 10,
        n_steps: 10,
        message_segment_length: 10,
        state_diff_size: 10,
        builtin_count: BuiltinCount {
            bitwise: 10,
            ecdsa: 10,
            ec_op: 10,
            keccak: 10,
            output: 10,
            pedersen: 10,
            poseidon: 10,
            range_check: 10,
        },
    };

    let bouncer_weights = BouncerWeights {
        gas: 7,
        n_steps: 0,
        message_segment_length: 10,
        state_diff_size: 7,
        builtin_count: BuiltinCount {
            bitwise: 6,
            ecdsa: 7,
            ec_op: 7,
            keccak: 8,
            output: 7,
            pedersen: 7,
            poseidon: 9,
            range_check: 10,
        },
    };

    let result = max_bouncer_weights.checked_sub(bouncer_weights).unwrap();
    let difference_bouncer_weights = max_bouncer_weights.sub(bouncer_weights);
    assert_eq!(result, difference_bouncer_weights);

    let bouncer_weights_exceeds_max = BouncerWeights {
        gas: 5,
        n_steps: 5,
        message_segment_length: 5,
        state_diff_size: 5,
        builtin_count: BuiltinCount {
            bitwise: 11,
            ecdsa: 5,
            ec_op: 5,
            keccak: 5,
            output: 5,
            pedersen: 5,
            poseidon: 5,
            range_check: 5,
        },
    };

    let result = max_bouncer_weights.checked_sub(bouncer_weights_exceeds_max);
    assert!(result.is_none());
}
