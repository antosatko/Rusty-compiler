//! DNDS Syntax specification
pub mod dictionary {
    /// Used to specify expected kind of word
    pub enum Words {
        Keyword,
        Operator,
        Value,
        Arguments,
        NewName,
        ExistingName,
        Code,
        Separator,
        Type,
    }

    pub fn get_word(expected_types: &Vec<Words>){
        
    }
}