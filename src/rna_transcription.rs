#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let bases = ['G', 'C', 'T', 'A'];
        if let Some(pos) = is_valid_base(dna, bases) {
            return Err(pos);
        }

        Ok(Dna {
            strand: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna_strand = self
            .strand
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!("uknown base found"),
            })
            .collect::<String>();
        Rna { strand: rna_strand }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let bases = ['G', 'C', 'U', 'A'];
        if let Some(pos) = is_valid_base(rna, bases) {
            return Err(pos);
        }

        Ok(Rna {
            strand: rna.to_string(),
        })
    }
}

fn is_valid_base(strand: impl AsRef<str>, bases: [char; 4]) -> Option<usize> {
    strand.as_ref().chars().position(|c| !bases.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rna_sequence() {
        let input = "";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_cytosine_is_guanine() {
        let input = "C";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("G").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_guanine_is_cytosine() {
        let input = "G";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("C").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_thymine_is_adenine() {
        let input = "T";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("A").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_adenine_is_uracil() {
        let input = "A";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("U").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement() {
        let input = "ACGTGGTCTTAA";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("UGCACCAGAAUU").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_dna_input() {
        let input = "U";
        let output = Dna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_dna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Dna::new(input);
        let expected = Err(4);
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_rna_input() {
        let input = "T";
        let output = Rna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_rna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Rna::new(input);
        let expected = Err(3);
        assert_eq!(output, expected);
    }
}
