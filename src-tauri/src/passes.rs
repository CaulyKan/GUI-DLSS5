/// Feed each DLSS result into the next pass using separate, reusable buffers.
pub(super) fn process_passes(
    input: &[u8],
    output: &mut Vec<u8>,
    intermediate: &mut Vec<u8>,
    count: i32,
    reset: bool,
    mut process: impl FnMut(&[u8], &mut [u8], bool) -> bool,
) -> Result<(), String> {
    let count = count.clamp(1, 5);
    if count > 1 {
        intermediate.resize(output.len(), 0);
    }
    // One native session is shared across stages. Multi-pass stages must not
    // reuse another stage's temporal history, including the next frame's pass 1.
    let reset = reset || count > 1;
    for pass in 1..=count {
        let source = if pass == 1 {
            input
        } else {
            std::mem::swap(output, intermediate);
            intermediate.as_slice()
        };
        if !process(source, output.as_mut_slice(), reset) {
            return Err(format!("DLSS 第 {pass}/{count} 次 Pass 未生成画面。"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::process_passes;

    #[test]
    fn chains_outputs_for_every_count_and_reuses_buffers_across_frames() {
        let mut output = vec![0; 4];
        let mut intermediate = Vec::new();
        for count in 1..=5 {
            for input in [[10; 4], [30; 4]] {
                let mut calls = 0;
                process_passes(
                    &input,
                    &mut output,
                    &mut intermediate,
                    count,
                    false,
                    |source, destination, reset| {
                        assert_ne!(source.as_ptr(), destination.as_ptr());
                        assert_eq!(source, &[input[0] + calls; 4]);
                        assert_eq!(reset, count > 1);
                        for (out, value) in destination.iter_mut().zip(source) {
                            *out = value + 1;
                        }
                        calls += 1;
                        true
                    },
                )
                .unwrap();
                assert_eq!(i32::from(calls), count);
                assert_eq!(output, vec![input[0] + count as u8; 4]);
            }
        }
    }

    #[test]
    fn clamps_count_and_preserves_single_pass_reset() {
        for (count, expected) in [(-3, 1), (0, 1), (8, 5)] {
            let mut calls = 0;
            process_passes(
                &[1; 4],
                &mut vec![0; 4],
                &mut Vec::new(),
                count,
                true,
                |_, _, reset| {
                    assert!(reset);
                    calls += 1;
                    true
                },
            )
            .unwrap();
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn stops_on_failure_and_recovers_after_resize() {
        let mut output = vec![0; 4];
        let mut intermediate = Vec::new();
        let mut calls = 0;
        let error = process_passes(
            &[1; 4],
            &mut output,
            &mut intermediate,
            5,
            false,
            |_, _, _| {
                calls += 1;
                calls < 2
            },
        )
        .unwrap_err();
        assert_eq!(calls, 2);
        assert!(error.contains("2/5"));
        output.resize(8, 0);
        process_passes(
            &[7; 8],
            &mut output,
            &mut intermediate,
            3,
            true,
            |source, destination, _| {
                destination.copy_from_slice(source);
                true
            },
        )
        .unwrap();
        assert_eq!(output, vec![7; 8]);
    }
}
