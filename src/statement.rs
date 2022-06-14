use crate::expression::*;
use crate::interpreter::*;

use std::rc::Rc;

pub trait StatementVisitor {
    fn visit_declaration(&mut self, stmt: &DeclarationStatement) -> StatementResult;
    fn visit_assignment(&mut self, stmt: &AssignmentStatement) -> StatementResult;
    fn visit_if(&mut self, stmt: &IfStatement) -> StatementResult;
    fn visit_function(&mut self, stmt: &FunctionStatement) -> StatementResult;
    fn visit_while(&mut self, stmt: &WhileStatement) -> StatementResult;
    fn visit_block(&mut self, stmt: &BlockStatement) -> StatementResult;
    fn visit_expression(&mut self, stmt: &ExpressionStatement) -> StatementResult;
    fn visit_return(&mut self, stmt: &ReturnStatement) -> StatementResult;
    fn visit_internal(&mut self, stmt: &InternalStatement) -> StatementResult;
}

pub enum StatementValue {
    Normal(ScriptValue),
    Return(ScriptValue),
}

pub trait Statement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult;
}

pub struct DeclarationStatement {
    pub variables: Vec<String>,
    pub initializer: Option<Box<dyn Expression>>,
}

impl Statement for DeclarationStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_declaration(self)
    }
}

pub struct AssignmentStatement {
    pub assignee: Box<dyn Expression>,
    pub expr: Box<dyn Expression>,
}

impl Statement for AssignmentStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_assignment(self)
    }
}

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub if_body: Box<dyn Statement>,
    pub else_body: Option<Box<dyn Statement>>,
}

impl Statement for IfStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_if(self)
    }
}

pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Statement for WhileStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_while(self)
    }
}

pub struct FunctionStatement {
    pub name: String,
    pub params: Vec<String>,
    pub body: Rc<dyn Statement>,
}

impl Statement for FunctionStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_function(self)
    }
}

pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_expression(self)
    }
}

pub struct BlockStatement {
    pub body: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_block(self)
    }
}

pub struct ReturnStatement {
    pub expr: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_return(self)
    }
}

pub type InternalFunction = fn(interpreter: &mut Interpreter) -> StatementResult;

pub struct InternalStatement {
    pub func: InternalFunction,
}

impl Statement for InternalStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) -> StatementResult {
        visitor.visit_internal(self)
    }
}
