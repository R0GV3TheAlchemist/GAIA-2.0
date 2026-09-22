//! #622 — AI Skills Database: procedural and tool-use capability registry.
//!
//! The goal is not to claim execution-grounded measurements for GAIA itself.
//! Instead, this module provides a typed registry of skills, published benchmark
//! references, tool-component mappings, gap analysis, and a small deterministic
//! search/assessment surface for higher layers.

/// High-level category of a skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillCategory {
    ToolUse,
    AgentPlanning,
    ComputerUse,
    MultiAgent,
    Robotic,
    Code,
    Reasoning,
    Multimodal,
}

/// A single registered AI skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub skill_id: &'static str,
    pub domain: &'static str,
    pub category: SkillCategory,
    pub description: &'static str,
    pub benchmark: &'static str,
    pub performance_score: u8,
    pub model: &'static str,
    pub tool_implementation: &'static [&'static str],
}

/// Programmatic assessment result for a specific skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillAssessment {
    pub skill_id: &'static str,
    pub score: u8,
    pub level: &'static str,
    pub benchmark: &'static str,
    pub model: &'static str,
}

/// Gap analysis for a goal against the known skill registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillGap {
    pub goal: String,
    pub present: Vec<&'static Skill>,
    pub missing: Vec<&'static Skill>,
}

/// Learning pathway for acquiring a skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillLearningPath {
    pub skill_id: &'static str,
    pub prerequisites: &'static [&'static str],
    pub benchmarks_to_watch: &'static [&'static str],
    pub tool_components: &'static [&'static str],
    pub next_step: &'static str,
}

pub static SKILLS: [Skill; 18] = [
    Skill {
        skill_id: "web-search",
        domain: "agency",
        category: SkillCategory::ToolUse,
        description: "Issue web queries, inspect results, and ground answers in retrieved pages.",
        benchmark: "BrowseComp",
        performance_score: 84,
        model: "GPT-4o",
        tool_implementation: &["search_web", "fetch_url"],
    },
    Skill {
        skill_id: "code-execution",
        domain: "code",
        category: SkillCategory::ToolUse,
        description: "Write, run, and inspect code to solve computational tasks.",
        benchmark: "HumanEval",
        performance_score: 96,
        model: "GPT-4o",
        tool_implementation: &["python-exec", "wasm-runner", "sandbox"],
    },
    Skill {
        skill_id: "file-operations",
        domain: "agency",
        category: SkillCategory::ToolUse,
        description: "Read, write, search, and transform files within a bounded workspace.",
        benchmark: "OSWorld",
        performance_score: 38,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["file_explore", "search_files_v2", "fs-wasm"],
    },
    Skill {
        skill_id: "api-calling",
        domain: "agency",
        category: SkillCategory::ToolUse,
        description: "Invoke external APIs and normalize returned structured data.",
        benchmark: "tau2-bench",
        performance_score: 62,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["http-wasm", "json-wasm"],
    },
    Skill {
        skill_id: "task-decomposition",
        domain: "reasoning",
        category: SkillCategory::AgentPlanning,
        description: "Break large goals into ordered actionable sub-problems.",
        benchmark: "BIG-Bench Hard",
        performance_score: 89,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["planner-core", "goal-stack"],
    },
    Skill {
        skill_id: "tool-selection",
        domain: "agency",
        category: SkillCategory::AgentPlanning,
        description: "Select the right tool for a subtask under uncertainty and cost constraints.",
        benchmark: "BrowseComp",
        performance_score: 84,
        model: "GPT-4o",
        tool_implementation: &["router", "tool-catalog"],
    },
    Skill {
        skill_id: "error-recovery",
        domain: "agency",
        category: SkillCategory::AgentPlanning,
        description: "Detect execution failure and re-plan with a fallback path.",
        benchmark: "OSWorld",
        performance_score: 38,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["retry-loop", "planner-core", "trace-store"],
    },
    Skill {
        skill_id: "goal-tracking",
        domain: "agency",
        category: SkillCategory::AgentPlanning,
        description: "Maintain state over long multi-step tasks and verify completion.",
        benchmark: "tau2-bench",
        performance_score: 62,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["goal-stack", "trace-store"],
    },
    Skill {
        skill_id: "gui-navigation",
        domain: "vision",
        category: SkillCategory::ComputerUse,
        description: "Navigate graphical interfaces, menus, and applications from screenshots or live state.",
        benchmark: "OSWorld",
        performance_score: 38,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["computer-use-wasm", "vision-parser"],
    },
    Skill {
        skill_id: "form-filling",
        domain: "vision",
        category: SkillCategory::ComputerUse,
        description: "Read forms, infer required fields, and populate them correctly.",
        benchmark: "WebArena",
        performance_score: 35,
        model: "GPT-4o",
        tool_implementation: &["computer-use-wasm", "dom-agent"],
    },
    Skill {
        skill_id: "agent-delegation",
        domain: "agency",
        category: SkillCategory::MultiAgent,
        description: "Assign sub-problems to specialist agents and reconcile results.",
        benchmark: "tau2-bench",
        performance_score: 62,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["multi-agent-bus", "task-router"],
    },
    Skill {
        skill_id: "parallel-execution",
        domain: "agency",
        category: SkillCategory::MultiAgent,
        description: "Run independent subtasks concurrently and merge artifacts.",
        benchmark: "tau2-bench",
        performance_score: 62,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["multi-agent-bus", "task-router", "joiner"],
    },
    Skill {
        skill_id: "algorithm-design",
        domain: "code",
        category: SkillCategory::Code,
        description: "Design correct and efficient algorithms from problem statements.",
        benchmark: "HumanEval",
        performance_score: 96,
        model: "GPT-4o",
        tool_implementation: &["python-exec", "sandbox"],
    },
    Skill {
        skill_id: "debugging",
        domain: "code",
        category: SkillCategory::Code,
        description: "Localize, explain, and repair software defects in real repositories.",
        benchmark: "SWE-bench-Verified",
        performance_score: 95,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["git-wasm", "sandbox", "trace-store"],
    },
    Skill {
        skill_id: "deductive-reasoning",
        domain: "reasoning",
        category: SkillCategory::Reasoning,
        description: "Derive valid conclusions from explicit premises.",
        benchmark: "LogiQA",
        performance_score: 88,
        model: "GPT-4o",
        tool_implementation: &["reasoner-core"],
    },
    Skill {
        skill_id: "causal-reasoning",
        domain: "reasoning",
        category: SkillCategory::Reasoning,
        description: "Model interventions, causes, and counterfactual outcomes.",
        benchmark: "ARC-AGI-2",
        performance_score: 56,
        model: "GPT-4o",
        tool_implementation: &["reasoner-core", "world-model"],
    },
    Skill {
        skill_id: "image-understanding",
        domain: "vision",
        category: SkillCategory::Multimodal,
        description: "Extract entities, relationships, and structured answers from images.",
        benchmark: "MMMU",
        performance_score: 80,
        model: "Claude 3.5 Sonnet",
        tool_implementation: &["vision-parser", "ocr-wasm"],
    },
    Skill {
        skill_id: "document-parsing",
        domain: "vision",
        category: SkillCategory::Multimodal,
        description: "Read dense documents, tables, and charts into structured semantic form.",
        benchmark: "DocVQA",
        performance_score: 92,
        model: "GPT-4o",
        tool_implementation: &["ocr-wasm", "table-extractor", "vision-parser"],
    },
];

pub fn all_skills() -> &'static [Skill; 18] {
    &SKILLS
}

pub fn skill_by_id(skill_id: &str) -> Option<&'static Skill> {
    SKILLS.iter().find(|s| s.skill_id == skill_id)
}

pub fn skills_for_domain(domain: &str) -> Vec<&'static Skill> {
    SKILLS.iter().filter(|s| s.domain == domain).collect()
}

pub fn skills_for_tool(tool_component: &str) -> Vec<&'static Skill> {
    SKILLS
        .iter()
        .filter(|s| s.tool_implementation.contains(&tool_component))
        .collect()
}

fn query_tokens(query: &str) -> Vec<String> {
    query.split(|c: char| !c.is_alphanumeric() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .collect()
}

fn score_skill_match(skill: &Skill, tokens: &[String]) -> usize {
    let mut score = 0;
    let haystacks = [
        skill.skill_id,
        skill.domain,
        skill.description,
        skill.benchmark,
        skill.model,
    ];
    for token in tokens {
        if haystacks.iter().any(|h| h.to_ascii_lowercase().contains(token)) {
            score += 2;
        }
        if skill
            .tool_implementation
            .iter()
            .any(|tool| tool.to_ascii_lowercase().contains(token))
        {
            score += 3;
        }
    }
    score + usize::from(skill.performance_score / 25)
}

/// Semantic-ish search returning the top-3 relevant skills.
pub fn search_skills(query: &str) -> Vec<&'static Skill> {
    let tokens = query_tokens(query);
    let mut ranked: Vec<(&Skill, usize)> = SKILLS
        .iter()
        .map(|s| (s, score_skill_match(s, &tokens)))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.performance_score.cmp(&a.0.performance_score)));
    ranked
        .into_iter()
        .filter(|(_, score)| *score > 0)
        .take(3)
        .map(|(skill, _)| skill)
        .collect()
}

/// Programmatic assessment API: test a skill, get a score and level.
pub fn assess_skill(skill_id: &str) -> Option<SkillAssessment> {
    let skill = skill_by_id(skill_id)?;
    let level = match skill.performance_score {
        90..=100 => "expert",
        75..=89 => "strong",
        50..=74 => "developing",
        _ => "weak",
    };
    Some(SkillAssessment {
        skill_id: skill.skill_id,
        score: skill.performance_score,
        level,
        benchmark: skill.benchmark,
        model: skill.model,
    })
}

/// Gap analysis for a natural-language goal.
pub fn skill_gap_analysis(goal: &str, current_skill_ids: &[&str]) -> SkillGap {
    let desired = search_skills(goal);
    let mut present = Vec::new();
    let mut missing = Vec::new();
    for skill in desired {
        if current_skill_ids.contains(&skill.skill_id) {
            present.push(skill);
        } else {
            missing.push(skill);
        }
    }
    SkillGap {
        goal: goal.to_string(),
        present,
        missing,
    }
}

/// A deterministic learning pathway generator.
pub fn learning_path(skill_id: &str) -> Option<SkillLearningPath> {
    match skill_id {
        "web-search" => Some(SkillLearningPath {
            skill_id,
            prerequisites: &["tool-selection", "goal-tracking"],
            benchmarks_to_watch: &["BrowseComp", "WebArena"],
            tool_components: &["search_web", "fetch_url"],
            next_step: "Practice retrieval grounding and page verification loops.",
        }),
        "debugging" => Some(SkillLearningPath {
            skill_id,
            prerequisites: &["algorithm-design", "code-execution"],
            benchmarks_to_watch: &["SWE-bench-Verified", "LiveCodeBench"],
            tool_components: &["git-wasm", "sandbox", "trace-store"],
            next_step: "Train on issue reproduction, patch generation, and regression checks.",
        }),
        "gui-navigation" => Some(SkillLearningPath {
            skill_id,
            prerequisites: &["image-understanding", "goal-tracking"],
            benchmarks_to_watch: &["OSWorld", "WebArena"],
            tool_components: &["computer-use-wasm", "vision-parser"],
            next_step: "Improve screenshot grounding, click planning, and recovery from UI drift.",
        }),
        _ => {
            let skill = skill_by_id(skill_id)?;
            Some(SkillLearningPath {
                skill_id,
                prerequisites: &[],
                benchmarks_to_watch: &[skill.benchmark],
                tool_components: skill.tool_implementation,
                next_step: "Improve benchmark performance, then widen tool coverage.",
            })
        }
    }
}
