use crate::models::{TestCase, TestSequence};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagExpression {
    Tag(String),
    And(Vec<TagExpression>),
    Or(Vec<TagExpression>),
    Not(Box<TagExpression>),
}

impl TagExpression {
    pub fn parse(input: &str) -> Result<Self, String> {
        let input = input.trim();

        if input.is_empty() {
            return Err("Empty tag expression".to_string());
        }

        Self::parse_or(input)
    }

    fn split_respecting_parens(input: &str, delimiter: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut i = 0;
        let chars: Vec<char> = input.chars().collect();

        while i < chars.len() {
            let ch = chars[i];

            if ch == '(' {
                depth += 1;
                current.push(ch);
                i += 1;
            } else if ch == ')' {
                depth -= 1;
                current.push(ch);
                i += 1;
            } else if depth == 0 {
                // Check if we're at the delimiter
                let remaining: String = chars[i..].iter().collect();
                if remaining.starts_with(delimiter) {
                    // Found delimiter at top level
                    parts.push(current.trim().to_string());
                    current = String::new();
                    i += delimiter.len();
                } else {
                    current.push(ch);
                    i += 1;
                }
            } else {
                current.push(ch);
                i += 1;
            }
        }

        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }

        if parts.is_empty() {
            parts.push(input.trim().to_string());
        }

        parts
    }

    fn parse_or(input: &str) -> Result<Self, String> {
        let parts = Self::split_respecting_parens(input, "||");

        if parts.len() == 1 {
            return Self::parse_and(&parts[0]);
        }

        let mut expressions = Vec::new();
        for part in parts {
            expressions.push(Self::parse_and(&part)?);
        }

        Ok(TagExpression::Or(expressions))
    }

    fn parse_and(input: &str) -> Result<Self, String> {
        let parts = Self::split_respecting_parens(input, "&&");

        if parts.len() == 1 {
            return Self::parse_not(&parts[0]);
        }

        let mut expressions = Vec::new();
        for part in parts {
            expressions.push(Self::parse_not(&part)?);
        }

        Ok(TagExpression::And(expressions))
    }

    fn parse_not(input: &str) -> Result<Self, String> {
        let input = input.trim();

        if let Some(stripped) = input.strip_prefix('!') {
            let inner = stripped.trim();
            return Ok(TagExpression::Not(Box::new(Self::parse_atom(inner)?)));
        }

        Self::parse_atom(input)
    }

    fn parse_atom(input: &str) -> Result<Self, String> {
        let input = input.trim();

        if input.starts_with('(') && input.ends_with(')') {
            return Self::parse(&input[1..input.len() - 1]);
        }

        if input.is_empty() {
            return Err("Empty tag name".to_string());
        }

        Ok(TagExpression::Tag(input.to_string()))
    }

    pub fn evaluate(&self, tags: &HashSet<String>) -> bool {
        match self {
            TagExpression::Tag(tag) => tags.contains(tag),
            TagExpression::And(exprs) => exprs.iter().all(|e| e.evaluate(tags)),
            TagExpression::Or(exprs) => exprs.iter().any(|e| e.evaluate(tags)),
            TagExpression::Not(expr) => !expr.evaluate(tags),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagFilter {
    expression: Option<TagExpression>,
    include_tags: HashSet<String>,
    exclude_tags: HashSet<String>,
}

impl TagFilter {
    pub fn new() -> Self {
        Self {
            expression: None,
            include_tags: HashSet::new(),
            exclude_tags: HashSet::new(),
        }
    }

    pub fn with_expression(mut self, expression: TagExpression) -> Self {
        self.expression = Some(expression);
        self
    }

    pub fn with_include_tags(mut self, tags: Vec<String>) -> Self {
        self.include_tags = tags.into_iter().collect();
        self
    }

    pub fn with_exclude_tags(mut self, tags: Vec<String>) -> Self {
        self.exclude_tags = tags.into_iter().collect();
        self
    }

    pub fn matches_test_case(&self, test_case: &TestCase) -> bool {
        let all_tags = self.collect_test_case_tags(test_case);
        self.matches_tags(&all_tags)
    }

    pub fn matches_sequence(&self, sequence: &TestSequence, inherited_tags: &[String]) -> bool {
        let mut all_tags: HashSet<String> = inherited_tags.iter().cloned().collect();
        all_tags.extend(sequence.tags.iter().cloned());
        self.matches_tags(&all_tags)
    }

    fn matches_tags(&self, tags: &HashSet<String>) -> bool {
        if !self.exclude_tags.is_empty()
            && tags.iter().any(|t| self.exclude_tags.contains(t))
        {
            return false;
        }

        if !self.include_tags.is_empty()
            && !self.include_tags.iter().any(|t| tags.contains(t))
        {
            return false;
        }

        if let Some(ref expr) = self.expression {
            if !expr.evaluate(tags) {
                return false;
            }
        }

        true
    }

    fn collect_test_case_tags(&self, test_case: &TestCase) -> HashSet<String> {
        let mut tags: HashSet<String> = test_case.tags.iter().cloned().collect();

        for sequence in &test_case.test_sequences {
            tags.extend(sequence.tags.iter().cloned());
        }

        tags
    }

    pub fn is_empty(&self) -> bool {
        self.expression.is_none() && self.include_tags.is_empty() && self.exclude_tags.is_empty()
    }
}

impl Default for TagFilter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TagInheritance;

impl TagInheritance {
    pub fn get_effective_tags(test_case: &TestCase, sequence: &TestSequence) -> HashSet<String> {
        let mut tags: HashSet<String> = test_case.tags.iter().cloned().collect();
        tags.extend(sequence.tags.iter().cloned());
        tags
    }

    pub fn get_test_case_tags(test_case: &TestCase) -> HashSet<String> {
        test_case.tags.iter().cloned().collect()
    }

    pub fn get_all_tags_in_test_case(test_case: &TestCase) -> HashSet<String> {
        let mut tags: HashSet<String> = test_case.tags.iter().cloned().collect();

        for sequence in &test_case.test_sequences {
            tags.extend(sequence.tags.iter().cloned());
        }

        tags
    }
}

type TagRule = Box<dyn Fn(&TestCase) -> bool + Send + Sync>;

pub struct DynamicTagEvaluator {
    rules: HashMap<String, TagRule>,
}

impl DynamicTagEvaluator {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn with_default_rules() -> Self {
        let mut evaluator = Self::new();

        evaluator.add_rule(
            "multi-sequence",
            Box::new(|tc: &TestCase| tc.test_sequences.len() > 1),
        );

        evaluator.add_rule(
            "single-sequence",
            Box::new(|tc: &TestCase| tc.test_sequences.len() == 1),
        );

        evaluator.add_rule(
            "has-manual-steps",
            Box::new(|tc: &TestCase| {
                tc.test_sequences
                    .iter()
                    .any(|seq| seq.steps.iter().any(|step| step.manual == Some(true)))
            }),
        );

        evaluator.add_rule(
            "automated-only",
            Box::new(|tc: &TestCase| {
                !tc.test_sequences
                    .iter()
                    .any(|seq| seq.steps.iter().any(|step| step.manual == Some(true)))
            }),
        );

        evaluator.add_rule(
            "has-initial-conditions",
            Box::new(|tc: &TestCase| {
                !tc.initial_conditions.is_empty() || !tc.general_initial_conditions.is_empty()
            }),
        );

        evaluator
    }

    pub fn add_rule<F>(&mut self, tag: &str, rule: Box<F>)
    where
        F: Fn(&TestCase) -> bool + Send + Sync + 'static,
    {
        self.rules.insert(tag.to_string(), rule);
    }

    pub fn evaluate(&self, test_case: &TestCase) -> HashSet<String> {
        let mut dynamic_tags = HashSet::new();

        for (tag, rule) in &self.rules {
            if rule(test_case) {
                dynamic_tags.insert(tag.clone());
            }
        }

        dynamic_tags
    }

    pub fn get_all_tags(&self, test_case: &TestCase) -> HashSet<String> {
        let mut all_tags = TagInheritance::get_all_tags_in_test_case(test_case);
        all_tags.extend(self.evaluate(test_case));
        all_tags
    }
}

impl Default for DynamicTagEvaluator {
    fn default() -> Self {
        Self::with_default_rules()
    }
}

pub fn filter_test_cases(
    test_cases: Vec<TestCase>,
    filter: &TagFilter,
    evaluator: Option<&DynamicTagEvaluator>,
) -> Vec<TestCase> {
    if filter.is_empty() {
        return test_cases;
    }

    test_cases
        .into_iter()
        .filter(|tc| {
            let mut tags = TagInheritance::get_all_tags_in_test_case(tc);

            if let Some(eval) = evaluator {
                tags.extend(eval.evaluate(tc));
            }

            filter.matches_tags(&tags)
        })
        .collect()
}

pub fn filter_sequences_in_test_case(
    test_case: &mut TestCase,
    filter: &TagFilter,
    evaluator: Option<&DynamicTagEvaluator>,
) {
    if filter.is_empty() {
        return;
    }

    let inherited_tags: Vec<String> = if let Some(eval) = evaluator {
        let mut tags = test_case.tags.clone();
        tags.extend(eval.evaluate(test_case));
        tags
    } else {
        test_case.tags.clone()
    };

    test_case
        .test_sequences
        .retain(|seq| filter.matches_sequence(seq, &inherited_tags));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_expression_single_tag() {
        let expr = TagExpression::parse("smoke").unwrap();
        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());

        assert!(expr.evaluate(&tags));

        tags.clear();
        tags.insert("regression".to_string());
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_tag_expression_and() {
        let expr = TagExpression::parse("smoke && fast").unwrap();

        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());
        tags.insert("fast".to_string());
        assert!(expr.evaluate(&tags));

        tags.clear();
        tags.insert("smoke".to_string());
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_tag_expression_or() {
        let expr = TagExpression::parse("smoke || regression").unwrap();

        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());
        assert!(expr.evaluate(&tags));

        tags.clear();
        tags.insert("regression".to_string());
        assert!(expr.evaluate(&tags));

        tags.clear();
        tags.insert("integration".to_string());
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_tag_expression_not() {
        let expr = TagExpression::parse("!slow").unwrap();

        let mut tags = HashSet::new();
        tags.insert("fast".to_string());
        assert!(expr.evaluate(&tags));

        tags.clear();
        tags.insert("slow".to_string());
        assert!(!expr.evaluate(&tags));
    }

    #[test]
    fn test_tag_expression_complex() {
        let expr = TagExpression::parse("(smoke || regression) && !slow").unwrap();

        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());
        tags.insert("fast".to_string());
        assert!(expr.evaluate(&tags));

        tags.clear();
        tags.insert("smoke".to_string());
        tags.insert("slow".to_string());
        assert!(!expr.evaluate(&tags));

        tags.clear();
        tags.insert("regression".to_string());
        assert!(expr.evaluate(&tags));
    }

    #[test]
    fn test_tag_filter_include() {
        let filter = TagFilter::new().with_include_tags(vec!["smoke".to_string()]);

        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());
        tags.insert("fast".to_string());
        assert!(filter.matches_tags(&tags));

        tags.clear();
        tags.insert("regression".to_string());
        assert!(!filter.matches_tags(&tags));
    }

    #[test]
    fn test_tag_filter_exclude() {
        let filter = TagFilter::new().with_exclude_tags(vec!["slow".to_string()]);

        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());
        assert!(filter.matches_tags(&tags));

        tags.clear();
        tags.insert("slow".to_string());
        assert!(!filter.matches_tags(&tags));
    }

    #[test]
    fn test_tag_filter_expression() {
        let expr = TagExpression::parse("smoke && !slow").unwrap();
        let filter = TagFilter::new().with_expression(expr);

        let mut tags = HashSet::new();
        tags.insert("smoke".to_string());
        tags.insert("fast".to_string());
        assert!(filter.matches_tags(&tags));

        tags.clear();
        tags.insert("smoke".to_string());
        tags.insert("slow".to_string());
        assert!(!filter.matches_tags(&tags));
    }

    #[test]
    fn test_tag_inheritance() {
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test".to_string(),
        );
        test_case.tags = vec!["smoke".to_string(), "priority-high".to_string()];

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Desc".to_string());
        sequence.tags = vec!["fast".to_string()];

        let effective_tags = TagInheritance::get_effective_tags(&test_case, &sequence);

        assert!(effective_tags.contains("smoke"));
        assert!(effective_tags.contains("priority-high"));
        assert!(effective_tags.contains("fast"));
        assert_eq!(effective_tags.len(), 3);
    }

    #[test]
    fn test_dynamic_tag_evaluator() {
        let evaluator = DynamicTagEvaluator::with_default_rules();

        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test".to_string(),
        );

        let seq1 = TestSequence::new(1, "Seq1".to_string(), "Desc1".to_string());
        let seq2 = TestSequence::new(2, "Seq2".to_string(), "Desc2".to_string());
        test_case.test_sequences = vec![seq1, seq2];

        let dynamic_tags = evaluator.evaluate(&test_case);

        assert!(dynamic_tags.contains("multi-sequence"));
        assert!(!dynamic_tags.contains("single-sequence"));
        assert!(dynamic_tags.contains("automated-only"));
    }

    #[test]
    fn test_filter_test_cases() {
        let mut tc1 = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test1".to_string(),
        );
        tc1.tags = vec!["smoke".to_string()];

        let mut tc2 = TestCase::new(
            "REQ002".to_string(),
            1,
            2,
            "TC002".to_string(),
            "Test2".to_string(),
        );
        tc2.tags = vec!["regression".to_string()];

        let test_cases = vec![tc1, tc2];

        let filter = TagFilter::new().with_include_tags(vec!["smoke".to_string()]);

        let filtered = filter_test_cases(test_cases, &filter, None);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "TC001");
    }
}
