use ungrammar::{Grammar, Node, Rule, Token};

pub struct GenerateAstNodes<'a> {
    pub grammar: &'a Grammar,
}

impl<'a> GenerateAstNodes<'a> {
    pub fn generate(&self) -> String {
        let ast_source = self.get_ast_source();
        println!("{:#?}", ast_source);
        "".into()
    }

    fn get_ast_source(&self) -> AstSource {
        let nodes = self.grammar.iter().collect::<Vec<Node>>();

        let mut ast_source = AstSource {
            enums: vec![],
            nodes: vec![],
        };

        for &node in &nodes {
            let name = self.grammar[node].name.clone();
            let rule = &self.grammar[node].rule;

            match self.get_ast_enum_or_node_source(rule, name) {
                AstEnumOrNodeSource::AstEnumSource(enum_source) => {
                    ast_source.enums.push(enum_source);
                }
                AstEnumOrNodeSource::AstNodeSource(node_source) => {
                    ast_source.nodes.push(node_source);
                }
            }
        }
        ast_source
    }

    fn get_ast_enum_or_node_source(&self, rule: &Rule, name: String) -> AstEnumOrNodeSource {
        match extract_enum(self.grammar, rule) {
            Some(variants) => AstEnumOrNodeSource::AstEnumSource(AstEnumSource {
                name,
                traits: Vec::new(),
                variants,
            }),
            None => {
                let mut fields = Vec::new();
                lower_rule(&mut fields, self.grammar, None, rule);
                AstEnumOrNodeSource::AstNodeSource(AstNodeSource {
                    name,
                    traits: Vec::new(),
                    fields,
                })
            }
        }
    }
}

/// If the rule is Alt and all the alternatives are Nodes,
/// then we represent the rule as an Enum
fn extract_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    rule.get_alt().map_or(None, |alternatives| {
        let node_names = alternatives
            .iter()
            .map(|rule| rule.get_node())
            .flat_map(|opt| opt.map(|node| grammar[*node].name.clone()))
            .collect::<Vec<_>>();

        // All alternatives should be a node
        if node_names.len() == alternatives.len() {
            Some(node_names)
        } else {
            None
        }
    })
}

fn lower_rule(acc: &mut Vec<Field>, grammar: &Grammar, label: Option<&String>, rule: &Rule) {}

#[derive(Debug)]
struct AstSource {
    pub enums: Vec<AstEnumSource>,
    pub nodes: Vec<AstNodeSource>,
}

#[derive(Debug)]
enum AstEnumOrNodeSource {
    AstEnumSource(AstEnumSource),
    AstNodeSource(AstNodeSource),
}

#[derive(Debug)]
struct AstEnumSource {
    pub name: String,
    pub traits: Vec<String>,
    pub variants: Vec<String>,
}

#[derive(Debug)]
struct AstNodeSource {
    pub name: String,
    pub traits: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
enum Field {
    Token(String),
    Node {
        name: String,
        node_type: String,
        cardinality: Cardinality,
    },
}

#[derive(Debug)]
enum Cardinality {
    Optional,
    Many,
}

trait RuleExt {
    fn get_label(&self) -> Option<(&String, &Rule)>;
    fn get_node(&self) -> Option<&Node>;
    fn get_token(&self) -> Option<&Token>;
    fn get_seq(&self) -> Option<&Vec<Rule>>;
    fn get_alt(&self) -> Option<&Vec<Rule>>;
    fn get_opt(&self) -> Option<&Rule>;
    fn get_rep(&self) -> Option<&Rule>;
}

impl RuleExt for Rule {
    fn get_label(&self) -> Option<(&String, &Rule)> {
        todo!()
    }

    fn get_node(&self) -> Option<&Node> {
        match self {
            Rule::Node(node) => Some(node),
            _ => None,
        }
    }

    fn get_token(&self) -> Option<&Token> {
        todo!()
    }

    fn get_seq(&self) -> Option<&Vec<Rule>> {
        todo!()
    }

    fn get_alt(&self) -> Option<&Vec<Rule>> {
        match self {
            Rule::Alt(alternatives) => Some(alternatives),
            _ => None,
        }
    }

    fn get_opt(&self) -> Option<&Rule> {
        todo!()
    }

    fn get_rep(&self) -> Option<&Rule> {
        todo!()
    }
}
