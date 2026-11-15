// Builds a consistency request from two clauses and prints the SMT-LIB query that
// filament-analyze would send to Z3 and cvc5. No solver is needed to lower a request.
use filament_analyze::{
    lower_analysis_request, AnalysisRequest, BindingGroup, BindingMember, StatementInput,
};
use filament_contract_model::{
    AnchorName, BooleanOperator, Clause, ClauseId, ClauseKind, ContractPackage,
    DeclarationEnvironment, ExecutionPoint, Expression, ExpressionKind, PackageId, Requirement,
    RequirementId, RequirementRef, RequirementRevision, SchemaVersion, SourceDocumentId,
    SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation, SymbolName,
    TypedExpression, ValueDeclaration, ValueDeclarationKind, ValueType,
};

fn span() -> SourceSpan {
    let source = SourceIdentity::new(
        SourceDocumentId::new("example").unwrap(),
        SourceRevision::new(1).unwrap(),
    );
    SourceSpan::new(
        SourceLocation::new(source.clone(), 1, 1, 0).unwrap(),
        SourceLocation::new(source, 1, 2, 1).unwrap(),
    )
    .unwrap()
}

fn point() -> ExecutionPoint {
    ExecutionPoint::Pre {
        operation: AnchorName::new("withdraw").unwrap().into(),
    }
}

fn input(name: &str) -> Expression {
    Expression::new(
        ExpressionKind::ValueReference {
            name: SymbolName::new(name).unwrap(),
            observation: StateObservation::Current,
        },
        span(),
    )
}

fn not(operand: Expression) -> Expression {
    Expression::new(
        ExpressionKind::BooleanNot {
            operand: Box::new(operand),
        },
        span(),
    )
}

fn and(left: Expression, right: Expression) -> Expression {
    Expression::new(
        ExpressionKind::Boolean {
            operator: BooleanOperator::TotalAnd,
            left: Box::new(left),
            right: Box::new(right),
        },
        span(),
    )
}

// One requirement with one assertion clause over the Boolean inputs `funded` and `frozen`.
fn statement(label: &str, expression: Expression) -> (StatementInput, DeclarationEnvironment) {
    let package_id = PackageId::new("example/account").unwrap();
    let requirement_id = RequirementId::new(format!("REQ-{label}")).unwrap();
    let revision = RequirementRevision::new(1).unwrap();
    let owner = RequirementRef::new(package_id.clone(), requirement_id.clone(), revision);
    let declarations = ["funded", "frozen"]
        .iter()
        .map(|name| {
            ValueDeclaration::new(
                SymbolName::new(*name).unwrap(),
                ValueDeclarationKind::Input,
                ValueType::Boolean,
                span(),
            )
        })
        .collect();
    let environment = DeclarationEnvironment::new(owner, vec![], declarations, vec![]).unwrap();
    let checked = environment
        .check_expression(&expression, &ValueType::Boolean, &point(), true)
        .unwrap();
    let clause = Clause::new(
        ClauseId::new(format!("C-{label}")).unwrap(),
        ClauseKind::Assertion,
        Some(point()),
        span(),
        checked,
    )
    .unwrap();
    let requirement = Requirement::<TypedExpression>::new(
        &package_id,
        requirement_id,
        revision,
        span(),
        vec![clause],
    )
    .unwrap();
    let package = ContractPackage::new(
        package_id,
        SchemaVersion::V1_1,
        span().source().clone(),
        vec![requirement],
    )
    .unwrap();
    let input = StatementInput::from_clause(
        &package,
        &package.requirements()[0],
        &package.requirements()[0].clauses()[0],
        environment.clone(),
    )
    .unwrap();
    (input, environment)
}

fn funded(environment: &DeclarationEnvironment) -> BindingMember {
    BindingMember::from_declaration(
        environment,
        &SymbolName::new("funded").unwrap(),
        StateObservation::Current,
        &point(),
    )
    .unwrap()
}

fn main() {
    // "funded and not frozen" and "funded": can both hold at once?
    let (pays, pays_env) = statement("pays", and(input("funded"), not(input("frozen"))));
    let (only_funded, funded_env) = statement("funded", input("funded"));

    // Each requirement declares its own `funded`, so without a binding group the
    // solver gets a separate variable for each.
    let unbound =
        AnalysisRequest::consistency(vec![], vec![pays.clone(), only_funded.clone()], vec![])
            .unwrap();
    println!(
        "variables without a binding: {}",
        lower_analysis_request(&unbound).unwrap().variables().len()
    );

    // A binding group says that two declarations are the same value.
    let group = BindingGroup::new("funded", vec![funded(&pays_env), funded(&funded_env)]).unwrap();
    let bound = AnalysisRequest::consistency(vec![], vec![pays, only_funded], vec![group]).unwrap();
    let query = lower_analysis_request(&bound).unwrap();
    println!("variables with a binding: {}", query.variables().len());
    println!("{}", query.query());
}
