use dialoguer::console::{Term, TermTarget};
use phper::arrays::ZArray;


pub trait AllowedForBoolAndString {}

impl AllowedForBoolAndString for String {}
impl AllowedForBoolAndString for bool {}

pub trait AllowedForZArrayAndString {}

impl AllowedForZArrayAndString for String {}
impl AllowedForZArrayAndString for ZArray {}


pub trait AllowedForTargetTerminalandTerminal {}

impl AllowedForTargetTerminalandTerminal for TermTarget {}
impl AllowedForTargetTerminalandTerminal for Term {}