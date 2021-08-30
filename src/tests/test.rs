#[cfg(test)]
pub mod tests {
    use crate::crypto::ciphers::*;

    #[test]
    fn easy() {
        assert_eq!(rot13("aaa"), "nnn");
    }
    #[test]
    fn middle(){
        assert_eq!(rot13("a aa"), "n nn");
    }
    #[test]
    fn vcrypt(){
        assert_eq!(vigenere_crypt("rapportoimmediato","verme"), "megbsmxfuqhiuueos");
    }
    
    #[test]
    fn vdecrypt(){
        assert_eq!(vigenere_decrypt("megbsmxfuqhiuueos","verme"), "rapportoimmediato");
    }

    #[test]
    fn carbotest(){
        assert_eq!(carbonaro("ciao"),"geoa");
    }

    #[test]
    fn atbashtest(){
        assert_eq!(atbash("ciao"),"uozi")
    }
    //todo test 
    #[test]
    fn polybius_crypt_test(){
        assert_eq!(polybius_crypt("ciao",5),"13241134")
    }
    //FIXME Su una grandezza di 5 è possibile che i test si buggano su i e j che ha la stessa chiave
    #[test]
    fn polybius_decrypt_test(){
        assert_eq!(polybius_decrypt("13241134",5),"cjao")
    }    

}