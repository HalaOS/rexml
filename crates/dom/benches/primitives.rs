fn main() {
    divan::main();
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod qname {
    use std::hint::black_box;

    use rexml_dom::QName;

    #[divan::bench]
    fn parse() {
        black_box(QName::try_from("h11111:hello").unwrap());
    }
}
