// Component Map
export type InlineComponentMap = {
  Text: Text;
  Icon: Icon;
};

export type BlockComponentMap = {
  Heading: Heading;
  Paragraph: Paragraph;
  ListItem: ListItem;
  List: List;
  BlockQuote: BlockQuote;
  Callout: Callout;
  Divider: Divider;
  Toggle: Toggle;
  Bookmark: Bookmark;
  File: File;
  Image: Image;
  CodeBlock: CodeBlock;
  Katex: Katex;
  Table: Table;
  TableRow: TableRow;
  TableCell: TableCell;
  ColumnList: ColumnList;
  Column: Column;
  Unsupported: Unsupported;
};

export type ComponentMap = InlineComponentMap & BlockComponentMap;

// Types
export type InlineComponentType = keyof InlineComponentMap;
export type BlockComponentType = keyof BlockComponentMap;
export type ComponentType = keyof ComponentMap;

export type InlineComponent = InlineComponentMap[keyof InlineComponentMap];
export type BlockComponent = BlockComponentMap[keyof BlockComponentMap];
export type Component = ComponentMap[keyof ComponentMap];

// Base Interfaces
export interface ComponentBase<
  T extends ComponentType = ComponentType,
  P = Record<any, any>
> {
  type: T;
  id?: string;
  props?: P;
  slots?: Record<string, ComponentBase | ComponentBase[]>;
}

export interface InlineComponentBase<
  T extends InlineComponentType = InlineComponentType,
  P = Record<any, any>
> extends ComponentBase<T, P> {
  type: T;
  id?: string;
  props?: P;
  slots?: undefined;
}

export interface BlockComponentBase<
  T extends BlockComponentType = BlockComponentType,
  P = Record<any, any>
> extends ComponentBase<T, P> {
  type: T;
  id?: string;
  props?: P;
  slots?: Record<string, Component | Component[]>;
}

// Inline Components
export interface Text extends InlineComponentBase<"Text"> {
  type: "Text";
  id?: string;
  props: {
    text: string;
    color?: string;
    backgroundColor?: string;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    strikethrough?: boolean;
    katex?: boolean;
    code?: boolean;
    kbd?: boolean;
    ruby?: string;
    href?: string;
    favicon?: string;
  };
  slots?: undefined;
}

export interface Icon extends InlineComponentBase<"Icon"> {
  type: "Icon";
  id?: string;
  props: {
    src: string;
    alt?: string;
  };
  slots?: undefined;
}

// Block Components
export interface Heading extends BlockComponentBase<"Heading"> {
  type: "Heading";
  id?: string;
  props: { level: 1 | 2 | 3 | 4 | 5 | 6 };
  slots: { default: InlineComponent[] };
}

export interface Paragraph extends BlockComponentBase<"Paragraph"> {
  type: "Paragraph";
  id?: string;
  props?: undefined;
  slots: { default: InlineComponent[] };
}

export interface ListItem extends BlockComponentBase<"ListItem"> {
  type: "ListItem";
  id?: string;
  props?: undefined;
  slots: { default: InlineComponent[] };
}

export interface List extends BlockComponentBase<"List"> {
  type: "List";
  id?: string;
  props?: { listStyle?: "unordered" | "ordered" };
  slots: { default: ListItem[] };
}

export interface BlockQuote extends BlockComponentBase<"BlockQuote"> {
  type: "BlockQuote";
  id?: string;
  props?: { cite?: string };
  slots: { default: Component[] };
}

export interface Callout extends BlockComponentBase<"Callout"> {
  type: "Callout";
  id?: string;
  props?: { type?: "note" | "tip" | "important" | "warning" | "caution" };
  slots: { default: Component[] };
}

export interface Divider extends BlockComponentBase<"Divider"> {
  type: "Divider";
  id?: string;
  props?: undefined;
  slots?: undefined;
}

export interface Toggle extends BlockComponentBase<"Toggle"> {
  type: "Toggle";
  id?: string;
  props?: undefined;
  slots: {
    default: Component[];
    summary: InlineComponent[];
  };
}

export interface Bookmark extends BlockComponentBase<"Bookmark"> {
  type: "Bookmark";
  id?: string;
  props: {
    url: string;
    title?: string;
    description?: string;
    image?: string;
  };
  slots?: undefined;
}

export interface File extends BlockComponentBase<"File"> {
  type: "File";
  id?: string;
  props: {
    src: string;
    name?: string;
  };
  slots?: undefined;
}

export interface Image extends BlockComponentBase<"Image"> {
  type: "Image";
  id?: string;
  props: {
    src: string;
    alt?: string;
  };
  slots?: undefined;
}

export interface CodeBlock extends BlockComponentBase<"CodeBlock"> {
  type: "CodeBlock";
  id?: string;
  props: {
    code: string;
    language: string;
  };
  slots?: {
    default: InlineComponent[];
  };
}

export interface Katex extends BlockComponentBase<"Katex"> {
  type: "Katex";
  id?: string;
  props: {
    expression: string;
  };
  slots?: undefined;
}

export interface Table extends BlockComponentBase<"Table"> {
  type: "Table";
  id?: string;
  props?: {
    hasColumnHeader?: boolean;
    hasRowHeader?: boolean;
    caption?: string;
  };
  slots: {
    header?: TableRow[];
    body: TableRow[];
  };
}

export interface TableRow extends BlockComponentBase<"TableRow"> {
  type: "TableRow";
  id?: string;
  props?: undefined;
  slots: { default: TableCell[] };
}

export interface TableCell extends BlockComponentBase<"TableCell"> {
  type: "TableCell";
  id?: string;
  props?: { isHeader?: boolean };
  slots: { default: InlineComponent[] };
}

export interface ColumnList extends BlockComponentBase<"ColumnList"> {
  type: "ColumnList";
  id?: string;
  props?: undefined;
  slots: { default: Column[] };
}

export interface Column extends BlockComponentBase<"Column"> {
  type: "Column";
  id?: string;
  props?: undefined;
  slots: { default: Component[] };
}

export interface Unsupported extends BlockComponentBase<"Unsupported"> {
  type: "Unsupported";
  id?: string;
  props?: { details: string };
  slots?: undefined;
}
