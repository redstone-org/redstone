use ungrammar::{Grammar, Node, Rule};

pub struct GenerateAstNodes<'a> {
    pub grammar: &'a Grammar,
}

impl<'a> GenerateAstNodes<'a> {
    pub fn generate(&self) -> String {
        let ast_source = lower(self.grammar);
        println!("{:#?}", ast_source);
        "".into()
    }
}

fn lower(grammar: &Grammar) -> AstSource {
    let nodes = grammar.iter().collect::<Vec<Node>>();

    let mut ast_source = AstSource {
        enums: vec![],
        nodes: vec![],
    };

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;

        println!("-- {:?} {} --", node, name);
        println!("{:?}", rule);
        match lower_enum(grammar, rule) {
            Some(variants) => {
                let enum_src = AstEnumSource {
                    name,
                    traits: Vec::new(),
                    variants,
                };
                ast_source.enums.push(enum_src);
            }
            None => {
                let fields = Vec::new();
                ast_source.nodes.push(AstNodeSource {
                    name,
                    traits: Vec::new(),
                    fields,
                });
            }
        }
        println!("\n\n");
    }

    ast_source
}

fn lower_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    let alternatives = match rule {
        Rule::Alt(it) => it,
        _ => return None,
    };
    let mut variants = Vec::new();
    for alternative in alternatives {
        match alternative {
            Rule::Node(it) => variants.push(grammar[*it].name.clone()),
            _ => return None,
        }
    }
    Some(variants)
}

#[derive(Debug)]
struct AstSource {
    pub enums: Vec<AstEnumSource>,
    pub nodes: Vec<AstNodeSource>,
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
    pub fields: Vec<String>,
}
