// Compile.rs v0.1.20170905
// A Rust-like WebAssembly language with self-hosted compiler

enum Token {
  Identifier,
  StrLiteral, CharLiteral, NumLiteral, True, False,
  LParen, RParen, LBrace, RBrace, Dot, Comma, Colon, Semicolon, DoubleColon, Arrow,

  // Keywords
  Break, Const, Continue, Else, Enum, Fun, If, Let, Loop, Mut, Pub, 
  Return, Static, While,

  // TODO
  As, Crate, Extern, For, Impl, In, Match, Mod, Move, Ref,               
  UpperSelf, Lowerself, Struct, Super, Trait, Type, Unsafe, Use, Where,

  // Operators
  MinPrecedence,
  Assign,
  Or, BoolOr, Xor,
  And, BoolAnd,
  Eql, Ne, Lt, Ltu, Le, Leu, Gt, Gtu, Ge, Geu,
  Shl, Shr, Shru,
  Add, Sub,
  Mul, Div, Divu, Rem, Remu,
  Not,

  // TODO
  Min, Max, CopySign, Rotl, Rotr, Abs, Neg, Ceil, Floor, Trunc, Round, Sqrt, Clz, Ctz, Cnt,

  // Data types
  F64, F32, I64, I32, Bool
}

enum Node {
  Module,
  Data, Enum, 
  Fun, Parameter, Return, Call, 
  Block,  
  Variable, Identifier, Literal, 
  Assign, Binary, Unary, 
  DotLoad, DotStore,  
  Iif, If, 
  Loop, Break, BreakIf, Continue,
  Pop
}

enum Error {
  DuplicateName, InvalidToken, MissingToken, Expression, TypeMismatch, 
  RootStatement, TypeNotInferred, NotDeclared, LiteralToInt, BlockStatement, 
  EmitNode, InvalidOperator, NotMutable, NoIdentifiers, NoParamList
}

// Magic number -0x00dec0de - used for debugging
const DEC0DE: i32 = 557785600;

// Output Binary (string)
static mut WASM: i32 = 0;

// Error messages (list)
static mut ERRORS: i32 = 0;

pub fn main() -> i32 {
  let dwasm: i32 = 0;  // Input (string)
  let ignore: i32 = new_string(dwasm.string_length);  // Fix the heap pointer to include the source string
  ERRORS = new_list();
  lexx(dwasm);
  if ERRORS.list_count.i32 == 0 { 
    parse();
  }
  if ERRORS.list_count.i32 == 0 {
    emit(dwasm);
  }
  if ERRORS.list_count.i32 > 0 { 
    parse_error_list();
  }
  WASM + string_length  // Return the memory location of the string
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lexer 

// Struct
const token_dec0de: i32 = 0;  // debugging marker
const token_kind:   i32 = 4;
const token_Value:  i32 = 8;
const token_line:   i32 = 12;
const token_column: i32 = 16;
const token_size:   i32 = 20;

static mut TokenList: i32 = 0;
static mut CurrentToken: i32 = 0;
static mut CurrentTokenItem: i32 = 0;
static mut NextToken: i32 = 0;

fn add_token(kind: i32, text: i32, line: i32, column: i32) {
  let mut token: i32 = allocate(token_size);
  token.token_dec0de = 6 - DEC0DE;
  token.token_kind = kind;
  token.token_Value = text;
  token.token_line = line;
  token.token_column = column;
  list_add(TokenList, token);
}

fn process_token(value_str: i32, line: i32, column: i32) {
  let mut kind: i32 = Token::Identifier;
  if str_eq_char(value_str, '(') { kind = Token::LParen;  
  } else if str_eq_char(value_str, ',') { kind = Token::Comma; 
  } else if str_eq_char(value_str, ')') { kind = Token::RParen; 
  } else if str_eq_char(value_str, '{') { kind = Token::LBrace; 
  } else if str_eq_char(value_str, '}') { kind = Token::RBrace; 
  } else if str_eq_char(value_str, ':') { kind = Token::Colon; 
  } else if str_eq_char(value_str, ';') { kind = Token::Semicolon; 
  } else if str_eq_char(value_str, '=') { kind = Token::Assign; 
  } else if str_eq_char(value_str, '<') { kind = Token::Lt;
  } else if str_eq_char(value_str, '>') { kind = Token::Gt;
  } else if str_eq_char(value_str, '+') { kind = Token::Add;
  } else if str_eq_char(value_str, '-') { kind = Token::Sub;
  } else if str_eq_char(value_str, '*') { kind = Token::Mul; 
  } else if str_eq_char(value_str, '/') { kind = Token::Div; 
  } else if str_eq_char(value_str, '!') { kind = Token::Not;
  } else if str_eq_char(value_str, '%') { kind = Token::Rem;
  } else if str_eq_char(value_str, '^') { kind = Token::Xor;
  } else if str_eq_char(value_str, '&') { kind = Token::And; 
  } else if str_eq_char(value_str, '|') { kind = Token::Or; 
  } else if str_eq_char(value_str, '<<') { kind = Token::Shl; 
  } else if str_eq_char(value_str, '>>') { kind = Token::Shr;
  } else if str_eq_char(value_str, '::') { kind = Token::DoubleColon; 
  } else if str_eq_char(value_str, '&&') { kind = Token::BoolAnd; 
  } else if str_eq_char(value_str, '||') { kind = Token::BoolOr; 
  } else if str_eq_char(value_str, '->') { kind = Token::Arrow; 
  } else if str_eq_char(value_str, '==') { kind = Token::Eql; 
  } else if str_eq_char(value_str, '!=') { kind = Token::Ne;
  } else if str_eq_char(value_str, '<=') { kind = Token::Le;
  } else if str_eq_char(value_str, '>=') { kind = Token::Ge; 
  } else if str_eq_char(value_str, '%+') { kind = Token::Remu;
  } else if str_eq_char(value_str, '/+') { kind = Token::Divu; 
  } else if str_eq_char(value_str, '>+') { kind = Token::Gtu;
  } else if str_eq_char(value_str, '<+') { kind = Token::Ltu;
  } else if str_eq_char(value_str, '<=+') { kind = Token::Leu; 
  } else if str_eq_char(value_str, '>=+') { kind = Token::Geu;
  } else if str_eq_char(value_str, '>>+') { kind = Token::Shru;
  } else if str_eq_char(value_str, 'fn') { kind = Token::Fun;
  } else if str_eq_char(value_str, 'if') { kind = Token::If; 
  } else if str_eq_char(value_str, 'pub') { kind = Token::Pub;
  } else if str_eq_char(value_str, 'let') { kind = Token::Let;
  } else if str_eq_char(value_str, 'mut') { kind = Token::Mut;
  } else if str_eq_char(value_str, 'mod') { kind = Token::Mod;
  } else if str_eq_char(value_str, 'loop') { kind = Token::Loop;
  } else if str_eq_char(value_str, 'enum') { kind = Token::Enum;
  } else if str_eq_char(value_str, 'else') { kind = Token::Else;
  } else if str_eq_char(value_str, 'true') { kind = Token::True; 
  } else if str_eq_char(value_str, 'false') { kind = Token::False;
  } else if str_eq_char(value_str, 'break') { kind = Token::Break;
  } else if str_eq_char(value_str, 'const') { kind = Token::Const;
  } else if str_eq_char(value_str, 'while') { kind = Token::While;
  } else if str_eq_char(value_str, 'static') { kind = Token::Static;
  } else if str_eq_char(value_str, 'return') { kind = Token::Return;
  } else if str_eq_char(value_str, 'continue') { kind = Token::Continue;
  } else if str_eq_char(value_str, 'i32') { kind = Token::I32;
  } else if str_eq_char(value_str, 'i64') { kind = Token::I64;
  } else if str_eq_char(value_str, 'f32') { kind = Token::F32;
  } else if str_eq_char(value_str, 'f64') { kind = Token::F64;
  } else if str_eq_char(value_str, 'bool') { kind = Token::Bool; }
  add_token(kind, value_str, line, column);
}

fn is_single_chr(chr: i32) -> i32 {
  chr == '(' | chr == ')' | chr == ',' | chr == '{' | chr == '}' | chr == ';'
}

fn is_operator_chr(chr: i32) -> i32 {
  chr == '=' | chr == '+' | chr == '-' | chr == '/' | chr == '*' | chr == '<' | chr == '>' | chr == '^' 
    | chr == '!' | chr == '%' | chr == ':' | chr == '&' | chr == '|'
}

fn lexx(dwasm: i32) {
  TokenList = new_list();
  let mut pos: i32 = -1;
  let mut line: i32 = 1;
  let mut column: i32 = 0;
  let length: i32 = dwasm.string_length;
  let mut start: i32 = 0;
  let mut value_str: i32 = 0;
  while pos < length { 
    pos = pos + 1;
    column = column + 1;
    let mut chr: i32 = get_chr(dwasm, pos);

    // newline chr
    if chr == 10 {
      line = line + 1;
      column = 0;

    // Identifiers & reserved words
    } else if is_alpha(chr) {
      start = pos;
       while pos < length {
        if (!is_alpha(chr)) & (!is_number(chr, false)) {
          pos = pos - 1;
          column = column - 1;
          break;
        }
        pos = pos + 1;
        column = column + 1;
        chr = get_chr(dwasm, pos);
      }
      value_str = sub_str(dwasm, start, pos - start + 1);
      process_token(value_str, line, column);
      if get_chr(dwasm, pos + 1) == '.' & is_alpha(get_chr(dwasm, pos + 2)) {
        pos = pos + 1;
        column = column + 1;
        chr = get_chr(dwasm, pos);
        add_token(Token::Dot, value_str, line, column);
      }
    
    // Single quoted chars | long chars up to 64 bit
    } else if chr == 39 {
      pos = pos + 1;
      column = column + 1;
      chr = get_chr(dwasm, pos);
      start = pos;
      while pos < length {
        if chr == 39 { break; }
        pos = pos + 1;
        column = column + 1;
        chr = get_chr(dwasm, pos);
      }
      value_str = sub_str(dwasm, start, pos - start);
      decode_str(value_str);
      add_token(Token::CharLiteral, value_str, line, column);

    // Double quoted strings
    } else if chr == '"' {
      pos = pos + 1;
      column = column + 1;
      chr = get_chr(dwasm, pos);
      start = pos;
      while pos < length {
        if chr == '"' { break; }
        pos = pos + 1;
        column = column + 1;
        chr = get_chr(dwasm, pos);
      }
      value_str = sub_str(dwasm, start, pos - start);
      decode_str(value_str);
      add_token(Token::StrLiteral, value_str, line, column);

    // Number literals, for example -42, 3.14, 0x8d4f0
    } else if is_number(chr, false) | ((chr == '-') & is_number(get_chr(dwasm, pos + 1), false)) {
      start = pos;
      let mut is_hex: bool = false;
      while pos < length {
        if (!is_number(chr, is_hex)) & (chr != '-') {
          if start + 1 == pos & chr == 'x' {
            is_hex = true;
          } else {
            pos = pos - 1;
            column = column - 1;
            break;
          }
        }
        pos = pos + 1;
        column = column + 1;
        chr = get_chr(dwasm, pos);
      }
      if chr == '.' & !is_hex {
        pos = pos + 2;
        column = column + 2;
        chr = get_chr(dwasm, pos);
        while pos < length {
          if (!is_number(chr, is_hex)) {
            pos = pos - 1;
            column = column - 1;
            break;
          }
          pos = pos + 1;
          column = column + 1;
          chr = get_chr(dwasm, pos);
        }
      }
      value_str = sub_str(dwasm, start, pos - start + 1);
      add_token(Token::NumLiteral, value_str, line, column);

    // Comments
    } else if chr == '/' & get_chr(dwasm, pos + 1) == '/' {
      while pos < length {
        if chr == 10 | chr == 13 {  // LF | CR
          column = 0;
          line = line + 1;
          break;
        }
        pos = pos + 1;
        column = column + 1;
        chr = get_chr(dwasm, pos);
      }
    
    // Parenthases & commas
    } else if is_single_chr(chr) {
      value_str = sub_str(dwasm, pos, 1);
      process_token(value_str, line, column);

    // Mathematical operators
    } else if is_operator_chr(chr) {
      if is_operator_chr(get_chr(dwasm, pos + 1)) {
        if is_operator_chr(get_chr(dwasm, pos + 2)) {
          value_str = sub_str(dwasm, pos, 3);
          pos = pos + 2;
          column = column + 2;
        } else {
          value_str = sub_str(dwasm, pos, 2);
          pos = pos + 1;
          column = column + 1;
        }
      } else {
        value_str = sub_str(dwasm, pos, 1);
      }
      process_token(value_str, line, column);

    }
    if pos >= length { break; }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Scoper

// Struct
const scope_dec0de:     i32 = 0;   // debugging marker
const scope_Node:       i32 = 4;
const scope_index:      i32 = 8;
const scope_Parent:     i32 = 12;
const scope_Symbols:    i32 = 16;
const scope_localIndex: i32 = 20;
const scope_size:       i32 = 24;

static mut CurrentScope: i32 = 0;
static mut GlobalScope: i32 = 0;

fn push_scope(node: i32) {
  let scope: i32 = allocate(scope_size);
  scope.scope_dec0de = 3 - DEC0DE;
  scope.scope_Symbols = new_list();
  scope.scope_Node = node;
  if CurrentScope {
    scope.scope_index.i32 = CurrentScope.scope_index + 1;
    scope.scope_Parent = CurrentScope;
  }
  node.node_Scope = scope;
  CurrentScope = scope;
}

fn pop_scope() {
  CurrentScope = CurrentScope.scope_Parent;
}

fn get_fn_scope(scope: i32) -> i32 {
  let mut fn_scope: i32 = scope;
  while fn_scope {
    if fn_scope.scope_Node.node_kind.i32 == Node::Fun { break; }
    if fn_scope.scope_Node.node_kind.i32 == Node::Module { break; }
    fn_scope = fn_scope.scope_Parent;
  }
  fn_scope
}

fn scope_register_name(scope: i32, name: i32, node: i32, token: i32) {
  if list_search(scope.scope_Symbols, name) {
    add_error(Error::DuplicateName, token);
  }
  let kind: i32 = node.node_kind;
  list_add_name(scope.scope_Symbols, node, name);
  if kind == Node::Variable | kind == Node::Parameter {
    let fn_scope: i32 = get_fn_scope(scope);
    let index: i32 = fn_scope.scope_localIndex;
    node.node_Scope = fn_scope;
    node.node_index = index;
    fn_scope.scope_localIndex = index + 1;
  }
}

fn scope_resolve(scope: i32, name: i32, token: i32) -> i32 {
  let mut node: i32 = 0;
  let mut recurse_scope: i32 = scope;
  while recurse_scope {
    node = list_search(recurse_scope.scope_Symbols, name);
    if node { break; }
    recurse_scope = recurse_scope.scope_Parent;
  }
  if !node {
    add_error(Error::NotDeclared, token);
  }
  node
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Parser 

// Structs
const node_dec0de:     i32 = 0;   // debugging marker
const node_kind:       i32 = 4;   // From the Node enum
const node_index:      i32 = 8;   // Zero based index number for funs, variables, parameters
const node_String:     i32 = 12;  // Literal value, Or fn/var/Parameter name
const node_Scope:      i32 = 16;  // scope for Module/Block/loop/fun used for name resolution
const node_ANode:      i32 = 20;  // Binary left, Call fn, return Expression, Block, Or fun body
const node_BNode:      i32 = 24;  // Binary/Unary right, else Block, fun return, Variable assignment
const node_CNode:      i32 = 28;  // If statement condition node
const node_Nodes:      i32 = 32;  // List of child Node for Module/Block, enums, Or fun locals
const node_ParamNodes: i32 = 36;  // List of params for Call/fn
const node_type:       i32 = 40;  // From the Token::_ enum
const node_dataType:   i32 = 44;  // inferred data type
const node_Token:      i32 = 48;
const node_assigns:    i32 = 52;
const node_size:       i32 = 56;

static mut RootNode: i32 = 0;
static mut ExportList: i32 = 0;
static mut DataList: i32 = 0;
static mut funIndex: i32 = 0;  // Next function index number

fn new_node(kind: i32) -> i32 {
  let node: i32 = allocate(node_size);
  node.node_dec0de = 2 - DEC0DE;
  node.node_Scope = CurrentScope;
  node.node_Token = CurrentToken;
  node.node_kind = kind;
  node
}

fn next_token() {
  CurrentTokenItem = CurrentTokenItem.item_Next;
  if CurrentTokenItem {
    CurrentToken = CurrentTokenItem.item_Object;
  } else {
    CurrentToken = 0;
  }
  let next_token_item: i32 = CurrentTokenItem.item_Next;
  if next_token_item {
    NextToken = next_token_item.item_Object;
  } else {
    NextToken = 0;
  }
}

fn is_binary_op(token: i32) -> bool {
  let kind: i32 = token.token_kind;
  kind == Token::Add | kind == Token::Sub | kind == Token::Mul | kind == Token::Div | kind == Token::Rem
    | kind == Token::Remu | kind == Token::Or | kind == Token::And | kind == Token::Lt | kind == Token::Eql 
    | kind == Token::Ne | kind == Token::Lt | kind == Token::Le | kind == Token::Gt | kind == Token::Ge 
    | kind == Token::Shl | kind == Token::Shr | kind == Token::Xor | kind == Token::Ltu | kind == Token::Leu 
    | kind == Token::Gtu | kind == Token::Geu | kind == Token::Shru | kind == Token::Rotl 
    | kind == Token::Rotr
}

fn is_unary_op(token: i32) -> bool {
  let kind: i32 = token.token_kind;
  kind == Token::Sub | kind == Token::Not | kind == Token::Cnt | kind == Token::Clz | kind == Token::Ctz
    | kind == Token::Abs | kind == Token::Neg | kind == Token::Ceil | kind == Token::Floor
    | kind == Token::Trunc | kind == Token::Round | kind == Token::Sqrt 
}

fn is_literal(token: i32) -> bool {
  let kind: i32 = token.token_kind;
  kind == Token::NumLiteral | kind == Token::CharLiteral | kind == Token::True | kind == Token::False
}

fn is_native_type(token: i32) -> bool {
  let k: i32 = token.token_kind;
  k == Token::I32 | k == Token::I64 | k == Token::F32 | k == Token::F64 | k == Token::Bool
}

fn eat_token(kind: i32) {
  if CurrentToken {
    if CurrentToken.token_kind == kind {
      next_token();
    } else {
      add_error(Error::InvalidToken, CurrentToken);
    }
  } else {
    let LastToken: i32 = TokenList.list_Last.item_Object;
    add_error(Error::MissingToken, LastToken);
  }
}

fn try_eat_token(kind: i32) -> bool {
  if CurrentToken {
    if CurrentToken.token_kind == kind {
      next_token();
      return true;
    }
  } 
  false
}

fn parse_fn_params() -> i32 {
  let params: i32 = new_list();
  eat_token(Token::LParen);
  while CurrentToken.token_kind.i32 != Token::RParen {
    let mutable: i32 = try_eat_token(Token::Mut);
    let name: i32 = CurrentToken.token_Value;
    next_token();
    eat_token(Token::Colon);
    let type: i32 = CurrentToken.token_kind;
    next_token();
    let FunParamNode: i32 = new_node(Node::Parameter);
    FunParamNode.node_type = type;
    FunParamNode.node_dataType = type;
    FunParamNode.node_String = name;
    if mutable {
      FunParamNode.node_assigns = -1;
    } else {
      FunParamNode.node_assigns = 1;
    }
    list_add_name(params, FunParamNode, name);
    if CurrentToken.token_kind.i32 != Token::Comma { break; }
    eat_token(Token::Comma);
  }
  eat_token(Token::RParen);
  params
}

fn parse_fn_block() -> i32 {
  let node: i32 = new_node(Node::Block);
  let BodyList: i32 = new_list();
  node.node_Nodes = BodyList;
  node.node_Scope = CurrentScope;
  while CurrentToken {
    if CurrentToken.token_kind.i32 == Token::RBrace { break; }
    let ChildNode: i32 = parse_statement();
    if !ChildNode { break; }
    list_add(BodyList, ChildNode);
  }
  node
}

fn parse_enum() -> i32 {
  eat_token(Token::Enum);
  let node: i32 = new_node(Node::Enum);
  let name: i32 = CurrentToken.token_Value;
  node.node_String = name;
  let Enums: i32 = new_list();
  node.node_Nodes = Enums;
  scope_register_name(CurrentScope, name, node, CurrentToken);
  next_token();
  eat_token(Token::LBrace);
  let mut enum_value: i32 = 1;
  while CurrentToken {
    if CurrentToken.token_kind.i32 == Token::RParen { break; }
    list_add_name(Enums, enum_value, CurrentToken.token_Value);
    next_token();
    if CurrentToken.token_kind.i32 != Token::Comma { break; }
    eat_token(Token::Comma);
    enum_value = enum_value + 1;
  }
  eat_token(Token::RBrace);
  node
}

fn parse_fn() -> i32 {
  let mut exported: bool = false;
  if CurrentToken.token_kind.i32 == Token::Pub {
    exported = true;
    eat_token(Token::Pub);
  }
  eat_token(Token::Fun);
  let mut type: i32 = 0;  
  let name: i32 = CurrentToken.token_Value;
  let node: i32 = new_node(Node::Fun);
  scope_register_name(CurrentScope, name, node, CurrentToken);
  next_token();
  let Locals: i32 = new_list();
  node.node_index = funIndex;
  funIndex = funIndex + 1;
  node.node_String = name;
  node.node_Nodes = Locals;
  let ParamList: i32 = parse_fn_params();
  node.node_ParamNodes = ParamList;
  if CurrentToken.token_kind.i32 == Token::Arrow {
    eat_token(Token::Arrow);
    type = CurrentToken.token_kind;
    next_token();
  }
  node.node_type = type;
  node.node_dataType = type;
  push_scope(node);
  let mut ParamItem: i32 = ParamList.list_First;
  while ParamItem {
    let ParamName: i32 = ParamItem.item_Name;
    let ParamNode: i32 = ParamItem.item_Object;
    scope_register_name(CurrentScope, ParamName, ParamNode, ParamNode.node_Token);
    ParamItem = ParamItem.item_Next;
  }
  if exported {
    list_add_name(ExportList, node, name);
  }
  eat_token(Token::LBrace);
  node.node_ANode = parse_fn_block();
  pop_scope();
  eat_token(Token::RBrace);
  node
}

fn parse_break() -> i32 {
  let node: i32 = new_node(Node::Break);
  eat_token(Token::Break);
  eat_token(Token::Semicolon);
  node
}

fn parse_continue() -> i32 {
  let node: i32 = new_node(Node::Continue);
  eat_token(Token::Continue);
  eat_token(Token::Semicolon);
  node
}

fn parse_literal() -> i32 {
  let node: i32 = new_node(Node::Literal);
  node.node_String.i32 = CurrentToken.token_Value;
  node.node_type.i32 = CurrentToken.token_kind;
  next_token();
  node
}

fn parse_identifier() -> i32 {
  let node: i32 = new_node(Node::Identifier);
  node.node_String.i32 = CurrentToken.token_Value;
  node.node_type.i32 = CurrentToken.token_kind;
  next_token();
  node
}

fn parse_call_params() -> i32 {
  let ParamList: i32 = new_list();
  eat_token(Token::LParen);
  while CurrentToken {
    if CurrentToken.token_kind.i32 == Token::RParen { break; }
    list_add(ParamList, parse_expression(Token::MinPrecedence));
    if CurrentToken.token_kind.i32 != Token::Comma { break; }
    eat_token(Token::Comma);
  }
  eat_token(Token::RParen);
  ParamList
}

fn parse_call_expression(Callee: i32) -> i32 {
  let node: i32 = new_node(Node::Call);
  node.node_ANode = Callee;
  node.node_ParamNodes = parse_call_params();
  node
}

fn parse_unary_expression() -> i32 {
  let node: i32 = new_node(Node::Unary);
  node.node_type.i32 = CurrentToken.token_kind;
  node.node_String.i32 = CurrentToken.token_Value;
  next_token();
  node.node_BNode = parse_expression(Token::Add);
  node
}

fn parse_double_colon() -> i32 {
  let node: i32 = new_node(Node::Literal);
  node.node_type = Token::NumLiteral;
  let EnumName: i32 = CurrentToken.token_Value;
  let EnumNode: i32 = scope_resolve(CurrentScope, EnumName, CurrentToken);
  next_token();
  eat_token(Token::DoubleColon);
  let EnumMember: i32 = CurrentToken.token_Value;
  let enum_value: i32 = list_search(EnumNode.node_Nodes, EnumMember);
  if !enum_value {
    add_error(Error::InvalidToken, CurrentToken);
  }
  node.node_String = i32_to_str(enum_value);
  next_token();
  node
}

fn parse_dot_load() -> i32 {
  let node: i32 = new_node(Node::DotLoad);
  let BodyList: i32 = new_list();
  node.node_Nodes = BodyList;
  list_add(BodyList, parse_identifier());
  while CurrentToken {
    if CurrentToken.token_kind.i32 != Token::Dot { break; }
    eat_token(Token::Dot);
    if is_native_type(CurrentToken) {
      node.node_dataType.i32 = CurrentToken.token_kind;
      next_token();
      break;
    } else {
      list_add(BodyList, parse_identifier());
    }
  }
  node
}

// A.B.C.i32 = x
fn parse_dot_store() -> i32 {
  let node: i32 = new_node(Node::DotStore);
  let BodyList: i32 = new_list();
  let mut dataType: i32 = 0;
  node.node_Nodes = BodyList;
  list_add(BodyList, parse_identifier());
  while CurrentToken {
    if CurrentToken.token_kind.i32 != Token::Dot { break; }
    eat_token(Token::Dot);
    if is_native_type(CurrentToken) {
      dataType = CurrentToken.token_kind;
      node.node_dataType = dataType;
      next_token();
      break;
    } else {
      list_add(BodyList, parse_identifier());
    }
  }
  eat_token(Token::Assign);
  node.node_ANode = parse_expression(Token::MinPrecedence);
  node.node_ANode.node_dataType = dataType;
  eat_token(Token::Semicolon);
  node
}

fn parse_prefix() -> i32 {
  let mut node: i32 = 0;
  let kind: i32 = CurrentToken.token_kind;
  if is_literal(CurrentToken) {
    node = parse_literal();
  } else if kind == Token::Identifier {
    let mut nextKind: i32 = 0;
    if NextToken {
       nextKind = NextToken.token_kind; 
    }
    if nextKind == Token::Dot {
      node = parse_dot_load();
    } else if nextKind == Token::DoubleColon {
      node = parse_double_colon();
    } else {
      node = parse_identifier();
    }
  } else if kind == Token::LParen {
    next_token();
    node = parse_expression(Token::MinPrecedence);
    eat_token(Token::RParen);
  } else if is_unary_op(CurrentToken) {
    node = parse_unary_expression();
  }
  node
}

fn parse_binary_expression(level: i32, Left: i32) -> i32 {
  let mut node: i32 = 0;
  let precedence: i32 = CurrentToken.token_kind;  // node_kind doubles as the precedence
  if level > precedence {
    node = Left;
  } else {
    node = new_node(Node::Binary);
    node.node_type.i32 = CurrentToken.token_kind;
    node.node_String.i32 = CurrentToken.token_Value;
    node.node_ANode = Left;
    next_token();
    node.node_BNode = parse_expression(precedence);
  }
  node
}

fn parse_assign_statement() -> i32 {
  let node: i32 = new_node(Node::Assign);
  node.node_ANode = parse_identifier();
  node.node_type = Token::Assign;
  node.node_String.i32 = CurrentToken.token_Value;
  eat_token(Token::Assign);
  node.node_BNode = parse_expression(Token::MinPrecedence);
  eat_token(Token::Semicolon);
  node
}

fn parse_infix(level: i32, Left: i32) -> i32 {
  let mut node: i32 = 0;
  if is_binary_op(CurrentToken) {
    node = parse_binary_expression(level, Left);
  } else if CurrentToken.token_kind.i32 == Token::LParen {
    node = parse_call_expression(Left);
    node.node_Token.i32 = Left.node_Token;
  } else {
    node = Left;
  }
  node
}

fn parse_call_statement() -> i32 {
  let IdentifierNode: i32 = parse_identifier();
  let node: i32 = parse_call_expression(IdentifierNode);
  eat_token(Token::Semicolon);
  node
}

// TODO: reintegrate
fn parse_breakif() -> i32 {
  let node: i32 = new_node(Node::BreakIf);
  node.node_CNode = parse_expression(Token::MinPrecedence);
  eat_token(Token::Semicolon);
  node
}

// TODO: reintegrate
fn parse_drop() -> i32 {
  let node: i32 = new_node(Node::Pop);
  node.node_CNode = parse_expression(Token::MinPrecedence);
  eat_token(Token::Semicolon);
  node
}

fn parse_expression(level: i32) -> i32 {
  let mut node: i32 = parse_prefix();
  while CurrentToken {
    let Expr: i32 = parse_infix(level, node);
    if Expr == 0 | Expr == node { break; }
    node = Expr;
  }
  node
}

fn parse_return_statement() -> i32 {
  let node: i32 = new_node(Node::Return);
  eat_token(Token::Return);
  node.node_ANode = parse_expression(Token::MinPrecedence);
  eat_token(Token::Semicolon);
  node
}

fn parse_return_expression() -> i32 {
  let node: i32 = new_node(Node::Return);
  let Expression: i32 = parse_expression(Token::MinPrecedence);
  node.node_ANode = Expression;
  if !Expression {
    add_error(Error::BlockStatement, CurrentToken);
  }
  node
}

fn parse_if_block() -> i32 {
  eat_token(Token::LBrace);
  let node: i32 = new_node(Node::Block);
  let BodyList: i32 = new_list();
  node.node_Nodes = BodyList;
  node.node_Scope = CurrentScope;
  while CurrentToken {
    if CurrentToken.token_kind.i32 == Token::RBrace { break; }
    let ChildNode: i32 = parse_statement();
    if !ChildNode { break; }
    list_add(BodyList, ChildNode);
  }
  eat_token(Token::RBrace);
  node
}

fn parse_if_statement() -> i32 {
  let node: i32 = new_node(Node::If);
  eat_token(Token::If);
  node.node_CNode = parse_expression(Token::MinPrecedence);
  push_scope(node);
  node.node_ANode = parse_if_block();
  pop_scope();
  if CurrentToken.token_kind.i32 == Token::Else {
    eat_token(Token::Else);
    push_scope(node);
    if CurrentToken.token_kind.i32 == Token::If {
      node.node_BNode = parse_if_statement();
    } else {
      node.node_BNode = parse_if_block();
    }
    pop_scope();
  }
  node
}

fn parse_loop_block() -> i32 {
  let node: i32 = new_node(Node::Block);
  let BodyList: i32 = new_list();
  node.node_Nodes = BodyList;
  node.node_Scope = CurrentScope;
  while CurrentToken {
    if CurrentToken.token_kind.i32 == Token::RBrace { break; }
    let ChildNode: i32 = parse_statement();
    if !ChildNode { break; }
    list_add(BodyList, ChildNode);
  }
  node
}

fn parse_loop_statement() -> i32 {
  let node: i32 = new_node(Node::Loop);
  eat_token(Token::Loop);
  eat_token(Token::LBrace);
  push_scope(node);
  node.node_ANode = parse_loop_block();
  pop_scope();
  eat_token(Token::RBrace);
  node
}

fn parse_while_statement() -> i32 {
  let node: i32 = new_node(Node::Loop);
  eat_token(Token::While);
  node.node_CNode = parse_expression(Token::MinPrecedence);
  eat_token(Token::LBrace);
  push_scope(node);
  node.node_ANode = parse_loop_block();
  pop_scope();
  eat_token(Token::RBrace);
  node
}

fn parse_const() -> i32 {
  eat_token(Token::Const);
  let name: i32 = CurrentToken.token_Value;
  let NameToken: i32 = CurrentToken;
  next_token();
  eat_token(Token::Colon);
  let type: i32 = CurrentToken.token_kind;
  next_token();
  let node: i32 = new_node(Node::Variable);
  node.node_type = type;
  node.node_dataType = type;
  node.node_String = name;
  scope_register_name(CurrentScope, name, node, NameToken);
  eat_token(Token::Assign);
  node.node_BNode = parse_expression(Token::MinPrecedence);
  if CurrentScope.scope_Parent.i32 {
    let fn_scope: i32 = get_fn_scope(CurrentScope);
    let FunNode: i32 = fn_scope.scope_Node;
    let mut FunLocalsList: i32 = FunNode.node_Nodes;
    if !FunLocalsList {
      FunLocalsList = new_list();
      FunNode.node_Nodes = FunLocalsList;
    }
    list_add(FunLocalsList, node);
  }
  eat_token(Token::Semicolon);
  node
}

fn parse_static() -> i32 {
  eat_token(Token::Static);
  let mutable: i32 = try_eat_token(Token::Mut);
  let name: i32 = CurrentToken.token_Value;
  let NameToken: i32 = CurrentToken;
  next_token();
  eat_token(Token::Colon);
  let type: i32 = CurrentToken.token_kind;
  next_token();
  let node: i32 = new_node(Node::Variable);
  node.node_type = type;
  node.node_dataType = type;
  node.node_String = name;
  if mutable {
    node.node_assigns = -1;
  } else {
    node.node_assigns = 1;
  }
  scope_register_name(CurrentScope, name, node, NameToken);
  eat_token(Token::Assign);
  node.node_BNode = parse_expression(Token::MinPrecedence);
  if CurrentScope.scope_Parent.i32 {
    let fn_scope: i32 = get_fn_scope(CurrentScope);
    let FunNode: i32 = fn_scope.scope_Node;
    let mut FunLocalsList: i32 = FunNode.node_Nodes;
    if !FunLocalsList {
      FunLocalsList = new_list();
      FunNode.node_Nodes = FunLocalsList;
    }
    list_add(FunLocalsList, node);
  }
  eat_token(Token::Semicolon);
  node
}

fn parse_declaration() -> i32 {
  eat_token(Token::Let);
  let mutable: i32 = try_eat_token(Token::Mut);
  let name: i32 = CurrentToken.token_Value;
  let NameToken: i32 = CurrentToken;
  next_token();
  eat_token(Token::Colon);
  let type: i32 = CurrentToken.token_kind;
  next_token();
  let node: i32 = new_node(Node::Variable);
  node.node_type = type;
  node.node_dataType = type;
  node.node_String = name;
  if mutable {
    node.node_assigns = -1;  // mutables have infinite assigns
  } else {
    node.node_assigns = 0;  // non-mutables can only be assigned once
  }
  scope_register_name(CurrentScope, name, node, NameToken);
  eat_token(Token::Assign);
  node.node_BNode = parse_expression(Token::MinPrecedence);
  if CurrentScope.scope_Parent.i32 {
    let fn_scope: i32 = get_fn_scope(CurrentScope);
    let FunNode: i32 = fn_scope.scope_Node;
    let mut FunLocalsList: i32 = FunNode.node_Nodes;
    if !FunLocalsList {
      FunLocalsList = new_list();
      FunNode.node_Nodes = FunLocalsList;
    }
    list_add(FunLocalsList, node);
  }
  eat_token(Token::Semicolon);
  node
}

// TODO: delete
fn parse_data() -> i32 {
  let node: i32 = new_node(Node::Data);
  if CurrentToken.token_kind.i32 == Token::NumLiteral {
    let OffsetToken: i32 = CurrentToken;
    node.node_ANode = OffsetToken;
    next_token();
    if CurrentToken.token_kind.i32 == Token::StrLiteral {
      list_add_name(DataList, OffsetToken, CurrentToken);
      node.node_BNode = CurrentToken;
      next_token();
    }
  }
  node
}

fn parse_statement() -> i32 {
  let mut node: i32 = 0;
  let kind: i32 = CurrentToken.token_kind;
  if kind == Token::Let {
    node = parse_declaration();
  } else if kind == Token::If {
    node = parse_if_statement();
  } else if kind == Token::Loop {
    node = parse_loop_statement();
  } else if kind == Token::While {
    node = parse_while_statement();
  } else if kind == Token::Continue {
    node = parse_continue();
  } else if kind == Token::Break {
    node = parse_break();
  } else if kind == Token::Identifier & NextToken.token_kind == Token::Dot {
    node = parse_dot_store();
  } else if kind == Token::Identifier & NextToken.token_kind == Token::LParen {
    node = parse_call_statement();
  } else if kind == Token::Identifier & NextToken.token_kind == Token::Assign {
    node = parse_assign_statement();
  } else if kind == Token::Return {
    node = parse_return_statement();
  } else {
    node = parse_return_expression();
  }
  node
}

fn parse_root_statement() -> i32 {
  let mut node: i32 = 0;
  let kind: i32 = CurrentToken.token_kind;
  if kind == Token::Fun {
    node = parse_fn();
  } else if kind == Token::Const {
    node = parse_const();
  } else if kind == Token::Static {
    node = parse_static();
  } else if kind == Token::Enum {
    node = parse_enum();
  } else if kind == Token::Pub {
    node = parse_fn();
  } else {
    add_error(Error::RootStatement, CurrentToken);
  }
  node
}

fn parse() {
  RootNode = new_node(Node::Module);
  ExportList = new_list();
  DataList = new_list();
  CurrentTokenItem = TokenList.list_First;
  CurrentToken = CurrentTokenItem.item_Object;
  push_scope(RootNode);
  GlobalScope = CurrentScope;
  let BodyList: i32 = new_list();
  RootNode.node_Nodes = BodyList;
  while CurrentToken {
    let Child: i32 = parse_root_statement();
    if !Child { break; }
    list_add(BodyList, Child);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Compiler 

static mut CurrentFunNode: i32 = 0;
static mut TypeList: i32 = 0;
static mut FunTypeList: i32 = 0;
static mut emitGlobalCount: i32 = 0;

fn emit(dwasm: i32) {
  WASM = new_empty_string(dwasm.string_length + 256);  // Guess
  CurrentScope = RootNode.node_Scope;
  emit_header();
  TypeList = new_list();
  FunTypeList = new_list();
  emit_type_section();
  emit_function_section();
  emit_memory_section();
  emit_global_section();
  emit_export_section();
  emit_code_section();
  emit_data_section();
}

fn emit_header() {
  append_i32(WASM, '\00asm');  // WASM magic: 00 61 73 6d
  append_i32(WASM, 1);         // WASM version
}

fn emit_type_section() {
  let BodyList: i32 = RootNode.node_Nodes;
  let skip: i32 = WASM.string_length;
  if BodyList {
    append_byte(WASM, 0x01);  // Type section
    append_byte(WASM, 0x00);  // section size (guess)
    let Start: i32 = WASM.string_length;
    append_byte(WASM, 0x00);  // types count (guess)  
    let mut index: i32 = 0;
    let mut Item: i32 = BodyList.list_First;
    while Item {
      let node: i32 = Item.item_Object;
      if node.node_kind.i32 == Node::Fun {
        emit_type(node, index);
        index = index + 1;
      }
      Item = Item.item_Next;
    }
    let count: i32 = TypeList.list_count;
    let length: i32 = WASM.string_length - Start;
    let offset: i32 = uleb_length(count) - 1 + uleb_length(length) - 1;
    offset_tail(WASM, Start, offset);
    WASM.string_length = Start - 1;
    append_uleb(WASM, length + uleb_length(count) - 1);
    append_uleb(WASM, count);
    WASM.string_length = WASM.string_length + length - 1;
  }
  if !FunTypeList.list_count.i32 { 
    WASM.string_length = skip;
  }
}

fn append_data_type(Str: i32, dataType: i32) {
  if dataType == Token::F64 {
    append_byte(Str, 0x7c);
  } else if dataType == Token::F32 {
    append_byte(Str, 0x7d);
  } else if dataType == Token::I64 {
    append_byte(Str, 0x7e);
  } else {
    append_byte(Str, 0x7f);
  }
}

fn emit_type(node: i32, funcNo: i32) {
  let ParamList: i32 = node.node_ParamNodes;
  let params: i32 = ParamList.list_count;
  let mut returns: i32 = 0;
  if node.node_type.bool { 
    returns = 1;
  }
  let TypeString: i32 = new_empty_string(1 + uleb_length(params) + params + uleb_length(returns) + returns);
  append_chr(TypeString, 0x60);  // fn type
  append_uleb(TypeString, params);
  let mut ParamItem: i32 = ParamList.list_First;
  while ParamItem {
    let dataType: i32 = ParamItem.item_Object.node_type;
    append_data_type(TypeString, dataType);
    ParamItem = ParamItem.item_Next;
  }
  let returnType: i32 = node.node_type;
  if returnType {
    append_uleb(TypeString, 0x01);  // return count
    append_data_type(TypeString, returnType);
  } else {
    append_uleb(TypeString, 0x00);  // return count
  }
  let mut typeIndex: i32 = index_list_search(TypeList, TypeString);
  if typeIndex == -1 {
    typeIndex = TypeList.list_count;
    list_add_name(TypeList, 0, TypeString);
    append_str(WASM, TypeString);
  }
  list_add(FunTypeList, typeIndex);
}

fn emit_function_section() {
  let funCount: i32 = FunTypeList.list_count;
  if funCount {
    append_byte(WASM, 0x03);  // Function section
    append_byte(WASM, 0x00);  // section size (guess)
    let start: i32 = WASM.string_length;
    append_uleb(WASM, funCount);  // types count
    let mut FunType: i32 = FunTypeList.list_First;
    while FunType {
      append_uleb(WASM, FunType.item_Object);
      FunType = FunType.item_Next;
    }
    let length: i32 = WASM.string_length - start;
    let offset: i32 = uleb_length(length) - 1;
    offset_tail(WASM, start, offset);
    WASM.string_length = start - 1;
    append_uleb(WASM, length);
    WASM.string_length = WASM.string_length + length;
  }
}

fn emit_memory_section() {
  append_byte(WASM, 0x05);  // Memory section
  append_uleb(WASM, 2 + uleb_length(1000));  // Size in bytes
  append_byte(WASM, 0x01);    // Count
  append_byte(WASM, 0x00);    // Resizable
  append_uleb(WASM, 1000);  // Pages
}

fn emit_global_section() {
  let skip: i32 = WASM.string_length;
  if RootNode.node_Nodes.i32 {
    append_byte(WASM, 0x06);  // Section code
    append_byte(WASM, 0x00);  // Section size (guess)
    let start: i32 = WASM.string_length;
    append_byte(WASM, 0x00);  // Globals count (guess)
    let mut Item: i32 = RootNode.node_Nodes.list_First;
    let mut count: i32 = 0;
    while Item {
      if Item.item_Object.node_kind.i32 == Node::Variable {
        emit_native_global(Item.item_Object);
        count = count + 1;
        emitGlobalCount = emitGlobalCount + 1;
      }
      Item = Item.item_Next;
    }
    let length: i32 = WASM.string_length - start;
    let offset: i32 = uleb_length(count) - 1 + uleb_length(length) - 1;
    offset_tail(WASM, start, offset);
    WASM.string_length = start - 1;
    append_uleb(WASM, length + uleb_length(count) - 1);
    append_uleb(WASM, count);
    WASM.string_length = WASM.string_length + length - 1;
  }
  if !emitGlobalCount {
    WASM.string_length = skip;
  }
}

fn emit_native_global(node: i32) {
  let dataType: i32 = node.node_type;  // Native type
  if dataType == Token::F64 { 
    append_byte(WASM, 0x7c);
    append_byte(WASM, 0x01);  // Mutable
    append_byte(WASM, 0x44);  // f64.const
  } else if dataType == Token::F32 { 
    append_byte(WASM, 0x7d);
    append_byte(WASM, 0x01);  // Mutable
    append_byte(WASM, 0x43);  // f32.const
  } else if dataType == Token::I64 {
    append_byte(WASM, 0x7e);
    append_byte(WASM, 0x01);  // Mutable
    append_byte(WASM, 0x42);  // i64.const
  } else {  // i32, bool
    append_byte(WASM, 0x7f);
    append_byte(WASM, 0x01);  // Mutable
    append_byte(WASM, 0x41);  // i32.const
  }
  let text: i32 = node.node_BNode.node_String;
  let nodeType: i32 = node.node_BNode.node_type;
  if nodeType == Token::True {
    append_byte(WASM, 0x01); 
  } else if nodeType == Token::False { 
    append_byte(WASM, 0x00); 
  } else if dataType == Token::F64 {
    append_f64(WASM, str_to_f64(text));
  } else if dataType == Token::F32 {
    append_f32(WASM, str_to_f32(text));
  } else if dataType == Token::I64 {
    append_sleb64(WASM, str_to_i64(text, node.node_BNode.node_Token));
  } else {
    append_sleb32(WASM, str_to_i32(text, node.node_BNode.node_Token));
  }
  append_byte(WASM, 0x0b);  // end
}

fn emit_export_section() {
  let BodyList: i32 = RootNode.node_Nodes;
  if BodyList {
    let mut count: i32 = ExportList.list_count;
    count = count + 1;  // +1 because we are also exporting the Memory
    if count {
      append_byte(WASM, 0x07);  // Export section
      append_byte(WASM, 0x00);  // Section size (guess)
      let start: i32 = WASM.string_length;
      append_uleb(WASM, count);  // Export count
      emit_export_mem();
      emit_export_fns();
      let length: i32 = WASM.string_length - start;
      let offset: i32 = uleb_length(length) - 1;
      offset_tail(WASM, start, offset);
      WASM.string_length = start - 1;
      append_uleb(WASM, length);
      WASM.string_length = WASM.string_length + length;
    }
  }
}

fn emit_export_fns() {
  let mut Item: i32 = ExportList.list_First;
  while Item {
    let name: i32 = Item.item_Name;
    append_uleb(WASM, name.string_length);
    append_str(WASM, name);
    append_byte(WASM, 0x00);  // Type: function
    append_uleb(WASM, Item.item_Object.node_index);
    Item = Item.item_Next;
  }
}

fn emit_export_mem() {
  append_uleb(WASM, 6);
  append_chr(WASM, 'memory');
  append_byte(WASM, 0x02);  // Type: memory
  append_byte(WASM, 0x00);  // Memory number 0 
}

fn emit_code_section() {
  if FunTypeList.list_count.i32 {
    append_byte(WASM, 0x0a);  // Code section
    append_byte(WASM, 0x00);  // Section size (guess)
    let start: i32 = WASM.string_length;
    append_uleb(WASM, FunTypeList.list_count);
    let mut FunItem: i32 = RootNode.node_Nodes.list_First;
    while FunItem {
      let FunNode: i32 = FunItem.item_Object;
      if FunNode.node_kind.i32 == Node::Fun {
        emit_fn_node(FunNode);
      }
      FunItem = FunItem.item_Next;
    }
    let length: i32 = WASM.string_length - start;
    let offset: i32 = uleb_length(length) - 1;
    offset_tail(WASM, start, offset);
    WASM.string_length = start - 1;
    append_uleb(WASM, length);
    WASM.string_length = WASM.string_length + length;
  }
}

fn emit_fn_node(node: i32) {
  CurrentFunNode = node;
  append_byte(WASM, 0x00);  // Function size (guess)
  let start: i32 = WASM.string_length;
  append_byte(WASM, 0x00);  // Local declaration count (guess)
  let LocalList: i32 = node.node_Nodes;
  let mut LocalItem: i32 = LocalList.list_First;
  let mut declCount: i32 = 0;
  while LocalItem {
    let dataType: i32 = LocalItem.item_Object.node_type;
    let mut count: i32 = 1;
    loop {
      let NextItem: i32 = LocalItem.item_Next;
      if !NextItem { break; }
      if dataType != NextItem.item_Object.node_type { break; }
      LocalItem = NextItem;
      count = count + 1;
    }
    append_uleb(WASM, count);  // count
    append_data_type(WASM, dataType);
    LocalItem = LocalItem.item_Next;
    declCount = declCount + 1;
  }
  emit_node(node.node_ANode);  // Body Block node
  append_byte(WASM, 0x0b);  // end
  let length: i32 = WASM.string_length - start;
  let offset: i32 = uleb_length(length) - 1 + uleb_length(declCount) - 1;
  offset_tail(WASM, start, offset);
  WASM.string_length = start - 1;
  append_uleb(WASM, length);
  append_uleb(WASM, declCount);
  WASM.string_length = WASM.string_length + length - 1;
}

fn emit_node(node: i32) {
  let kind: i32 = node.node_kind;
  if kind == Node::Block {
    emit_block(node);
  } else if kind == Node::Assign {
    emit_assign(node, false);
  } else if kind == Node::Unary {
    emit_unary(node);
  } else if kind == Node::Call {
    emit_call(node);
  } else if kind == Node::Return {
    emit_return(node);
  } else if kind == Node::If {
    emit_if(node);
  } else if kind == Node::BreakIf {
    emit_breakif(node);
  } else if kind == Node::Pop {
    emit_drop(node);
  } else if kind == Node::Loop {
    emit_loop(node);
  } else if kind == Node::Literal {
    emit_literal(node);
  } else if kind == Node::Identifier {
    emit_identifier(node);
  } else if kind == Node::DotLoad {
    emit_dot_load(node);
  } else if kind == Node::DotStore {
    emit_dot_store(node);
  } else if kind == Node::Variable {
    emit_variable(node);
  } else if kind == Node::Continue {
    append_byte(WASM, 0x0c);  // br
    append_uleb(WASM, scope_level(node, Node::Loop));
  } else if kind == Node::Break {
    append_byte(WASM, 0x0c);  // br
    append_uleb(WASM, scope_level(node, Node::Loop) + 1);
  } else {
    add_error(Error::EmitNode, node.node_Token);
  }
}

fn emit_expression(node: i32) {
  let kind: i32 = node.node_kind;
  if kind == Node::Binary {
    emit_binary(node);
  } else if kind == Node::Unary {
    emit_unary(node);
  } else if kind == Node::Call {
    emit_call(node);
  } else if kind == Node::Literal {
    emit_literal(node);
  } else if kind == Node::Identifier {
    emit_identifier(node);
  } else if kind == Node::DotLoad {
    emit_dot_load(node);
  } else if kind == Node::Variable {
    emit_variable(node);
  } else {
    add_error(Error::Expression, node.node_Token);
  }
}

fn emit_assign(node: i32, isExpression: bool) {
  let resolved_node: i32 = scope_resolve(CurrentScope, node.node_ANode.node_String, node.node_Token);
  let dataType: i32 = resolved_node.node_type;
  let BNode: i32 = node.node_BNode;
  let assigns: i32 = resolved_node.node_assigns;
  if assigns == 0 { 
    add_error(Error::NotMutable, node.node_Token);
  }
  if assigns > 0 {
    resolved_node.node_assigns = assigns - 1;
  }
  node.node_dataType = dataType;
  if BNode.node_dataType != 0 & BNode.node_dataType != dataType {
    add_error(Error::TypeMismatch, node.node_Token);
  }
  BNode.node_dataType = dataType;
  emit_expression(BNode);
  if resolved_node.node_Scope == GlobalScope {
    append_byte(WASM, 0x24);  // set_global
    if isExpression {
      append_uleb(WASM, resolved_node.node_index);
      append_byte(WASM, 0x23);  // get_global
    }
  } else {
    if isExpression {
      append_byte(WASM, 0x22);  // tee_local
    } else {
      append_byte(WASM, 0x21);  // set_local
    }
  }
  append_uleb(WASM, resolved_node.node_index);
}

fn emit_binary(node: i32) {
  let type: i32 = node.node_type;
  let mut dataType: i32 = node.node_dataType;
  let ANode: i32 = node.node_ANode;
  let BNode: i32 = node.node_BNode;
  if !dataType {
    dataType = infer_data_type(node);
    if !dataType {
      add_error(Error::TypeNotInferred, node.node_Token);
    }
    node.node_dataType = dataType;
  }
  ANode.node_dataType = dataType;
  BNode.node_dataType = dataType;
  emit_expression(ANode);
  emit_expression(BNode);
  emit_operatorS(type, dataType, node);
}

fn emit_operatorS(type: i32, dataType: i32, node: i32) {
  if dataType == Token::F64 {
    if type == Token::Eql { append_byte(WASM, 0x61); 
    } else if type == Token::Ne { append_byte(WASM, 0x62); 
    } else if type == Token::Lt { append_byte(WASM, 0x63); 
    } else if type == Token::Gt { append_byte(WASM, 0x64); 
    } else if type == Token::Le { append_byte(WASM, 0x65); 
    } else if type == Token::Ge { append_byte(WASM, 0x66); 
    } else if type == Token::Add { append_byte(WASM, 0xa0); 
    } else if type == Token::Sub { append_byte(WASM, 0xa1); 
    } else if type == Token::Mul { append_byte(WASM, 0xa2); 
    } else if type == Token::Div { append_byte(WASM, 0xa3); 
    } else if type == Token::Min { append_byte(WASM, 0xa4); 
    } else if type == Token::Max { append_byte(WASM, 0xa5); 
    } else if type == Token::Abs { append_byte(WASM, 0x99); 
    } else if type == Token::Neg { append_byte(WASM, 0x9a); 
    } else if type == Token::Sqrt { append_byte(WASM, 0x9f); 
    } else if type == Token::Ceil { append_byte(WASM, 0x9b); 
    } else if type == Token::Floor { append_byte(WASM, 0x9c); 
    } else if type == Token::Trunc { append_byte(WASM, 0x9d); 
    } else if type == Token::Round { append_byte(WASM, 0x9e); 
    } else if type == Token::CopySign { append_byte(WASM, 0xa6); 
    } else { 
      add_error(Error::InvalidOperator, node.node_Token); 
    }
  } else if dataType == Token::F32 {
    if type == Token::Eql { append_byte(WASM, 0x5b); 
    } else if type == Token::Ne { append_byte(WASM, 0x5c);
    } else if type == Token::Lt { append_byte(WASM, 0x5d);
    } else if type == Token::Gt { append_byte(WASM, 0x5e);
    } else if type == Token::Le { append_byte(WASM, 0x5f);
    } else if type == Token::Ge { append_byte(WASM, 0x60); 
    } else if type == Token::Abs { append_byte(WASM, 0x8b); 
    } else if type == Token::Neg { append_byte(WASM, 0x8c); 
    } else if type == Token::Ceil { append_byte(WASM, 0x8d);
    } else if type == Token::Floor { append_byte(WASM, 0x8e);
    } else if type == Token::Trunc { append_byte(WASM, 0x8f);
    } else if type == Token::Round { append_byte(WASM, 0x90);
    } else if type == Token::Sqrt { append_byte(WASM, 0x91);
    } else if type == Token::Add { append_byte(WASM, 0x92);
    } else if type == Token::Sub { append_byte(WASM, 0x93);
    } else if type == Token::Mul { append_byte(WASM, 0x94);
    } else if type == Token::Div { append_byte(WASM, 0x95);
    } else if type == Token::Min { append_byte(WASM, 0x96);
    } else if type == Token::Max { append_byte(WASM, 0x97);
    } else if type == Token::CopySign { append_byte(WASM, 0x98);
    } else {
      add_error(Error::InvalidOperator, node.node_Token); 
    }
  } else if dataType == Token::I64 {
    if type == Token::Not { append_byte(WASM, 0x50); 
    } else if type == Token::Eql { append_byte(WASM, 0x51); 
    } else if type == Token::Ne { append_byte(WASM, 0x52); 
    } else if type == Token::Lt { append_byte(WASM, 0x53); 
    } else if type == Token::Ltu { append_byte(WASM, 0x54); 
    } else if type == Token::Gt { append_byte(WASM, 0x55); 
    } else if type == Token::Gtu { append_byte(WASM, 0x56); 
    } else if type == Token::Le { append_byte(WASM, 0x57);
    } else if type == Token::Leu { append_byte(WASM, 0x58);
    } else if type == Token::Ge { append_byte(WASM, 0x59); 
    } else if type == Token::Geu { append_byte(WASM, 0x5a);
    } else if type == Token::Clz { append_byte(WASM, 0x79);
    } else if type == Token::Ctz { append_byte(WASM, 0x7a); 
    } else if type == Token::Cnt { append_byte(WASM, 0x7b);
    } else if type == Token::Add { append_byte(WASM, 0x7c);
    } else if type == Token::Sub { append_byte(WASM, 0x7d);
    } else if type == Token::Mul { append_byte(WASM, 0x7e);
    } else if type == Token::Div { append_byte(WASM, 0x7f);
    } else if type == Token::Divu { append_byte(WASM, 0x80);
    } else if type == Token::Rem { append_byte(WASM, 0x81);
    } else if type == Token::Remu { append_byte(WASM, 0x82);
    } else if type == Token::And { append_byte(WASM, 0x83);
    } else if type == Token::Or { append_byte(WASM, 0x84);
    } else if type == Token::Xor { append_byte(WASM, 0x85);
    } else if type == Token::Shl { append_byte(WASM, 0x86);
    } else if type == Token::Shr { append_byte(WASM, 0x87);
    } else if type == Token::Shru { append_byte(WASM, 0x88);
    } else if type == Token::Rotl { append_byte(WASM, 0x89);
    } else if type == Token::Rotr { append_byte(WASM, 0x8a); 
    } else {
      add_error(Error::InvalidOperator, node.node_Token); 
    }
  } else {
    if type == Token::Not { append_byte(WASM, 0x45); 
    } else if type == Token::Eql { append_byte(WASM, 0x46); 
    } else if type == Token::Ne { append_byte(WASM, 0x47); 
    } else if type == Token::Lt { append_byte(WASM, 0x48); 
    } else if type == Token::Ltu { append_byte(WASM, 0x49); 
    } else if type == Token::Gt { append_byte(WASM, 0x4a); 
    } else if type == Token::Gtu { append_byte(WASM, 0x4b); 
    } else if type == Token::Le { append_byte(WASM, 0x4c); 
    } else if type == Token::Leu { append_byte(WASM, 0x4d); 
    } else if type == Token::Ge { append_byte(WASM, 0x4e); 
    } else if type == Token::Geu { append_byte(WASM, 0x4f); 
    } else if type == Token::Clz { append_byte(WASM, 0x67); 
    } else if type == Token::Ctz { append_byte(WASM, 0x68); 
    } else if type == Token::Cnt { append_byte(WASM, 0x69); 
    } else if type == Token::Add { append_byte(WASM, 0x6a); 
    } else if type == Token::Sub { append_byte(WASM, 0x6b); 
    } else if type == Token::Mul { append_byte(WASM, 0x6c); 
    } else if type == Token::Div { append_byte(WASM, 0x6d); 
    } else if type == Token::Divu { append_byte(WASM, 0x6e); 
    } else if type == Token::Rem { append_byte(WASM, 0x6f); 
    } else if type == Token::Remu { append_byte(WASM, 0x70); 
    } else if type == Token::And { append_byte(WASM, 0x71); 
    } else if type == Token::Or { append_byte(WASM, 0x72); 
    } else if type == Token::Xor { append_byte(WASM, 0x73); 
    } else if type == Token::Shl { append_byte(WASM, 0x74); 
    } else if type == Token::Shr { append_byte(WASM, 0x75); 
    } else if type == Token::Shru { append_byte(WASM, 0x76); 
    } else if type == Token::Rotl { append_byte(WASM, 0x77); 
    } else if type == Token::Rotr { append_byte(WASM, 0x78); 
    } else { 
      add_error(Error::InvalidOperator, node.node_Token); 
    }
  }
}

fn emit_unary(node: i32) {
  let type: i32 = node.node_type;
  let dataType: i32 = node.node_dataType;
  if type == Token::Sub {
    if dataType == Token::F64 {
      append_byte(WASM, 0x44);  // f64.const
      append_f64(WASM, 0); 
    } else if dataType == Token::F32 {
      append_byte(WASM, 0x43);  // f32.const
      append_f32(WASM, 0);
    } else if dataType == Token::I64 {
      append_byte(WASM, 0x42);  // i64.const 
      append_byte(WASM, 0x00);  // 0
    } else {
      append_byte(WASM, 0x41);  // i32.const 
      append_byte(WASM, 0x00);  // 0
    }
  }
  emit_expression(node.node_BNode);
  emit_operatorS(type, dataType, node);
}

fn emit_identifier(node: i32) {
  let resolved_node: i32 = scope_resolve(CurrentScope, node.node_String, node.node_Token);
  let mut dataType: i32 = resolved_node.node_dataType;
  let mut nodeDataType: i32 = node.node_dataType;
  if dataType == Token::Bool {
    dataType = Token::I32;
  }
  if nodeDataType == Token::Bool {
    nodeDataType = Token::I32;
  }
  if nodeDataType != 0 & nodeDataType != dataType {
    add_error(Error::TypeMismatch, node.node_Token);
  }
  node.node_dataType = dataType;
  if resolved_node.node_Scope == GlobalScope {
    append_byte(WASM, 0x23);  // get_global
  } else {
    append_byte(WASM, 0x20);  // get_local
  }
  append_uleb(WASM, resolved_node.node_index);
}

// A.B.C.D
// loadX(load(load(A + B) + C) + D)
// A B + load() C + load() D + loadX()
fn emit_dot_load(node: i32) {
  let dataType: i32 = node.node_dataType;
  let IdentList: i32 = node.node_Nodes;
  let mut Item: i32 = IdentList.list_First;
  let itemCount: i32 = IdentList.list_count;
  let mut itemNo: i32 = 1;
  emit_identifier(Item.item_Object);
  Item = Item.item_Next;
  while Item {
    itemNo = itemNo + 1;
    emit_identifier(Item.item_Object);
    append_byte(WASM, 0x6a);  // i32.Add
    if itemNo < itemCount {
      append_byte(WASM, 0x28);  // i32.load
    } else {
      if !dataType {
        add_error(Error::TypeNotInferred, node.node_Token);
      }
      if dataType == Token::F64 {
        append_byte(WASM, 0x2b);  // f64.load
      } else if dataType == Token::F32 {
        append_byte(WASM, 0x2a);  // f32.load
      } else if dataType == Token::I64 {
        append_byte(WASM, 0x29);  // i64.load
      } else {
        append_byte(WASM, 0x28);  // i32.load
      }
    }
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
    Item = Item.item_Next;
  }
}

// A.B.C.D = x
// storeX(load(load(A + B) + C) + D, x)
// A B + load() C + load() D + x storeX()
fn emit_dot_store(node: i32) {
  let mut dataType: i32 = node.node_dataType;
  if !dataType {
    dataType = infer_data_type(node.node_ANode);
    node.node_dataType = dataType;
  }
  let IdentList: i32 = node.node_Nodes;
  if IdentList {
    let mut Item: i32 = IdentList.list_First;
    let itemCount: i32 = IdentList.list_count;
    let mut itemNo: i32 = 1;
    emit_identifier(Item.item_Object);
    Item = Item.item_Next;
    while Item {
      itemNo = itemNo + 1;
      emit_identifier(Item.item_Object);
      append_byte(WASM, 0x6a);  // i32.Add
      if itemNo < itemCount {
        append_byte(WASM, 0x28);  // i32.load
      } else {
        emit_expression(node.node_ANode);
        if dataType == Token::F64 {
          append_byte(WASM, 0x39);  // f64.store
        } else if dataType == Token::F32 {
          append_byte(WASM, 0x38);  // f32.store
        } else if dataType == Token::I64 {
          append_byte(WASM, 0x37);  // i64.store
        } else {
          append_byte(WASM, 0x36);  // i32.store
        }
      }
      append_byte(WASM, 0x00);  // alignment
      append_byte(WASM, 0x00);  // offset
      Item = Item.item_Next;
    }
  } else {
  add_error(Error::NoIdentifiers, node.node_Token);
  }
}

fn emit_num_literal(node: i32, dataType: i32) {
  if dataType == Token::F64 {
    append_byte(WASM, 0x44);  // f64.const
    append_f64(WASM, str_to_f64(node.node_String));
  } else if dataType == Token::F32 {
    append_byte(WASM, 0x43);  // f32.const
    append_f32(WASM, str_to_f32(node.node_String));
  } else if dataType == Token::I64 {
    append_byte(WASM, 0x42);  // i64.const
    append_sleb64(WASM, str_to_i64(node.node_String, node.node_Token));
  } else {
    append_byte(WASM, 0x41);  // i32.const
    append_sleb32(WASM, str_to_i32(node.node_String, node.node_Token));
  }
}

fn emit_chr_literal(node: i32, dataType: i32) {
  let name: i32 = node.node_String;
  if dataType == Token::I64 {
    append_byte(WASM, 0x42);  // i64.const
    if name.string_length.i32 > 4 {
      append_sleb64(WASM, load64(name + string_Chars));
    } else {
      append_sleb32(WASM, name.string_Chars);
    }
  } else {
    append_byte(WASM, 0x41);  // i32.const
    append_sleb32(WASM, name.string_Chars);
  }
}

fn emit_literal(node: i32) {
  let type: i32 = node.node_type;
  let dataType: i32 = node.node_dataType;
  if type == Token::NumLiteral {
    emit_num_literal(node, dataType);
  } else if type == Token::CharLiteral {
    emit_chr_literal(node, dataType);
  } else if type == Token::True {
    append_byte(WASM, 0x41);  // i32.const
    append_byte(WASM, 0x01);  // 1
  } else if type == Token::False {
    append_byte(WASM, 0x41);  // i32.const
    append_byte(WASM, 0x00);  // 0
  }
}

fn emit_fn_call_args(CallNode: i32, FunNode: i32) {
  let ArgumentList: i32 = CallNode.node_ParamNodes;
  if ArgumentList {
    let mut ArgumentItem: i32 = ArgumentList.list_First;
    let ParamList: i32 = FunNode.node_ParamNodes;
    if ParamList {
      let mut ParamItem: i32 = ParamList.list_First;
      while ArgumentItem {
        let ArgumentNode: i32 = ArgumentItem.item_Object;
        let ParamNode: i32 = ParamItem.item_Object;
        ArgumentNode.node_dataType.i32 = ParamNode.node_dataType;
        emit_expression(ArgumentNode);
        ArgumentItem = ArgumentItem.item_Next;
        ParamItem = ParamItem.item_Next;
      }
    } else {
      add_error(Error::NoParamList, CallNode.node_Token);
    }
  }
}

fn emit_call_args(CallNode: i32, data_Type: i32) {
  let ArgumentList: i32 = CallNode.node_ParamNodes;
  let mut ArgumentItem: i32 = ArgumentList.list_First;
  while ArgumentItem {
    let ArgumentNode: i32 = ArgumentItem.item_Object;
    ArgumentNode.node_dataType = data_Type;
    emit_expression(ArgumentNode);
    ArgumentItem = ArgumentItem.item_Next;
  }
}

fn emit_call_args2(CallNode: i32, data_TypeA: i32, data_TypeB: i32) {
  let ArgumentList: i32 = CallNode.node_ParamNodes;
  let mut ArgumentItem: i32 = ArgumentList.list_First;
  let mut isFirst: bool = true;
  while ArgumentItem {
    let ArgumentNode: i32 = ArgumentItem.item_Object;
    if isFirst {
      ArgumentNode.node_dataType = data_TypeA;
    } else {    
      ArgumentNode.node_dataType = data_TypeB;
    }
    emit_expression(ArgumentNode);
    ArgumentItem = ArgumentItem.item_Next;
    isFirst = false;
  }
}

fn emit_call(node: i32) {
  let name: i32 = node.node_ANode.node_String;
  if str_eq_char(name, 'i64_i32') {
    emit_call_args(node, Token::I64);
    append_byte(WASM, 0xa7);  // i32.wrap/i64
  } else if str_eq_char(name, 'f32_i32') {
    emit_call_args(node, Token::F32);
    append_byte(WASM, 0xa8);  // i32.trunc_s/f32
  } else if str_eq_char(name, 'f32_i32u') {
    emit_call_args(node, Token::F32);
    append_byte(WASM, 0xa9);  // i32.trunc_u/f32
  } else if str_eq_char(name, 'f64_i32') {
    emit_call_args(node, Token::F64);
    append_byte(WASM, 0xaa);  // i32.trunc_s/f64
  } else if str_eq_char(name, 'f64_i32u') {
    emit_call_args(node, Token::F64);
    append_byte(WASM, 0xab);  // i32.trunc_u/f64
  } else if str_eq_char(name, 'i32_i64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0xac);  // i64.extend_s/i32
  } else if str_eq_char(name, 'i32_i64u') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0xad);  // i64.extend_u/i32
  } else if str_eq_char(name, 'f32_i64') {
    emit_call_args(node, Token::F32);
    append_byte(WASM, 0xae);  // i64.trunc_s/f32
  } else if str_eq_char(name, 'f32_i64u') {
    emit_call_args(node, Token::F32);
    append_byte(WASM, 0xaf);  // i64.trunc_u/f32
  } else if str_eq_char(name, 'f64_i64') {
    emit_call_args(node, Token::F64);
    append_byte(WASM, 0xb0);  // i64.trunc_s/f64
  } else if str_eq_char(name, 'f64_i64u') {
    emit_call_args(node, Token::F64);
    append_byte(WASM, 0xb1);  // i64.trunc_u/f64
  } else if str_eq_char(name, 'i32_f32') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0xb2);  // f32.convert_s/i32    
  } else if str_eq_char(name, 'i32_f32u') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0xb3);  // f32.convert_u/i32   
  } else if str_eq_char(name, 'i64_f32') {
    emit_call_args(node, Token::I64);
    append_byte(WASM, 0xb4);  // f32.convert_s/i64
  } else if str_eq_char(name, 'i64_f32u') {
    emit_call_args(node, Token::I64);
    append_byte(WASM, 0xb5);  // f32.convert_u/i64
  } else if str_eq_char(name, 'f64_f32') {
    emit_call_args(node, Token::F64);
    append_byte(WASM, 0xb6);  // f32.demote/f64
  } else if str_eq_char(name, 'i32_f64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0xb7);  // f64.convert_s/i32
  } else if str_eq_char(name, 'i32_f64u') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0xb8);  // f64.convert_u/i32
  } else if str_eq_char(name, 'i64_f64') {
    emit_call_args(node, Token::I64);
    append_byte(WASM, 0xb9);  // f64.convert_s/i64
  } else if str_eq_char(name, 'i64_f64u') {
    emit_call_args(node, Token::I64);
    append_byte(WASM, 0xba);  // f64.convert_u/i64
  } else if str_eq_char(name, 'f32_f64') {
    emit_call_args(node, Token::F32);
    append_byte(WASM, 0xbb);  // f64.promote/f32
  } else if str_eq_char(name, 'load32') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x28);  // i32.load
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'load64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x29);  // i64.load
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'loadf32') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x2a);  // f32.load
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'loadf64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x2b);  // f64.load
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'load8') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x2c);  // i32.load8_s
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'load8u') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x2d);  // i32.load8_u
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'load16') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x2e);  // i32.load16_s
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'load16u') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x2f);  // i32.load16_u
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'loa8i64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x30);  // i64.load8_s
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'loa8u64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x31);  // i64.load8_u
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'loa16i64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x32);  // i64.load16_s
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset    
  } else if str_eq_char(name, 'loa16u64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x33);  // i64.load16_u
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset 
  } else if str_eq_char(name, 'loa32i64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x34);  // i64.load32_s
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset    
  } else if str_eq_char(name, 'loa32u64') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x35);  // i64.load32_u
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset    
  } else if str_eq_char(name, 'store32') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x36);  // i32.store
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'store64') {
    emit_call_args2(node, Token::I32, Token::I64);
    append_byte(WASM, 0x37);  // i64.store
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'storeF32') {
    emit_call_args2(node, Token::I32, Token::F32);
    append_byte(WASM, 0x38);  // f32.store
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'storeF64') {
    emit_call_args2(node, Token::I32, Token::F64);
    append_byte(WASM, 0x39);  // f64.store
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'store8') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x3a);  // i32.store8
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'store16') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x3b);  // i32.store16
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'i64sto8') {
    emit_call_args2(node, Token::I32, Token::I64);
    append_byte(WASM, 0x3c);  // i64.store8
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'i64sto16') {
    emit_call_args2(node, Token::I32, Token::I64);
    append_byte(WASM, 0x3d);  // i64.store16
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'i64sto32') {
    emit_call_args2(node, Token::I32, Token::I64);
    append_byte(WASM, 0x3e);  // i64.store32
    append_byte(WASM, 0x00);  // alignment
    append_byte(WASM, 0x00);  // offset
  } else if str_eq_char(name, 'memsize') {
    append_byte(WASM, 0x3f);  // current_memory
    append_byte(WASM, 0x00);  // memory number
  } else if str_eq_char(name, 'memgrow') {
    emit_call_args(node, Token::I32);
    append_byte(WASM, 0x40);  // grow_memory
    append_byte(WASM, 0x00);  // memory number
  } else {
    let resolved_node: i32 = scope_resolve(CurrentScope, name, node.node_Token);
    if resolved_node {
      emit_fn_call_args(node, resolved_node);
      append_byte(WASM, 0x10);  // Call
      append_uleb(WASM, resolved_node.node_index);
    }
  }
}

fn emit_block(node: i32) {
  let scope: i32 = node.node_Scope;
  CurrentScope = scope;
  let BlockList: i32 = node.node_Nodes;
  let mut Item: i32 = BlockList.list_First;
  while Item {
    emit_node(Item.item_Object);
    Item = Item.item_Next;
  }
  CurrentScope = scope.scope_Parent;
}

fn emit_if(node: i32) {
  emit_expression(node.node_CNode);  // If condition Expression
  append_byte(WASM, 0x04);  // if
  append_byte(WASM, 0x40);  // void
  emit_node(node.node_ANode);  // Then Block
  let ElseBlock: i32 = node.node_BNode;
  if ElseBlock {
    append_byte(WASM, 0x05);  // else
    emit_node(ElseBlock);
  }
  append_byte(WASM, 0x0b);  // end
}

fn scope_level(node: i32, kind: i32) -> i32 {
  let mut scope: i32 = node.node_Scope;
  let mut level: i32 = 0;
  while scope {
    if scope.scope_Node.node_kind == kind { break; }
    level = level + 1;
    scope = scope.scope_Parent;
  }
  level
}

fn emit_loop(node: i32) {
  append_byte(WASM, 0x02);  // Block
  append_byte(WASM, 0x40);  // void 
  append_byte(WASM, 0x03);  // loop
  append_byte(WASM, 0x40);  // void 
  let WhileNode: i32 = node.node_CNode;
  if WhileNode {
    emit_expression(WhileNode);
    let mut dataType: i32 = WhileNode.node_dataType;
    if !dataType {
      dataType = infer_data_type(WhileNode);
      if !dataType {
        add_error(Error::TypeNotInferred, WhileNode.node_Token);
      }
      WhileNode.node_dataType = dataType;
    }
    emit_operatorS(Token::Not, dataType, WhileNode);
    append_byte(WASM, 0x0d);  // br_if
    append_uleb(WASM, scope_level(node, Node::Loop) + 1);
  }
  emit_node(node.node_ANode);
  append_byte(WASM, 0x0c);  // br
  append_byte(WASM, 0x00);  // level 
  append_byte(WASM, 0x0b);  // end
  append_byte(WASM, 0x0b);  // end
}

fn infer_call_data_type(node: i32) -> i32 {
  let name: i32 = node.node_String;
  if        str_eq_char(name, 'load64')   { return Token::I64;
  } else if str_eq_char(name, 'load32')   { return Token::I32;
  } else if str_eq_char(name, 'load8')    { return Token::I32;
  } else if str_eq_char(name, 'load8u')   { return Token::I32;
  } else if str_eq_char(name, 'memsize')  { return Token::I32;
  } else if str_eq_char(name, 'loa_f32')  { return Token::F32;
  } else if str_eq_char(name, 'loa_f64')  { return Token::F64;
  } else if str_eq_char(name, 'f32_i32')  { return Token::I32;
  } else if str_eq_char(name, 'f32_i32u') { return Token::I32;
  } else if str_eq_char(name, 'f64_i32')  { return Token::I32;
  } else if str_eq_char(name, 'f64_i32u') { return Token::I32;
  } else if str_eq_char(name, 'i32_i64')  { return Token::I64;
  } else if str_eq_char(name, 'i32_i64u') { return Token::I64;
  } else if str_eq_char(name, 'f32_i64')  { return Token::I64;
  } else if str_eq_char(name, 'f32_i64u') { return Token::I64;
  } else if str_eq_char(name, 'f64_i64')  { return Token::I64;
  } else if str_eq_char(name, 'f64_i64u') { return Token::I64;
  } else if str_eq_char(name, 'i32_f32')  { return Token::F32;
  } else if str_eq_char(name, 'i32_f32u') { return Token::F32;
  } else if str_eq_char(name, 'i64_f32')  { return Token::F32;
  } else if str_eq_char(name, 'i64_f32u') { return Token::F32;
  } else if str_eq_char(name, 'f64_f32')  { return Token::F32;
  } else if str_eq_char(name, 'i32_f64')  { return Token::F64;
  } else if str_eq_char(name, 'i32_f64u') { return Token::F64;
  } else if str_eq_char(name, 'i64_f64')  { return Token::F64;
  } else if str_eq_char(name, 'i64_f64u') { return Token::F64;
  } else if str_eq_char(name, 'f32_f64')  { return Token::F64;
  } else {
    let resolved_node: i32 = scope_resolve(CurrentScope, name, node.node_Token);
    return resolved_node.node_dataType;
  }
  0
}

fn infer_data_type(node: i32) -> i32 {
  let mut dataType: i32 = node.node_dataType;
  let kind: i32 = node.node_kind;
  if kind == Node::Binary | kind == Node::Iif | kind == Node::Assign {
    dataType = infer_data_type(node.node_ANode);
    if !dataType {
      dataType = infer_data_type(node.node_BNode);
    }
  } else if kind == Node::Identifier {
    let resolved_node: i32 = scope_resolve(CurrentScope, node.node_String, node.node_Token);
    dataType = resolved_node.node_dataType;
  } else if kind == Node::Unary {
    dataType = infer_data_type(node.node_BNode);
  } else if kind == Node::Call {
    dataType = infer_call_data_type(node.node_ANode);
  }
  dataType
}

fn emit_iif(node: i32) {
  let mut dataType: i32  = node.node_dataType;
  let ANode: i32  = node.node_ANode;
  let BNode: i32  = node.node_BNode;
  let CNode: i32  = node.node_CNode;
  if !dataType {
    dataType = infer_data_type(node);
    if !dataType {
      add_error(Error::TypeNotInferred, node.node_Token);
    }
    node.node_dataType = dataType;
  }
  ANode.node_dataType = dataType;
  BNode.node_dataType = dataType;
  emit_expression(ANode);
  emit_expression(BNode);
  emit_expression(CNode);
  append_byte(WASM, 0x1b);  // select
}

fn emit_variable(node: i32) {
  let type: i32  = node.node_type;
  let BNode: i32  = node.node_BNode;
  BNode.node_dataType = type;
  emit_expression(BNode);
  append_byte(WASM, 0x21);  // set_local
  append_uleb(WASM, node.node_index);
}

fn emit_return(node: i32) {
  let ANode: i32  = node.node_ANode;
  let dataType: i32  = CurrentFunNode.node_dataType;
  if dataType {
    node.node_dataType = dataType;
    ANode.node_dataType = dataType;
    emit_expression(ANode);
  }
  if scope_level(node, Node::Fun) > 0 {
    append_byte(WASM, 0x0f);  // return
  }
}

fn emit_breakif(node: i32) {
  emit_expression(node.node_CNode);  // If condition Expression
  append_byte(WASM, 0x0d);  // br_if
  append_uleb(WASM, scope_level(node, Node::Loop) + 1);
}

fn emit_drop(node: i32) {
  emit_expression(node.node_CNode);
  append_byte(WASM, 0x1a);  // drop
}

fn emit_data_section() {
  let count: i32 = DataList.list_count;
  if count {
    append_byte(WASM, 0x0b);  // Data section
    append_byte(WASM, 0x00);  // Section size (guess)
    let start: i32 = WASM.string_length;
    append_uleb(WASM, count);
    let mut DataItem: i32 = DataList.list_First;
    while DataItem {
      append_byte(WASM, 0x00);  // memory index 
      append_byte(WASM, 0x41);  // i32.const
      append_uleb(WASM, str_to_i32(DataItem.item_Object.token_Value, DataItem.item_Object));  // offset
      append_byte(WASM, 0x0b);  // end
      let DataString: i32 = DataItem.item_Name.token_Value;
      let dataLength: i32 = DataString.string_length;
      append_uleb(WASM, dataLength);
      append_str(WASM, DataString);
      DataItem = DataItem.item_Next;
    }
    let length: i32 = WASM.string_length - start;
    let offset: i32 = uleb_length(length) - 1;
    offset_tail(WASM, start, offset);
    WASM.string_length = start - 1;
    append_uleb(WASM, length);
    WASM.string_length = WASM.string_length + length;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ERRORS

fn add_error(errorNo: i32, token: i32) {
  list_add_name(ERRORS, token, errorNo);
}

fn parse_error_list() {
  let mut ErrorItem: i32 = ERRORS.list_First;
  if ErrorItem {
    let ErrorString: i32 = new_empty_string(1000);
    while ErrorItem {
      let token: i32 = ErrorItem.item_Object;
      let errorNo: i32 = ErrorItem.item_number;
      if errorNo == Error::DuplicateName {
        append_chr3(ErrorString, 'Duplicat', 'e identi', 'fier');
      } else if errorNo == Error::InvalidToken {
        append_chr2(ErrorString, 'Invalid ', 'token');
      } else if errorNo == Error::MissingToken {
        append_chr2(ErrorString, 'Missing ', 'token');
      } else if errorNo == Error::RootStatement {
        append_chr3(ErrorString, 'Invalid ', 'root sta', 'tement');
      } else if errorNo == Error::BlockStatement {
        append_chr3(ErrorString, 'Invalid ', 'Block st', 'atement');
      } else if errorNo == Error::TypeMismatch {
        append_chr2(ErrorString, 'Type mis', 'match');
      } else if errorNo == Error::NotDeclared {
        append_chr3(ErrorString, 'Identifi', 'er Not d', 'eclared');
      } else if errorNo == Error::LiteralToInt {
        append_chr3(ErrorString, 'Could no', 't conver', 't to int');
      } else if errorNo == Error::Expression {
        append_chr3(ErrorString, 'Expressi', 'on expec', 'ted');
      } else if errorNo == Error::TypeNotInferred {
        append_chr3(ErrorString, 'Could no', 't determ', 'ine type');
      } else if errorNo == Error::NotMutable {
        append_chr2(ErrorString, 'Not ', 'mutable');
      } else if errorNo == Error::NoParamList {
        append_chr3(ErrorString, 'No ', 'param ', 'list');
      } else if errorNo == Error::EmitNode {
        append_chr3(ErrorString, 'Unexpect', 'ed node ', 'type');
      } else if errorNo == Error::InvalidOperator {
        append_chr2(ErrorString, 'Invalid ', 'operator');
      } else {  
        append_chr(ErrorString, 'Error ');
        append_i32_to_str(ErrorString, errorNo);
      }
      if token {
        append_chr(ErrorString, ' line ');
        append_i32_to_str(ErrorString, token.token_line);
        append_chr(ErrorString, ' column ');
        if token.token_Value.i32 {
          append_i32_to_str(ErrorString, token.token_column - token.token_Value.string_length);
          append_chr(ErrorString, ' token ');
          append_str(ErrorString, token.token_Value);
        } else {
          append_i32_to_str(ErrorString, token.token_column);
        }
        append_chr(ErrorString, 13);
      }
      WASM = ErrorString;
      ErrorItem = ErrorItem.item_Next;
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Function library

fn str_to_i32(Str: i32, token: i32) -> i32 {
  return i64_i32(str_to_i64(Str, token));
}

fn str_to_i64(Str: i32, token: i32) -> i64 {  // Supports ints & 0x-prefixed hex
  let mut is_hex: bool = false;
  let mut i: i64 = 0;
  let length: i32 = Str.string_length;
  let mut offset: i32 = 0;
  let mut chr: i32 = 0;
  if length >= 3 {
    if get_chr(Str, 0) == '0' & get_chr(Str, 1) == 'x' {
      is_hex = true;
    }
  }
  if is_hex {
    offset = 2;
    while offset < length {
      i = i * 16;
      chr = get_chr(Str, offset);
      if chr >= '0' & chr <= '9' {
        i = i + i32_i64(chr) - '0';
      } else if chr >= 'a' & chr <= 'f' {
        i = i + i32_i64(chr) - 'a' + 10;
      } else if chr >= 'A' & chr <= 'F' {
        i = i + i32_i64(chr) - 'A' + 10;
      } else {
        add_error(Error::LiteralToInt, token);
      }
      offset = offset + 1;
    }
  } else {
    while offset < length {
      i = i * 10;
      chr = get_chr(Str, offset);
      if chr >= '0' & chr <= '9' {
        i = i + i32_i64(chr) - '0';
      } else if offset == 0 & chr == '-' {
      } else {
        add_error(Error::LiteralToInt, token);
      }
      offset = offset + 1;
    }
  }
  if get_chr(Str, 0) == '-' { 
    i = -i;
  }
  i
}

fn str_to_f32(Str: i32) -> f32 {
  return f64_f32(str_to_f64(Str));
}

fn str_to_f64(Str: i32) -> f64 {
  let mut f: f64 = f;
  let length: i32 = Str.string_length;
  let mut offset: i32 = 0;
  let mut d: f64 = 1;
  let mut isAfterDot: bool = false;
  while offset < length {
    let chr: i32 = get_chr(Str, offset);
    if chr == '.' {
      isAfterDot = true;
    } else {
      if isAfterDot { 
        f = f + i32_f64(chr - '0') / d;
        d = d * 10;
      } else {
        if chr >= '0' & chr <= '9' {
          f = f * 10 + i32_f64(chr - '0');
        }
      }
    }
    offset = offset + 1;
  }
  if get_chr(Str, 0) == '-' { 
    f = -f; 
  }
  f
}

fn uleb_length(i: i32) -> i32 {
  if i >+ 268435456 {
    return 5;
  } else if i >+ 2097151 { 
    return 4; 
  } else if i >+ 16383 {
    return 3;
  } else if i >+ 127 {
    return 2;
  }
  1
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Strings

// Structs
const string_dec0de: i32 = 0;
const string_max:    i32 = 4;
const string_length: i32 = 8;
const string_Chars:  i32 = 12;
const string_size:   i32 = 12;

// Pascal-style strings: We store the length instead of using a null terminator
fn new_string(length: i32) -> i32 {
  let Str: i32 = allocate(string_size + length);
  Str.string_dec0de = 7 - DEC0DE;
  Str.string_max = length;
  Str.string_length = length;
  Str
}

fn new_empty_string(maxLength: i32) -> i32 {
  let Str: i32 = allocate(string_size + maxLength);
  Str.string_dec0de = 7 - DEC0DE;
  Str.string_max = maxLength;
  Str.string_length = 0;
  Str
}

fn append_str(Str: i32, AppendString: i32) {
  let appendLength: i32 = AppendString.string_length;
  let maxLength: i32 = Str.string_max;
  let mut offset: i32 = 0;
  while offset < appendLength {
    append_byte(Str, get_chr(AppendString, offset));
    if Str.string_length >= maxLength { break; }
    offset = offset + 1;
  }
}

fn append_i32_to_str(Str: i32, i: i32) {
  let length: i32 = Str.string_length;
  let appendLength: i32 = decimal_str_length(i);
  let mut offset: i32 = appendLength;
  if length + appendLength <= Str.string_max {
    while offset {
      let chr: i32 = '0' + i % 10;
      offset = offset - 1;
      set_chr(Str, length + offset, chr);
      i = i / 10;
      if !i { break; }
    }  
    Str.string_length = length + appendLength;
  }
}

fn i32_to_str(i: i32) -> i32 {
  let S: i32 = new_empty_string(12);
  append_i32_to_str(S, i);
  S
}

fn append_i32(Str: i32, i: i32) {
  let length: i32 = Str.string_length;
  if length + 4 <= Str.string_max {
    store32(Str + string_Chars + length, i);
    Str.string_length = length + 4;
  }
}

fn append_f32(Str: i32, f: f32) {
  let length: i32 = Str.string_length;
  if length + 4 <= Str.string_max {
    storeF32(Str + string_Chars + Str.string_length, f);
    Str.string_length = length + 4;
  }
}

fn append_f64(Str: i32, f: f64) {
  let length: i32 = Str.string_length;
  if length + 8 <= Str.string_max {
    storeF64(Str + string_Chars + length, f);
    Str.string_length = length + 8;
  }
}

fn append_byte(Str: i32, i: i32) {
  let length: i32 = Str.string_length;
  if length + 1 <= Str.string_max {
    store8(Str + string_Chars + length, i);
    Str.string_length = length + 1;
  }
}

fn append_chr(Str: i32, i: i64) {
  loop {
    let chr: i32 = i64_i32(i % 256);
    append_byte(Str, chr);
    i = i >>+ 8;
    if i == 0 { break; }
  }
}

fn append_chr2(Str: i32, i: i64, j: i64) {
  append_chr(Str, i);
  append_chr(Str, j);
}

fn append_chr3(Str: i32, i: i64, j: i64, k: i64) {
  append_chr(Str, i);
  append_chr(Str, j);
  append_chr(Str, k);
}

fn append_uleb(Str: i32, i: i32) {
  let length: i32 = Str.string_length;
  if length + uleb_length(i) <= Str.string_max {
    while i >=+ 128 {
      let chr: i32 = 128 + (i % 128);
      append_byte(Str, chr);
      i = i >>+ 7;
    }
    append_byte(Str, i);
  }
}

fn append_sleb32(Str: i32, i: i32) {
  append_sleb64(Str, i32_i64(i));
}

fn append_sleb64(Str: i32, mut i: i64) {
  if i >= 0 { 
    loop {
      if i < 64 { break; }
      append_byte(Str, i64_i32(128 + (i % 128)));
      i = i >> 7;
    }
    append_byte(Str, i64_i32(i));
  } else {
    loop {
      if i >= -64 { break; }
      append_byte(Str, i64_i32((i %+ 128) - 128));
      i = i >> 7;
    }
    append_byte(Str, i64_i32(i - 128));
  }
}

fn offset_tail(Str: i32, start: i32, offset: i32) {
  if offset > 0 {
    if Str.string_length + offset <= Str.string_max {
      Str.string_length = Str.string_length + offset;
      let mut copy: i32 = Str.string_length;
      while copy >= start {
        set_chr(Str, copy + offset, get_chr(Str, copy));
        copy = copy - 1;
      }
    }
  }
}

fn decimal_str_length(i: i32) -> i32 {
  let mut length: i32 = 1;
  loop {
    i = i / 10;
    if !i { break; }
    length = length + 1;
  }
  length
}

fn get_chr(Str: i32, offset: i32) -> i32 {
  return load8u(Str + string_Chars + offset);
}

fn set_chr(Str: i32, offset: i32, chr: i32) {
  store8(Str + string_Chars + offset, chr);
}

fn sub_str(Str: i32, offset: i32, mut length: i32) -> i32 {
  if offset >= Str.string_length {
    length = 0;
  }
  if offset + length >= Str.string_length {
    length = Str.string_length - offset;
  }
  let R: i32 = new_string(length);
  while length > 0 {
    length = length - 1;
    if offset + length >= 0 {
      set_chr(R, length, get_chr(Str, offset + length));
    }
  }
  R
}

fn str_eq(A: i32, B: i32) -> bool {
  let length: i32 = A.string_length;
  if length == B.string_length {
    let mut offset: i32 = 0;
    loop {
      if get_chr(A, offset) != get_chr(B, offset) {
        return false;
      }
      if offset >= length { break; }
      offset = offset + 1;
    }
  } else {
    return false;
  }
  true
}

fn str_eq_char(Str: i32, a: i64) -> bool {
  let length: i32 = Str.string_length;
  if length > 8 {
    return false;
  } else if length > 4 {
    if a != load64(Str + string_Chars) { return false; }
  } else if length > 0 {
    if a != i32_i64(load32(Str + string_Chars)) { return false; }
  } else {
    if a != 0 { return false; }
  }
  true
}

fn hex_chr_to_i32(chr: i32) -> i32 {
  if chr >= '0' & chr <= '9' {
    return chr - '0';
  } else if chr >= 'a' & chr <= 'f' {
    return chr - 'a' + 10;
  } else if chr >= 'A' & chr <= 'F' {
    return chr - 'A' + 10;
  }
  0
}

// Strings may contain escaped hex bytes for example "\5a" -> "Z"
fn decode_str(S: i32) {
  let length: i32 = S.string_length;
  let mut i: i32 = 0;
  let mut o: i32 = 0;
  while i < length {
    if get_chr(S, i) == 92 { // backslash
      i = i + 1;
      if is_number(get_chr(S, i), true) & is_number(get_chr(S, i + 1), true) {
        let mut chr: i32 = hex_chr_to_i32(get_chr(S, i));
        chr = chr * 16;
        chr = chr + hex_chr_to_i32(get_chr(S, i + 1));
        set_chr(S, o, chr);
        i = i + 1;
      }
    } else if i > o {
      set_chr(S, o, get_chr(S, i));
    }
    i = i + 1;
    o = o + 1;
  }
  S.string_length = o;
  while o < length {
    set_chr(S, o, 0);
    o = o + 1;
  }
}

fn is_alpha(chr: i32) -> bool {
  (chr >= 'a' & chr <= 'z') | (chr >= 'A' & chr <= 'Z') | (chr == '_')
}

fn is_number(chr: i32, hexNum: bool) -> bool {
  if chr >= '0' & chr <= '9' {
    return true;
  } else if hexNum {
    if (chr >= 'a' & chr <= 'f') | (chr >= 'A' & chr <= 'F') { 
      return true;
    }
  }
  false
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lists

// Structs
const list_dec0de: i32 = 0;  // debugging marker
const list_First:  i32 = 4;
const list_Last:   i32 = 8;
const list_count:  i32 = 12;
const list_size:   i32 = 16;

const item_dec0de: i32 = 0;  // debugging marker
const item_Next:   i32 = 4;
const item_Object: i32 = 8;
const item_Name:   i32 = 12;   const item_number: i32 = 12;
const item_size:   i32 = 16;

fn new_list() -> i32 {
  let List: i32 = allocate(list_size);
  List.list_dec0de = 4 - DEC0DE;
  List
}

fn list_add(List: i32, Object: i32) {
  let Item: i32 = allocate(item_size);
  Item.item_dec0de = 5 - DEC0DE;
  Item.item_Object = Object;
  if !List.list_First.i32 {
    List.list_First = Item;
  } else {
    List.list_Last.item_Next = Item;
  }
  List.list_Last = Item;
  List.list_count.i32 = List.list_count + 1;
}

fn list_add_name(List: i32, Object: i32, name: i32) {
  let Item: i32 = allocate(item_size);
  Item.item_dec0de = 5 - DEC0DE;
  Item.item_Object = Object;
  Item.item_Name = name;
  if !List.list_First.i32 {
    List.list_First = Item;
  } else {
    List.list_Last.item_Next = Item;
  }
  List.list_Last = Item;
  List.list_count.i32 = List.list_count + 1;
}

// Find a string in a list & return the object
fn list_search(List: i32, FindName: i32) -> i32 {
  let mut Item: i32 = List.list_First;
  while Item {
    if str_eq(Item.item_Name, FindName) {
      return Item.item_Object;
    }
    Item = Item.item_Next;
  }
  0
}

// Find a string in a list & return the index
fn index_list_search(List: i32, FindName: i32) -> i32 {
  let mut Item: i32 = List.list_First;
  let mut index: i32 = 0;
  while Item {
    if str_eq(Item.item_Name, FindName) {
      return index;
    }
    Item = Item.item_Next;
    index = index + 1;
  }
  -1
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Memory management

// Most browsers currently only support 32bit sized WASM memories
const SIZEINT: i32 = 4;

// Next free memory location
static mut Heap: i32 = 0;

fn allocate(length: i32) -> i32 {
  let R: i32 = Heap;
  Heap = Heap + length;
  if Heap % SIZEINT {
    Heap = Heap + SIZEINT - Heap % SIZEINT;  // Fix the alignment
  }
  R
}

// Pierre Rossouw 2017  https://github.com/PierreRossouw/rswasm