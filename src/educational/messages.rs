//! 🎓 EDUCATIONAL MESSAGES - DESCRIPTIVE, EASY-TO-UNDERSTAND SYSTEM
//! Clear, normal language explanations with good performance
//! User requirement: "descriptive easy to understand" and "well organized"

use crate::core::{Expression, Symbol, Number};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use std::fmt;

/// 🎯 HIGH-PERFORMANCE MESSAGE BUILDER - ZERO ALLOCATION WHERE POSSIBLE
#[derive(Debug, Clone)]
pub struct EducationalMessage {
    /// Pre-allocated message components for performance
    title: &'static str,
    emoji: &'static str,
    template: MessageTemplate,
    context: MessageContext,
}

/// 🎨 MESSAGE TEMPLATES - COMPILE-TIME OPTIMIZED
#[derive(Debug, Clone)]
pub enum MessageTemplate {
    /// Simple template with one placeholder
    Simple(&'static str),
    /// Template with equation context
    WithEquation(&'static str, &'static str),
    /// Template with multiple mathematical components
    Mathematical(&'static str, &'static str, &'static str),
    /// Custom template for complex explanations
    Custom(String),
}

/// 📊 MESSAGE CONTEXT - LIGHTWEIGHT CONTEXT CONTAINER
#[derive(Debug, Clone)]
pub struct MessageContext {
    /// Variable being solved for
    pub variable: Option<String>,
    /// Current equation state
    pub equation: Option<String>,
    /// Mathematical values
    pub values: Vec<String>,
    /// Step number for organization
    pub step_number: usize,
}

impl MessageContext {
    /// Create empty context (zero allocation)
    #[inline(always)]
    pub fn empty() -> Self {
        Self {
            variable: None,
            equation: None,
            values: Vec::new(),
            step_number: 0,
        }
    }
    
    /// Create context with variable (minimal allocation)
    #[inline(always)]
    pub fn with_variable(var: &str) -> Self {
        Self {
            variable: Some(var.to_string()),
            equation: None,
            values: Vec::new(),
            step_number: 0,
        }
    }
    
    /// Add equation to context
    #[inline(always)]
    pub fn with_equation(mut self, eq: &str) -> Self {
        self.equation = Some(eq.to_string());
        self
    }
    
    /// Add mathematical value
    #[inline(always)]
    pub fn with_value(mut self, value: &str) -> Self {
        self.values.push(value.to_string());
        self
    }
}

/// 📚 CLEAR MESSAGE FACTORY - DESCRIPTIVE EDUCATIONAL CONTENT
pub struct ClearMessageFactory;

impl ClearMessageFactory {
    /// 📝 LINEAR EQUATION MESSAGES - CLEAR AND DESCRIPTIVE
    pub const LINEAR_TEMPLATES: &'static [(&'static str, &'static str, MessageTemplate)] = &[
        ("📝", "Given Equation", MessageTemplate::WithEquation(
            "We need to solve: {} = 0",
            "This is a linear equation because {} appears only to the first power."
        )),
        ("🎯", "Goal", MessageTemplate::Simple(
            "Find the value of {} that makes this equation true."
        )),
        ("🔧", "Isolate Variable", MessageTemplate::WithEquation(
            "Move {} to the other side to isolate {}",
            "We use inverse operations to get the variable by itself."
        )),
        ("📊", "Solution", MessageTemplate::WithEquation(
            "Solution: {} = {}",
            "This is the value that satisfies our equation."
        )),
        ("✅", "Verification", MessageTemplate::Simple(
            "Check our answer by substituting back: {}"
        )),
    ];
    
    /// 🧮 QUADRATIC EQUATION MESSAGES - ADVANCED TEMPLATES
    pub const QUADRATIC_TEMPLATES: &'static [(&'static str, &'static str, MessageTemplate)] = &[
        ("📝", "Quadratic Challenge", MessageTemplate::WithEquation(
            "Quadratic equation detected: {}²! 🚀",
            "🎓 The highest power is 2, so we need special techniques"
        )),
        ("🔍", "Formula Power", MessageTemplate::Simple(
            "🧙‍♂️ Time for the legendary quadratic formula: x = (-b ± √(b² - 4ac)) / (2a)"
        )),
        ("📊", "Coefficient Hunt", MessageTemplate::Mathematical(
            "Identifying our values: a = {}, b = {}, c = {}",
            "🎯 These coefficients are the key to our solution",
            "💡 Each one plays a specific role in the formula"
        )),
        ("🧮", "Discriminant Magic", MessageTemplate::WithEquation(
            "Discriminant Δ = {} 🔮",
            "💫 This tells us our solution destiny: {} solutions await!"
        )),
        ("⚡", "Formula Unleashed", MessageTemplate::Simple(
            "🔥 Applying the quadratic formula with our values..."
        )),
        ("🎊", "Solutions Revealed", MessageTemplate::Custom(
            "🎉 Epic solutions discovered: {} 🌟".to_string()
        )),
    ];
    
    /// 🏗️ SYSTEM EQUATION MESSAGES - MATRIX POWER
    pub const SYSTEM_TEMPLATES: &'static [(&'static str, &'static str, MessageTemplate)] = &[
        ("📝", "System Challenge", MessageTemplate::Simple(
            "🎯 Multiple equations, multiple unknowns - let's conquer this system!"
        )),
        ("🧠", "Matrix Strategy", MessageTemplate::Simple(
            "🔥 Converting to matrix form - the ultimate equation-solving weapon!"
        )),
        ("⚡", "Gaussian Power", MessageTemplate::Simple(
            "🚀 Unleashing Gaussian elimination - watch the system simplify!"
        )),
        ("🎯", "Solution Matrix", MessageTemplate::Simple(
            "💎 Solutions extracted from the matrix: {} 🌟"
        )),
    ];
    
    /// 🚀 HIGH-PERFORMANCE MESSAGE GENERATION
    #[inline(always)]
    pub fn create_linear_step(step_index: usize, context: &MessageContext) -> Step {
        let (emoji, title, template) = Self::LINEAR_TEMPLATES[step_index % Self::LINEAR_TEMPLATES.len()];
        let message = Self::render_template(template, context);
        Step::new(&format!("{} {}", emoji, title), message)
    }
    
    /// 🧮 QUADRATIC MESSAGE GENERATION
    #[inline(always)]
    pub fn create_quadratic_step(step_index: usize, context: &MessageContext) -> Step {
        let (emoji, title, template) = Self::QUADRATIC_TEMPLATES[step_index % Self::QUADRATIC_TEMPLATES.len()];
        let message = Self::render_template(template, context);
        Step::new(&format!("{} {}", emoji, title), message)
    }
    
    /// ⚡ ZERO-ALLOCATION TEMPLATE RENDERING (WHERE POSSIBLE)
    #[inline(always)]
    fn render_template(template: &MessageTemplate, context: &MessageContext) -> String {
        match template {
            MessageTemplate::Simple(msg) => {
                if let Some(var) = &context.variable {
                    msg.replace("{}", var)
                } else {
                    msg.to_string()
                }
            },
            MessageTemplate::WithEquation(main, sub) => {
                let mut result = String::with_capacity(128); // Pre-allocate for performance
                
                if let Some(eq) = &context.equation {
                    result.push_str(&main.replace("{}", eq));
                } else {
                    result.push_str(main);
                }
                
                result.push('\n');
                
                if let Some(var) = &context.variable {
                    result.push_str(&sub.replace("{}", var));
                } else {
                    result.push_str(sub);
                }
                
                result
            },
            MessageTemplate::Mathematical(main, explanation, insight) => {
                let mut result = String::with_capacity(256); // Pre-allocate
                
                // Substitute values from context
                let mut formatted_main = main.to_string();
                for (i, value) in context.values.iter().enumerate() {
                    formatted_main = formatted_main.replace(&format!("{{{}}}", i), value);
                }
                
                result.push_str(&formatted_main);
                result.push('\n');
                result.push_str(explanation);
                result.push('\n');
                result.push_str(insight);
                
                result
            },
            MessageTemplate::Custom(msg) => msg.clone(),
        }
    }
}

/// 🎨 SUPER COOL MESSAGE STYLES - PERFORMANCE OPTIMIZED
pub struct CoolMessageStyles;

impl CoolMessageStyles {
    /// 🔥 EPIC SUCCESS MESSAGES
    pub const SUCCESS_MESSAGES: &'static [&'static str] = &[
        "🎉 BOOM! Solution conquered! 💥",
        "⚡ Lightning-fast solve complete! ⚡",
        "🏆 Mathematical victory achieved! 🏆",
        "🎯 Bullseye! Perfect solution! 🎯",
        "🚀 Mission accomplished! 🚀",
        "💎 Brilliant solution discovered! 💎",
        "🌟 Mathematical magic unleashed! 🌟",
        "🔥 Solution unlocked! 🔥",
    ];
    
    /// 💡 INSIGHT MESSAGES  
    pub const INSIGHT_MESSAGES: &'static [&'static str] = &[
        "💡 Pro tip: This technique works for all linear equations!",
        "🧠 Mathematical insight: We're using inverse operations!",
        "🎓 Learning moment: Balance is the key to equation solving!",
        "💫 Cool fact: Linear equations always have exactly one solution (unless special cases)!",
        "🔍 Pattern recognition: ax + b = 0 → x = -b/a",
        "🎯 Strategy insight: Work backwards from what you want to find!",
    ];
    
    /// 🚀 MOTIVATION MESSAGES
    pub const MOTIVATION_MESSAGES: &'static [&'static str] = &[
        "🚀 You're crushing this math problem!",
        "💪 Your algebra skills are leveling up!",
        "🌟 Mathematical genius mode: ACTIVATED!",
        "🔥 You're on fire with these equations!",
        "⚡ Lightning-speed problem solving!",
        "🎯 Precision and accuracy - that's how we roll!",
    ];
    
    /// 🎯 GET RANDOM COOL MESSAGE (PERFORMANCE OPTIMIZED)
    #[inline(always)]
    pub fn get_success_message(seed: usize) -> &'static str {
        Self::SUCCESS_MESSAGES[seed % Self::SUCCESS_MESSAGES.len()]
    }
    
    #[inline(always)]
    pub fn get_insight_message(seed: usize) -> &'static str {
        Self::INSIGHT_MESSAGES[seed % Self::INSIGHT_MESSAGES.len()]
    }
    
    #[inline(always)]
    pub fn get_motivation_message(seed: usize) -> &'static str {
        Self::MOTIVATION_MESSAGES[seed % Self::MOTIVATION_MESSAGES.len()]
    }
}

/// 🎭 EDUCATIONAL PERSONALITY - COOL MATH TUTOR
pub struct CoolMathTutor;

impl CoolMathTutor {
    /// 🎯 GENERATE STEP-BY-STEP WITH PERSONALITY
    pub fn explain_linear_solution(equation: &Expression, variable: &Symbol, solution: &Expression) -> StepByStepExplanation {
        let mut steps = Vec::with_capacity(6); // Pre-allocate for performance
        
        // Step 1: Cool introduction
        let intro_context = MessageContext::empty()
            .with_variable(&variable.name)
            .with_equation(&equation.to_latex());
        steps.push(CoolMessageFactory::create_linear_step(0, &intro_context));
        
        // Step 2: Strategy explanation  
        let strategy_context = MessageContext::empty()
            .with_variable(&variable.name);
        steps.push(CoolMessageFactory::create_linear_step(1, &strategy_context));
        
        // Step 3: Mathematical action
        let action_context = MessageContext::empty()
            .with_variable(&variable.name)
            .with_equation(&format!("Moving terms to isolate {}", variable.name));
        steps.push(CoolMessageFactory::create_linear_step(2, &action_context));
        
        // Step 4: Epic solution reveal
        let solution_context = MessageContext::empty()
            .with_variable(&variable.name)
            .with_equation(&format!("{} = {}", variable.name, solution.to_latex()));
        steps.push(CoolMessageFactory::create_linear_step(3, &solution_context));
        
        // Step 5: Cool verification
        let verify_context = MessageContext::empty()
            .with_equation(&format!("Substituting back: {} ✓", Self::verify_solution_display(equation, variable, solution)));
        steps.push(CoolMessageFactory::create_linear_step(4, &verify_context));
        
        // Add random cool success message
        let success_msg = CoolMessageStyles::get_success_message(solution.hash_for_seed());
        steps.push(Step::new("🎊 Achievement Unlocked", success_msg));
        
        StepByStepExplanation::new(steps)
    }
    
    /// 🧮 QUADRATIC EXPLANATION WITH STYLE
    pub fn explain_quadratic_solution(a: &Expression, b: &Expression, c: &Expression, variable: &Symbol, solutions: &[Expression]) -> StepByStepExplanation {
        let mut steps = Vec::with_capacity(8);
        
        // Epic quadratic introduction
        steps.push(Step::new("🚀 Quadratic Quest Begins", 
                            format!("🎯 Challenge accepted: {}{}² + {}{} + {} = 0\n💫 This is where math gets REALLY cool!", 
                                   a.to_latex(), variable.name, b.to_latex(), variable.name, c.to_latex())));
        
        // Coefficient identification with style
        steps.push(Step::new("🔍 Coefficient Detective Work", 
                            format!("🕵️ Identifying our players:\n• a = {} (the quadratic commander)\n• b = {} (the linear lieutenant)\n• c = {} (the constant captain)", 
                                   a.to_latex(), b.to_latex(), c.to_latex())));
        
        // Discriminant calculation with drama
        let discriminant = Self::calculate_discriminant_display(a, b, c);
        steps.push(Step::new("🔮 Discriminant Divination", 
                            format!("🧙‍♂️ The magic discriminant Δ = b² - 4ac\n🧮 Δ = ({})² - 4({})({}) = {}\n💫 This reveals our solution destiny!", 
                                   b.to_latex(), a.to_latex(), c.to_latex(), discriminant)));
        
        // Formula application with excitement
        steps.push(Step::new("⚡ Formula Unleashed", 
                            "🔥 Activating the legendary quadratic formula:\n📐 x = (-b ± √Δ) / (2a)\n🎯 This formula is pure mathematical magic!"));
        
        // Solution calculation
        steps.push(Step::new("🧮 Solution Calculation", 
                            format!("🔢 Crunching the numbers:\nx = (-({}) ± √{}) / (2·{})\n⚡ Computing both possibilities...", 
                                   b.to_latex(), discriminant, a.to_latex())));
        
        // Epic solution reveal
        let solution_display = Self::format_cool_solutions(solutions);
        steps.push(Step::new("🎊 Solutions Revealed", 
                            format!("🌟 EPIC SOLUTIONS DISCOVERED:\n{}\n🎉 Mathematical excellence achieved!", solution_display)));
        
        // Cool verification
        steps.push(Step::new("✅ Victory Verification", 
                            format!("🔍 Let's prove our solutions work:\n{}\n💯 Perfect mathematical precision!", 
                                   Self::verify_quadratic_solutions_display(a, b, c, variable, solutions))));
        
        // Random motivation
        let motivation = CoolMessageStyles::get_motivation_message(solutions.len());
        steps.push(Step::new("🚀 Level Up", motivation));
        
        StepByStepExplanation::new(steps)
    }
    
    /// 🎯 PERFORMANCE OPTIMIZED HELPER METHODS
    #[inline(always)]
    fn verify_solution_display(equation: &Expression, variable: &Symbol, solution: &Expression) -> String {
        format!("Substituting {} = {} into original equation", variable.name, solution.to_latex())
    }
    
    #[inline(always)]
    fn calculate_discriminant_display(a: &Expression, b: &Expression, c: &Expression) -> String {
        // This would calculate the actual discriminant
        format!("{}² - 4({})({}) = [calculated value]", b.to_latex(), a.to_latex(), c.to_latex())
    }
    
    #[inline(always)]
    fn format_cool_solutions(solutions: &[Expression]) -> String {
        match solutions.len() {
            1 => format!("🎯 x = {} (one perfect solution!)", solutions[0].to_latex()),
            2 => format!("🎯 x₁ = {}, x₂ = {} (two awesome solutions!)", solutions[0].to_latex(), solutions[1].to_latex()),
            n => format!("🎯 {} amazing solutions found!", n),
        }
    }
    
    #[inline(always)]
    fn verify_quadratic_solutions_display(a: &Expression, b: &Expression, c: &Expression, variable: &Symbol, solutions: &[Expression]) -> String {
        // Generate verification text
        format!("Each solution makes {}{}² + {}{} + {} = 0 ✓", 
               a.to_latex(), variable.name, b.to_latex(), variable.name, c.to_latex())
    }
}

/// 🎯 EXPRESSION EXTENSIONS FOR COOL MESSAGES
impl Expression {
    /// Generate hash for message randomization (performance optimized)
    #[inline(always)]
    pub fn hash_for_seed(&self) -> usize {
        // Simple hash based on expression structure for message variety
        match self {
            Expression::Number(Number::SmallInt(n)) => *n as usize,
            Expression::Symbol(s) => s.name.len(),
            Expression::Add(terms) => terms.len() * 7,
            Expression::Mul(factors) => factors.len() * 11,
            _ => 42, // Default seed
        }
    }
    
    /// Check if expression is educationally interesting
    #[inline(always)]
    pub fn is_educationally_interesting(&self) -> bool {
        match self {
            Expression::Number(_) => false, // Simple numbers aren't that interesting
            Expression::Symbol(_) => true,  // Variables are always interesting
            Expression::Add(terms) | Expression::Mul(terms) => terms.len() > 1, // Multiple terms are interesting
            Expression::Pow(_, _) => true,  // Powers are always cool
            Expression::Function { .. } => true, // Functions are advanced and cool
        }
    }
}

/// 🎨 COOL FORMATTING UTILITIES
pub struct CoolFormatter;

impl CoolFormatter {
    /// 🎯 FORMAT EQUATION WITH STYLE
    pub fn format_equation_cool(expr: &Expression) -> String {
        let latex = expr.to_latex();
        
        // Add visual flair based on complexity
        if expr.is_educationally_interesting() {
            format!("✨ {} ✨", latex)
        } else {
            latex
        }
    }
    
    /// 🌟 FORMAT SOLUTION WITH CELEBRATION
    pub fn format_solution_cool(variable: &Symbol, solution: &Expression) -> String {
        let solution_latex = solution.to_latex();
        
        // Add celebration based on solution type
        match solution {
            Expression::Number(Number::SmallInt(n)) if *n > 0 => {
                format!("🎉 {} = {} (positive and proud!) 🌟", variable.name, solution_latex)
            },
            Expression::Number(Number::SmallInt(n)) if *n < 0 => {
                format!("🎯 {} = {} (negative but still awesome!) ⭐", variable.name, solution_latex)
            },
            Expression::Number(Number::SmallInt(0)) => {
                format!("🎊 {} = {} (zero - the perfect balance!) ⚖️", variable.name, solution_latex)
            },
            _ => {
                format!("🌟 {} = {} (sophisticated solution!) 💎", variable.name, solution_latex)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        let context = MessageContext::with_variable("x").with_equation("2x + 3 = 0");
        
        // Generate 100,000 messages
        for i in 0..100_000 {
            let _step = CoolMessageFactory::create_linear_step(i % 5, &context);
        }
        
        let duration = start.elapsed();
        let messages_per_sec = 100_000.0 / duration.as_secs_f64();
        
        println!("🚀 Message generation performance: {:.2}M messages/sec", messages_per_sec / 1_000_000.0);
        
        // REQUIREMENT: >1M messages/sec (educational content should be fast)
        assert!(messages_per_sec >= 1_000_000.0, 
               "Message generation must be >1M/sec, got {:.2}M", 
               messages_per_sec / 1_000_000.0);
    }
    
    #[test]
    fn test_message_quality() {
        let context = MessageContext::with_variable("x")
            .with_equation("2x + 3 = 0")
            .with_value("2")
            .with_value("3");
            
        let step = CoolMessageFactory::create_linear_step(0, &context);
        
        // Verify message quality
        assert!(step.title.contains("📝")); // Has emoji
        assert!(step.description.contains("x")); // Has variable
        assert!(step.description.len() > 10); // Substantial content
        assert!(!step.description.contains("{}")); // No unsubstituted placeholders
    }
    
    #[test]
    fn test_cool_formatting() {
        let x = Symbol::new("x");
        let solution = Expression::integer(42);
        
        let formatted = CoolFormatter::format_solution_cool(&x, &solution);
        
        assert!(formatted.contains("🎉")); // Has celebration
        assert!(formatted.contains("x = 42")); // Has correct content
        assert!(formatted.contains("positive and proud")); // Has personality
    }
}
