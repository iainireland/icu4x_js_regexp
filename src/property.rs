// TODO: license

// The set of properties supported by the ECMAScript language specification.
pub enum Property {
    // Binary properties: https://tc39.es/ecma262/#table-binary-unicode-properties
    Alphabetic,
    AsciiHexDigit,
    BidiControl,
    BidiMirrored,
    CaseIgnorable,
    Cased,
    ChangesWhenCasefolded,
    ChangesWhenCasemapped,
    ChangesWhenLowercased,
    ChangesWhenNfkcCasefolded,
    ChangesWhenTitlecased,
    ChangesWhenUppercased,
    Dash,
    DefaultIgnorableCodePoint,
    Deprecated,
    Diacritic,
    Emoji,
    EmojiComponent,
    EmojiModifierBase,
    EmojiModifier,
    EmojiPresentation,
    ExtendedPictographic,
    Extender,
    GraphemeBase,
    GraphemeExtend,
    HexDigit,
    IdsBinaryOperator,
    IdsTrinaryOperator,
    IdContinue,
    IdStart,
    Ideographic,
    JoinControl,
    LogicalOrderException,
    Lowercase,
    Math,
    NoncharacterCodePoint,
    PatternSyntax,
    PatternWhiteSpace,
    QuotationMark,
    Radical,
    RegionalIndicator,
    SentenceTerminal,
    SoftDotted,
    TerminalPunctuation,
    UnifiedIdeograph,
    Uppercase,
    VariationSelector,
    WhiteSpace,
    XidContinue,
    XidStart,

    // Enumerated properties: https://tc39.es/ecma262/#table-nonbinary-unicode-properties
    GeneralCategory,
    Script,
    ScriptExtension,

    // Special cases: See https://unicode.org/reports/tr18/#General_Category_Property
    Ascii,
    Any,
    Assigned,
}

pub fn get_property(prop_name: &str) -> Option<Property> {
    match prop_name {
        "Alphabetic" | "Alpha" => Some(Property::Alphabetic),
        "ASCII_Hex_Digit" | "AHex" => Some(Property::AsciiHexDigit),
        "Bidi_Control" | "Bidi_C" => Some(Property::BidiControl),
        "Bidi_Mirrored" | "Bidi_M" => Some(Property::BidiMirrored),
        "Case_Ignorable" | "CI" => Some(Property::CaseIgnorable),
        "Cased" => Some(Property::Cased),
        "Changes_When_Casefolded" | "CWCF" => Some(Property::ChangesWhenCasefolded),
        "Changes_When_Casemapped" | "CWCM" => Some(Property::ChangesWhenCasemapped),
        "Changes_When_Lowercased" | "CWL" => Some(Property::ChangesWhenLowercased),
        "Changes_When_NFKC_Casefolded" | "CWKCF" => Some(Property::ChangesWhenNfkcCasefolded),
        "Changes_When_Titlecased" | "CWT" => Some(Property::ChangesWhenTitlecased),
        "Changes_When_Uppercased" | "CWU" => Some(Property::ChangesWhenUppercased),
        "Dash" => Some(Property::Dash),
        "Default_Ignorable_Code_Point" | "DI" => Some(Property::DefaultIgnorableCodePoint),
        "Deprecated" | "Dep" => Some(Property::Deprecated),
        "Diacritic" | "Dia" => Some(Property::Diacritic),
        "Emoji" => Some(Property::Emoji),
        "Emoji_Component" | "EComp" => Some(Property::EmojiComponent),
        "Emoji_Modifier_Base" | "EBase" => Some(Property::EmojiModifierBase),
        "Emoji_Modifier" | "EMod" => Some(Property::EmojiModifier),
        "Emoji_Presentation" | "EPres" => Some(Property::EmojiPresentation),
        "Extended_Pictographic" | "ExtPict" => Some(Property::ExtendedPictographic),
        "Extender" | "Ext" => Some(Property::Extender),
        "Grapheme_Base" | "Gr_Base" => Some(Property::GraphemeBase),
        "Grapheme_Extend" | "Gr_Ext" => Some(Property::GraphemeExtend),
        "Hex_Digit" | "Hex" => Some(Property::HexDigit),
        "IDS_Binary_Operator" | "IDSB" => Some(Property::IdsBinaryOperator),
        "IDS_Trinary_Operator" | "IDST" => Some(Property::IdsTrinaryOperator),
        "Id_Continue" | "IDC" => Some(Property::IdContinue),
        "Id_Start" | "IDS" => Some(Property::IdStart),
        "Ideographic" | "Ideo" => Some(Property::Ideographic),
        "Join_Control" | "JoinC" => Some(Property::JoinControl),
        "Logical_Order_Exception" | "LOE" => Some(Property::LogicalOrderException),
        "Lowercase" | "Lower" => Some(Property::Lowercase),
        "Math" => Some(Property::Math),
        "Noncharacter_Code_Point" | "NChar" => Some(Property::NoncharacterCodePoint),
        "Pattern_Syntax" | "Pat_Syn" => Some(Property::PatternSyntax),
        "Pattern_White_Space" | "Pat_WS" => Some(Property::PatternWhiteSpace),
        "Quotation_Mark" | "QMark" => Some(Property::QuotationMark),
        "Radical" => Some(Property::Radical),
        "Regional_Indicator" | "RI" => Some(Property::RegionalIndicator),
        "SentenceTerminal" | "STerm" => Some(Property::SentenceTerminal),
        "Soft_Dotted" | "SD" => Some(Property::SoftDotted),
        "Terminal_Punctuation" | "Term" => Some(Property::TerminalPunctuation),
        "Unified_Ideograph" | "UIdeo" => Some(Property::UnifiedIdeograph),
        "Uppercase" | "Upper" => Some(Property::Uppercase),
        "Variation_Selector" | "VS" => Some(Property::VariationSelector),
        "White_Space" | "space" => Some(Property::WhiteSpace),
        "Xid_Continue" | "XIDC" => Some(Property::XidContinue),
        "Xid_Start" | "XIDS" => Some(Property::XidStart),

        "General_Category" | "gc" => Some(Property::GeneralCategory),
        "Script" | "sc" => Some(Property::Script),
        "Script_Extensions" | "scx" => Some(Property::ScriptExtension),

        "ASCII" => Some(Property::Ascii),
        "Any" => Some(Property::Any),
        "Assigned" => Some(Property::Assigned),

        _ => None,
    }
}
