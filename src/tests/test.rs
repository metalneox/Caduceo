#[cfg(test)]
pub mod tests {
    use crate::crypto::ciphers::*;

    #[test]
    fn easy() {
        assert_eq!(rot13("aaa"), "nnn");
    }
    #[test]
    fn middle() {
        assert_eq!(rot13("a aa"), "n nn");
    }
    #[test]
    fn vcrypt() {
        assert_eq!(
            vigenere_crypt("rapportoimmediato", "verme"),
            "megbsmxfuqhiuueos"
        );
    }

    #[test]
    fn vdecrypt() {
        assert_eq!(
            vigenere_decrypt("megbsmxfuqhiuueos", "verme"),
            "rapportoimmediato"
        );
    }

    #[test]
    fn carbotest() {
        assert_eq!(carbonaro("ciao"), "geoa");
    }

    #[test]
    fn atbashtest() {
        assert_eq!(atbash("ciao"), "uozi")
    }
    //TODO Meglio un array di ritorno che una stringa
    #[test]
    fn polybius_crypt_test() {
        assert_eq!(polybius_crypt("ciao", 5), "13241134")
    }
    #[test]
    fn polybius_decrypt_test() {
        assert_eq!(polybius_decrypt("13241134", 5), "ciao")
    }

    #[test]
    fn nihilist_crypt_test() {
                                 //text    //key
        assert_eq!(nihilist_crypt("prova", "ciao"), "65 46 57 94 37")
    }

    #[test]
    fn nihilist_decrypt_test() {
                                 //text crypt    //key
        assert_eq!(nihilist_decrypt("65 46 57 94 37", "ciao"), "prova")
    }




}
