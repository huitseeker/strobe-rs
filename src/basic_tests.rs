/*
   How to run the Python 2 code listed below:
     1. Download a copy of strobe (https://sourceforge.net/p/strobe) into the root directory of
        this crate and name the folder `orig-strobe`
     2. Make a Python file with any of the code below in the root directory of this crate.
     3. Run `python2 FILE`
*/

#[cfg(test)]
use crate::{
    keccak::KECCAK_BLOCK_SIZE,
    prelude::*,
    strobe::{SecParam, Strobe},
};

/*
    # The Python 2 code used to generate this test vector:
    import sys
    sys.path.insert(0, "./orig-strobe/python")
    from Strobe.Strobe import Strobe
    s = Strobe("", security=128)
    print("[{}]".format(', '.join(map("0x{:02x}".format, s.st))))
*/
#[test]
fn test_init_128() {
    let s = Strobe::new(b"", SecParam::B128);
    let initial_st = s.st.0;
    let expected_st: &[u8; 8 * KECCAK_BLOCK_SIZE] = &[
        0x9c, 0x7f, 0x16, 0x8f, 0xf8, 0xfd, 0x55, 0xda, 0x2a, 0xa7, 0x3c, 0x23, 0x55, 0x65, 0x35,
        0x63, 0xdc, 0x0c, 0x47, 0x5c, 0x55, 0x15, 0x26, 0xf6, 0x73, 0x3b, 0xea, 0x22, 0xf1, 0x6c,
        0xb5, 0x7c, 0xd3, 0x1f, 0x68, 0x2e, 0x66, 0x0e, 0xe9, 0x12, 0x82, 0x4a, 0x77, 0x22, 0x01,
        0xee, 0x13, 0x94, 0x22, 0x6f, 0x4a, 0xfc, 0xb6, 0x2d, 0x33, 0x12, 0x93, 0xcc, 0x92, 0xe8,
        0xa6, 0x24, 0xac, 0xf6, 0xe1, 0xb6, 0x00, 0x95, 0xe3, 0x22, 0xbb, 0xfb, 0xc8, 0x45, 0xe5,
        0xb2, 0x69, 0x95, 0xfe, 0x7d, 0x7c, 0x84, 0x13, 0x74, 0xd1, 0xff, 0x58, 0x98, 0xc9, 0x2e,
        0xe0, 0x63, 0x6b, 0x06, 0x72, 0x73, 0x21, 0xc9, 0x2a, 0x60, 0x39, 0x07, 0x03, 0x53, 0x49,
        0xcc, 0xbb, 0x1b, 0x92, 0xb7, 0xb0, 0x05, 0x7e, 0x8f, 0xa8, 0x7f, 0xce, 0xbc, 0x7e, 0x88,
        0x65, 0x6f, 0xcb, 0x45, 0xae, 0x04, 0xbc, 0x34, 0xca, 0xbe, 0xae, 0xbe, 0x79, 0xd9, 0x17,
        0x50, 0xc0, 0xe8, 0xbf, 0x13, 0xb9, 0x66, 0x50, 0x4d, 0x13, 0x43, 0x59, 0x72, 0x65, 0xdd,
        0x88, 0x65, 0xad, 0xf9, 0x14, 0x09, 0xcc, 0x9b, 0x20, 0xd5, 0xf4, 0x74, 0x44, 0x04, 0x1f,
        0x97, 0xb6, 0x99, 0xdd, 0xfb, 0xde, 0xe9, 0x1e, 0xa8, 0x7b, 0xd0, 0x9b, 0xf8, 0xb0, 0x2d,
        0xa7, 0x5a, 0x96, 0xe9, 0x47, 0xf0, 0x7f, 0x5b, 0x65, 0xbb, 0x4e, 0x6e, 0xfe, 0xfa, 0xa1,
        0x6a, 0xbf, 0xd9, 0xfb, 0xf6,
    ];

    assert_eq!(&initial_st[..], &expected_st[..]);
}

/*
    # The Python 2 code used to generate this test vector:
    import sys
    sys.path.insert(0, "./orig-strobe/python")
    from Strobe.Strobe import Strobe
    s = Strobe("", security=256)
    print("[{}]".format(', '.join(map("0x{:02x}".format, s.st))))
*/
#[test]
fn test_init_256() {
    let s = Strobe::new(b"", SecParam::B256);
    let initial_st = s.st.0;
    let expected_st: &[u8; 8 * KECCAK_BLOCK_SIZE] = &[
        0x37, 0xc1, 0x15, 0x06, 0xed, 0x61, 0xe7, 0xda, 0x7c, 0x1a, 0x2f, 0x2c, 0x1f, 0x49, 0x74,
        0xb0, 0x71, 0x66, 0xc2, 0xea, 0x7f, 0x62, 0xec, 0xa6, 0xe0, 0x36, 0xc1, 0x6e, 0xae, 0x39,
        0xb4, 0xdf, 0x3a, 0x06, 0x11, 0xf1, 0x36, 0xc7, 0x33, 0x94, 0x31, 0x13, 0x2c, 0xdb, 0x18,
        0x03, 0x08, 0xc0, 0x53, 0x61, 0xab, 0xf7, 0xb9, 0xc6, 0x89, 0x49, 0xab, 0x1e, 0x5c, 0x0b,
        0xbf, 0xab, 0x0a, 0xb0, 0x66, 0xa0, 0x13, 0x96, 0xdb, 0x8d, 0xb1, 0x26, 0x02, 0x0c, 0xf7,
        0x96, 0xb2, 0x3f, 0x0e, 0xe1, 0xcf, 0x40, 0xda, 0x8f, 0x8b, 0xfc, 0x34, 0x27, 0x34, 0x14,
        0x4a, 0x64, 0x08, 0x29, 0x44, 0x5a, 0x67, 0xab, 0x3e, 0x15, 0x46, 0xc0, 0x97, 0xe3, 0x23,
        0xd3, 0xda, 0xe7, 0xc6, 0x2e, 0x62, 0xd3, 0xdd, 0xae, 0x90, 0x98, 0x31, 0xa1, 0x64, 0x9c,
        0xd8, 0x07, 0x97, 0x7b, 0x5e, 0x44, 0x88, 0xae, 0x42, 0xfc, 0x36, 0xec, 0x2c, 0x5a, 0x78,
        0x0d, 0x52, 0xa3, 0x22, 0xa6, 0xe9, 0xbe, 0xff, 0x73, 0x89, 0xcb, 0x8f, 0xe7, 0x6a, 0xb5,
        0x5d, 0xc6, 0xa0, 0x60, 0xa7, 0x22, 0xb9, 0x64, 0xb6, 0xe8, 0xfe, 0x8b, 0xb5, 0xb9, 0x1a,
        0x9b, 0xbc, 0x61, 0xc0, 0x86, 0x7e, 0x6d, 0xfc, 0x5b, 0x5c, 0x6d, 0xd5, 0xb5, 0xa7, 0x26,
        0xc9, 0x18, 0xe4, 0x0b, 0xe9, 0xb1, 0xcf, 0xa7, 0xef, 0xa6, 0x92, 0xf5, 0x05, 0xdc, 0xac,
        0xde, 0x80, 0x03, 0xe8, 0xbb,
    ];

    assert_eq!(&initial_st[..], &expected_st[..]);
}

/*
    # The Python 2 code used to generate this test vector:
    import sys
    sys.path.insert(0, "./orig-strobe/python")
    from Strobe.Strobe import Strobe
    s = Strobe("seqtest", security=256)

    s.prf(10)
    s.ad("Hello")
    s.send_enc("World")
    s.send_clr("foo")
    s.ratchet()
    s.recv_clr("bar")
    s.recv_enc("baz")
    for i in xrange(100):
        s.send_enc("X"*i)
    s.prf(123)
    s.send_mac()

    print("[{}]".format(', '.join(map("0x{:02x}".format, s.st))))
*/
#[test]
fn test_seq() {
    let mut s = Strobe::new(b"seqtest", SecParam::B256);

    let mut buf = [0u8; 10];
    s.prf(&mut buf[..], false);

    s.ad(b"Hello", false);

    let mut buf = b"World".to_vec();
    s.send_enc(buf.as_mut_slice(), false);

    s.send_clr(b"foo", false);
    s.ratchet(32, false);
    s.recv_clr(b"bar", false);

    let mut buf = b"baz".to_vec();
    s.recv_enc(buf.as_mut_slice(), false);

    for i in 0..100 {
        let mut buf = vec![b'X'; i];
        s.send_enc(buf.as_mut_slice(), false);
    }

    let mut buf = [0u8; 123];
    s.prf(&mut buf[..], false);

    let mut buf = [0u8; 16];
    s.send_mac(&mut buf[..], false);

    let final_st = s.st.0;
    let expected_st = [
        0xdf, 0x7a, 0x38, 0x71, 0x06, 0xcc, 0x24, 0x82, 0x11, 0x31, 0x60, 0x43, 0xa9, 0xf0, 0xf5,
        0xd0, 0x49, 0xc2, 0xce, 0xd3, 0x85, 0xfc, 0x9e, 0xa8, 0x0e, 0xc1, 0x46, 0xa4, 0xa1, 0x96,
        0x02, 0x30, 0x78, 0xe6, 0x16, 0x62, 0x50, 0x1b, 0xab, 0x23, 0x5d, 0xcb, 0x85, 0x34, 0x3a,
        0x67, 0xc6, 0x6c, 0xd8, 0x79, 0x45, 0xee, 0x2b, 0xaa, 0xc0, 0x09, 0x45, 0xc7, 0xf6, 0x42,
        0xd9, 0xbc, 0x43, 0xe1, 0xd5, 0x2c, 0x6e, 0x71, 0x6f, 0xfa, 0x9a, 0x39, 0x9d, 0x11, 0xfd,
        0x62, 0xfb, 0x15, 0x04, 0x85, 0xf9, 0xe3, 0xc1, 0x24, 0x95, 0x04, 0x84, 0x95, 0x3c, 0x74,
        0x38, 0x3d, 0x5e, 0x08, 0x87, 0x64, 0xa3, 0x57, 0xdd, 0xb0, 0x40, 0x5b, 0x40, 0x25, 0x93,
        0xb8, 0x3a, 0x75, 0x1d, 0xb7, 0xdf, 0xc4, 0x34, 0x4d, 0xfa, 0x94, 0xc6, 0x98, 0x13, 0xb3,
        0x75, 0xf2, 0xdc, 0xd0, 0xe3, 0xe9, 0x44, 0xba, 0xfd, 0x98, 0x13, 0xc1, 0x59, 0xc7, 0x46,
        0xa7, 0xb0, 0x65, 0x70, 0x20, 0x3d, 0x56, 0xeb, 0x84, 0x18, 0x1c, 0xca, 0x5b, 0x7a, 0xe4,
        0xad, 0x3a, 0x57, 0x6b, 0x40, 0x80, 0x29, 0x0c, 0x63, 0x11, 0xd8, 0x6f, 0x89, 0xb8, 0x32,
        0xf0, 0xb1, 0xde, 0x8c, 0x0a, 0x4f, 0x00, 0x90, 0x16, 0x0d, 0xc1, 0x9f, 0xd4, 0x69, 0x9c,
        0x56, 0xb1, 0xd8, 0x9e, 0xc0, 0x8d, 0x40, 0x7a, 0x36, 0xe3, 0xb3, 0x9c, 0xd4, 0x91, 0x17,
        0xd7, 0xed, 0x4c, 0x4b, 0xa5,
    ];

    assert_eq!(&final_st[..], &expected_st[..]);
}

/*
    # The Python 2 code used to generate these test vectors:
    import sys
    sys.path.insert(0, "./orig-strobe/python")
    from Strobe.Strobe import Strobe
    I,A,C,T,M,K = 1<<0, 1<<1, 1<<2, 1<<3, 1<<4, 1<<5
    s = Strobe("metadatatest", security=256)

    m = s.key("key", meta_flags=A|T|M, metadata="meta1")
    m += s.prf(10, meta_flags=I|A|C|M, metadata=10)
    m += s.send_enc("pt", meta_flags=A|T|M, metadata="meta3")

    print("accumulated metadata == [{}]".format(', '.join(map("0x{:02x}".format, m))))
    print("state == [{}]".format(', '.join(map("0x{:02x}".format, s.st))))
*/
#[test]
fn test_metadata() {
    // We will accumulate output over 3 operations and 3 meta-operations
    let mut s = Strobe::new(b"metadatatest", SecParam::B256);
    let mut output = Vec::new();

    let buf = b"meta1";
    s.meta_send_clr(buf, false);
    output.extend_from_slice(buf);

    // This does not output anything
    s.key(b"key", false);

    let mut buf = [0u8; 10];
    s.meta_prf(&mut buf, false);
    output.extend_from_slice(&buf[..]);

    // We don't have to re-zero the buffer. Our internal special-casing for PRF does this for us
    s.prf(&mut buf, false);
    output.extend_from_slice(&buf[..]);

    let buf = b"meta3";
    s.meta_send_clr(buf, false);
    output.extend(buf);

    let mut buf = b"pt".to_vec();
    s.send_enc(buf.as_mut_slice(), false);
    output.extend(buf);

    let expected_output = [
        0x6d, 0x65, 0x74, 0x61, 0x31, 0xa7, 0xe5, 0x96, 0xe0, 0x8f, 0x39, 0x19, 0x3c, 0x4f, 0x84,
        0xdb, 0x00, 0xbb, 0xce, 0xbb, 0xf3, 0x7e, 0xc6, 0x33, 0x8b, 0x6d, 0x65, 0x74, 0x61, 0x33,
        0xe9, 0x0b,
    ];
    let expected_st = [
        0xe9, 0x0b, 0x29, 0xad, 0x32, 0x0c, 0x27, 0x53, 0x07, 0x48, 0xcd, 0x38, 0xde, 0xf7, 0x23,
        0xb0, 0x54, 0x21, 0x14, 0xae, 0xd1, 0xfc, 0x55, 0xd0, 0xc6, 0xc2, 0x58, 0x85, 0xaa, 0x5d,
        0x30, 0x30, 0x88, 0xb9, 0x6b, 0x55, 0xf0, 0x01, 0xbd, 0x30, 0xc4, 0xd5, 0x00, 0x72, 0xad,
        0x58, 0xad, 0x08, 0xbc, 0x0c, 0x7b, 0x8f, 0xad, 0xe5, 0x02, 0x57, 0xa9, 0xe4, 0xbe, 0xb0,
        0x1a, 0x96, 0x44, 0xc6, 0x25, 0x9d, 0x58, 0x30, 0x85, 0xf4, 0xee, 0xe0, 0xdd, 0x32, 0x39,
        0x18, 0x8d, 0x46, 0x02, 0xa2, 0x9a, 0x64, 0x3d, 0x7a, 0x4e, 0xd0, 0xaa, 0x57, 0xf1, 0x97,
        0x9e, 0xb5, 0xca, 0x18, 0x6c, 0xd2, 0x2b, 0x4f, 0xb6, 0x78, 0x30, 0x2f, 0xe1, 0xb0, 0x34,
        0x10, 0x21, 0xdc, 0xd6, 0xdd, 0x63, 0x2f, 0x14, 0x23, 0x41, 0x78, 0xe4, 0x98, 0x9c, 0x5c,
        0x8a, 0xae, 0x00, 0x31, 0xb7, 0xa9, 0x90, 0x16, 0x91, 0x41, 0x38, 0x0a, 0xc4, 0xe2, 0x3f,
        0x39, 0x4d, 0x5e, 0xc7, 0x58, 0x59, 0x5d, 0xe5, 0x31, 0xdc, 0x7c, 0x0b, 0x06, 0xca, 0x8b,
        0x95, 0x75, 0x31, 0x70, 0x9a, 0xee, 0x42, 0xf3, 0x2c, 0xef, 0x88, 0x1c, 0x6e, 0xc9, 0x9b,
        0x69, 0xe5, 0xaf, 0xfc, 0x30, 0x93, 0x51, 0x29, 0x17, 0x93, 0x46, 0x21, 0xc5, 0x6c, 0xc9,
        0x95, 0xd4, 0x58, 0x7c, 0xeb, 0x5a, 0x9d, 0x90, 0xa6, 0x7d, 0x5d, 0x36, 0xf6, 0x3f, 0xa6,
        0xde, 0xb2, 0x48, 0xe2, 0x88,
    ];

    assert_eq!(&output, &expected_output);
    assert_eq!(&s.st.0[..], &expected_st[..]);
}

// Test that streaming in data using the `more` flag works as expected
#[test]
fn test_streaming() {
    // Compute a few things without breaking up their inputs
    let one_shot_st: Vec<u8> = {
        let mut s = Strobe::new(b"streamingtest", SecParam::B256);
        s.ad(b"mynonce", false);

        let mut buf = b"hello there".to_vec();
        s.recv_enc(buf.as_mut_slice(), false);

        let mut buf = [0u8; 16];
        s.send_mac(&mut buf[..], false);

        s.st.0.to_vec()
    };
    // Now do the same thing but stream the inputs
    let streamed_st: Vec<u8> = {
        let mut s = Strobe::new(b"streamingtest", SecParam::B256);
        s.ad(b"my", false);
        s.ad(b"nonce", true);

        let mut buf = b"hello".to_vec();
        s.recv_enc(buf.as_mut_slice(), false);

        let mut buf = b" there".to_vec();
        s.recv_enc(buf.as_mut_slice(), true);

        let mut buf = [0u8; 10];
        s.send_mac(&mut buf[..], false);
        let mut buf = [0u8; 6];
        s.send_mac(&mut buf[..], true);
        s.st.0.to_vec()
    };

    assert_eq!(one_shot_st, streamed_st);
}

// Test that decrypt(encrypt(msg)) == msg
#[test]
fn test_enc_correctness() {
    let orig_msg = b"Hello there";
    let mut tx = Strobe::new(b"enccorrectnesstest", SecParam::B256);
    let mut rx = Strobe::new(b"enccorrectnesstest", SecParam::B256);

    tx.key(b"the-combination-on-my-luggage", false);
    rx.key(b"the-combination-on-my-luggage", false);

    // Encrypt and decrypt the original message
    let mut buf = orig_msg.to_vec();
    tx.send_enc(buf.as_mut_slice(), false);
    rx.recv_enc(buf.as_mut_slice(), false);

    assert_eq!(orig_msg, buf.as_slice());
}

// Test that recv_mac(send_mac()) doesn't error, and recv_mac(otherstuff) does error
#[test]
fn test_mac_correctness() {
    let mut tx = Strobe::new(b"maccorrectnesstest", SecParam::B256);
    let mut rx = Strobe::new(b"maccorrectnesstest", SecParam::B256);

    // Just do some stuff with the state

    tx.key(b"secretsauce", false);
    let mut msg = b"attack at dawn".to_vec();
    tx.send_enc(msg.as_mut_slice(), false);

    let mut mac = [0u8; 16];
    tx.send_mac(&mut mac[..], false);

    rx.key(b"secretsauce", false);
    rx.recv_enc(&mut msg[..], false);

    // Test that valid MACs are accepted
    let mut rx_copy = rx.clone();
    let good_res = rx_copy.recv_mac(&mut mac[..], false);
    assert!(good_res.is_ok());

    // Test that invalid MACs are rejected
    let mut bad_mac = {
        let mut tmp = mac.to_vec();
        tmp.push(0);
        tmp
    };
    let bad_res = rx.recv_mac(&mut bad_mac[..], false);
    assert!(bad_res.is_err());
}
