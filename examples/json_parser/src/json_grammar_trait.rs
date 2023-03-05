// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

// Disable clippy warnings that can result in the way how parol generates code.
#![allow(clippy::enum_variant_names)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::upper_case_acronyms)]

use crate::json_grammar::JsonGrammar;
use parol_runtime::parser::{ParseTreeType, UserActionsTrait};
use parol_runtime::{ParserError, Result};

///
/// The `JsonGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait JsonGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// Json: Value;
    ///
    fn json(&mut self, _value: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// Object: "\{" ObjectSuffix;
    ///
    fn object(&mut self, _l_brace: &ParseTreeType, _object_suffix: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ObjectSuffix: Pair ObjectList /* Vec */ "\}";
    ///
    fn object_suffix_0(
        &mut self,
        _pair: &ParseTreeType,
        _object_list: &ParseTreeType,
        _r_brace: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ObjectSuffix: "\}";
    ///
    fn object_suffix_1(&mut self, _r_brace: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ObjectList /* Vec<T>::Push */: "," Pair ObjectList;
    ///
    fn object_list_0(
        &mut self,
        _comma: &ParseTreeType,
        _pair: &ParseTreeType,
        _object_list: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// ObjectList /* Vec<T>::New */: ;
    ///
    fn object_list_1(&mut self) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// Pair: String ":" Value;
    ///
    fn pair(
        &mut self,
        _string: &ParseTreeType,
        _colon: &ParseTreeType,
        _value: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// Array: "\[" ArraySuffix;
    ///
    fn array(&mut self, _l_bracket: &ParseTreeType, _array_suffix: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// ArraySuffix: Value ArrayList /* Vec */ "\]";
    ///
    fn array_suffix_0(
        &mut self,
        _value: &ParseTreeType,
        _array_list: &ParseTreeType,
        _r_bracket: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// ArraySuffix: "\]";
    ///
    fn array_suffix_1(&mut self, _r_bracket: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// ArrayList /* Vec<T>::Push */: "," Value ArrayList;
    ///
    fn array_list_0(
        &mut self,
        _comma: &ParseTreeType,
        _value: &ParseTreeType,
        _array_list: &ParseTreeType,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// ArrayList /* Vec<T>::New */: ;
    ///
    fn array_list_1(&mut self) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// Value: String;
    ///
    fn value_0(&mut self, _string: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// Value: Number;
    ///
    fn value_1(&mut self, _number: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// Value: Object;
    ///
    fn value_2(&mut self, _object: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// Value: Array;
    ///
    fn value_3(&mut self, _array: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// Value: "true";
    ///
    fn value_4(&mut self, _true: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// Value: "false";
    ///
    fn value_5(&mut self, _false: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// Value: "null";
    ///
    fn value_6(&mut self, _null: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// String: "\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    ///
    fn string(&mut self, _string: &ParseTreeType) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// Number: "-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)?";
    ///
    fn number(&mut self, _number: &ParseTreeType) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait<'_> for JsonGrammar {
    ///
    /// This function is implemented automatically for the user's item JsonGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeType],
    ) -> Result<()> {
        match prod_num {
            0 => self.json(&children[0]),
            1 => self.object(&children[0], &children[1]),
            2 => self.object_suffix_0(&children[0], &children[1], &children[2]),
            3 => self.object_suffix_1(&children[0]),
            4 => self.object_list_0(&children[0], &children[1], &children[2]),
            5 => self.object_list_1(),
            6 => self.pair(&children[0], &children[1], &children[2]),
            7 => self.array(&children[0], &children[1]),
            8 => self.array_suffix_0(&children[0], &children[1], &children[2]),
            9 => self.array_suffix_1(&children[0]),
            10 => self.array_list_0(&children[0], &children[1], &children[2]),
            11 => self.array_list_1(),
            12 => self.value_0(&children[0]),
            13 => self.value_1(&children[0]),
            14 => self.value_2(&children[0]),
            15 => self.value_3(&children[0]),
            16 => self.value_4(&children[0]),
            17 => self.value_5(&children[0]),
            18 => self.value_6(&children[0]),
            19 => self.string(&children[0]),
            20 => self.number(&children[0]),
            _ => Err(ParserError::InternalError(format!(
                "Unhandled production number: {}",
                prod_num
            ))
            .into()),
        }
    }
}
