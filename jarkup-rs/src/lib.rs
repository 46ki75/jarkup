mod r#macro;
mod skip_fn;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Component {
    InlineComponent(InlineComponent),
    BlockComponent(BlockComponent),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum InlineComponent {
    Text(Text),
    Icon(Icon),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum BlockComponent {
    Heading(Heading),
    Paragraph(Paragraph),
    ListItem(ListItem),
    List(List),
    BlockQuote(BlockQuote),
    Callout(Callout),
    Divider(Divider),
    Toggle(Toggle),
    Bookmark(Bookmark),
    File(File),
    Image(Image),
    CodeBlock(CodeBlock),
    Katex(Katex),
    Table(Table),
    TableRow(TableRow),
    TableCell(TableCell),
    ColumnList(ColumnList),
    Column(Column),
    Unsupported(Unsupported),
}

// Text # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    /// Always `true`
    pub inline: bool,

    pub props: TextProps,
    // Always `None`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<TextSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextProps {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

    #[serde(skip_serializing_if = "crate::skip_fn::option_false")]
    pub bold: Option<bool>,

    #[serde(skip_serializing_if = "crate::skip_fn::option_false")]
    pub italic: Option<bool>,

    #[serde(skip_serializing_if = "crate::skip_fn::option_false")]
    pub underline: Option<bool>,

    #[serde(skip_serializing_if = "crate::skip_fn::option_false")]
    pub strikethrough: Option<bool>,

    #[serde(skip_serializing_if = "crate::skip_fn::option_false")]
    pub katex: Option<bool>,

    #[serde(skip_serializing_if = "crate::skip_fn::option_false")]
    pub code: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruby: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextSlots;

crate::to_inline_component!(Text);

// Icon # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    /// Always `true`
    pub inline: bool,

    pub props: IconProps,

    // Always `None`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<IconSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IconProps {
    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IconSlots;

crate::to_inline_component!(Icon);

// Heading # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heading {
    /// Always `false`
    pub inline: bool,

    pub props: HeadingProps,

    pub slots: HeadingSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "u8", into = "u8")]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<HeadingLevel> for u8 {
    fn from(level: HeadingLevel) -> Self {
        match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }
}

impl TryFrom<u8> for HeadingLevel {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HeadingLevel::H1),
            2 => Ok(HeadingLevel::H2),
            3 => Ok(HeadingLevel::H3),
            4 => Ok(HeadingLevel::H4),
            5 => Ok(HeadingLevel::H5),
            6 => Ok(HeadingLevel::H6),
            _ => Err(format!("Invalid heading level: {}", value)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeadingProps {
    pub level: HeadingLevel,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeadingSlots {
    pub default: Vec<InlineComponent>,
}

crate::to_block_component!(Heading);

// Paragraph # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    /// Always `false`
    pub inline: bool,

    // Always `None`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<ParagraphProps>,

    pub slots: ParagraphSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphProps;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphSlots {
    pub default: Vec<InlineComponent>,
}

crate::to_block_component!(Paragraph);

// ListItem # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<ListItemProps>,
    pub slots: ListItemSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListItemProps;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListItemSlots {
    pub default: Vec<InlineComponent>,
}

crate::to_block_component!(ListItem);

// List # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<ListProps>,
    pub slots: ListSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_style: Option<ListStyle>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ListStyle {
    #[default]
    Unordered,
    Ordered,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListSlots {
    pub default: Vec<Component>,
}

crate::to_block_component!(List);

// BlockQuote # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockQuote {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<BlockQuoteProps>,
    pub slots: BlockQuoteSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockQuoteProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cite: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockQuoteSlots {
    pub default: Vec<Component>,
}

crate::to_block_component!(BlockQuote);

// Callout # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Callout {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<CalloutProps>,
    pub slots: CalloutSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum CalloutType {
    #[default]
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CalloutProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CalloutType>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CalloutSlots {
    pub default: Vec<Component>,
}

crate::to_block_component!(Callout);

// Divider # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Divider {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<DividerProps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<DividerSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DividerProps;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DividerSlots;

crate::to_block_component!(Divider);

// Toggle # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Toggle {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<ToggleProps>,
    pub slots: ToggleSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToggleProps;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToggleSlots {
    pub default: Vec<Component>,
    pub summary: Vec<InlineComponent>,
}

crate::to_block_component!(Toggle);

// Bookmark # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub inline: bool,
    pub props: BookmarkProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<BookmarkSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkProps {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkSlots;

crate::to_block_component!(Bookmark);

// File # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub inline: bool,
    pub props: FileProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<FileSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileProps {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileSlots;

crate::to_block_component!(File);

// Image # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub inline: bool,
    pub props: ImageProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<ImageSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageProps {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageSlots;

crate::to_block_component!(Image);

// CodeBlock # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeBlock {
    pub inline: bool,
    pub props: CodeBlockProps,
    pub slots: CodeBlockSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeBlockProps {
    pub code: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeBlockSlots {
    pub default: Vec<InlineComponent>,
}

crate::to_block_component!(CodeBlock);

// Katex # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Katex {
    pub inline: bool,
    pub props: KatexProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<KatexSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct KatexProps {
    pub expression: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct KatexSlots;

crate::to_block_component!(Katex);

// Table # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<TableProps>,
    pub slots: TableSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_column_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_row_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableSlots {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<Component>>,
    pub body: Vec<Component>,
}

crate::to_block_component!(Table);

// TableRow # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<TableRowProps>,
    pub slots: TableRowSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProps;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableRowSlots {
    pub default: Vec<Component>,
}

crate::to_block_component!(TableRow);

// TableCell # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<TableCellProps>,
    pub slots: TableCellSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableCellProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_header: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableCellSlots {
    pub default: Vec<InlineComponent>,
}

crate::to_block_component!(TableCell);

// ColumnList # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ColumnList {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<ColumnListProps>,
    pub slots: ColumnListSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ColumnListProps {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ColumnListSlots {
    pub default: Vec<Component>,
}

crate::to_block_component!(ColumnList);

// Column # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<ColumnProps>,
    pub slots: ColumnSlots,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ColumnProps {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ColumnSlots {
    pub default: Vec<Component>,
}

crate::to_block_component!(Column);

// Unsupported # -------------------------------------------------- #
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Unsupported {
    pub inline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<UnsupportedProps>,
    pub slots: Option<UnsupportedSlots>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnsupportedProps {
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnsupportedSlots {}

crate::to_block_component!(Unsupported);
