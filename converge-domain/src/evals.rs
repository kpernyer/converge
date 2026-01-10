// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: LicenseRef-Proprietary
// All rights reserved. This source code is proprietary and confidential.
// Unauthorized copying, modification, or distribution is strictly prohibited.

//! Domain-level evals for Converge.
//!
//! These evals define what "good" means for domain-specific outcomes.
//! They are:
//! - Outcome-based, not path-based
//! - Reusable across models, agents, and time
//! - Stored as traceable artifacts
//! - Usable in invariant checks
//!
//! # Philosophy
//!
//! In Converge, evals are not tests of behavior — they are formal definitions
//! of acceptable outcomes. This aligns with the principle that:
//!
//! - Evals test whether a convergence outcome satisfies intent-level properties
//! - Evals are business semantics, not implementation details
//! - Evals are the competitive moat (most systems can't define them clearly)

use converge_core::{Context, ContextKey, Eval, EvalOutcome, EvalResult};

/// Eval: Strategy diversity
///
/// Ensures at least 3 distinct strategies exist with no two targeting
/// the same primary channel.
///
/// This is a domain-level eval that defines what "good" means for
/// growth strategy convergence.
pub struct StrategyDiversityEval;

impl Eval for StrategyDiversityEval {
    fn name(&self) -> &str {
        "strategy_diversity"
    }

    fn description(&self) -> &str {
        "Ensures at least 3 distinct strategies exist with no two targeting the same primary channel"
    }

    fn dependencies(&self) -> &[ContextKey] {
        &[ContextKey::Strategies]
    }

    fn evaluate(&self, ctx: &Context) -> EvalResult {
        let strategies = ctx.get(ContextKey::Strategies);

        if strategies.len() < 3 {
            return EvalResult::with_facts(
                self.name(),
                EvalOutcome::Fail,
                strategies.len() as f64 / 3.0,
                format!("Only {} strategies found, need at least 3", strategies.len()),
                strategies.iter().map(|s| s.id.clone()).collect(),
            );
        }

        // Check for channel diversity (simplified: check if content mentions different channels)
        let channels: Vec<&str> = strategies
            .iter()
            .filter_map(|s| {
                // Simplified: extract channel from content
                // In production, this would parse structured data
                if s.content.contains("email") {
                    Some("email")
                } else if s.content.contains("social") {
                    Some("social")
                } else if s.content.contains("content") {
                    Some("content")
                } else if s.content.contains("paid") {
                    Some("paid")
                } else {
                    None
                }
            })
            .collect();

        let unique_channels: std::collections::HashSet<&str> = channels.iter().copied().collect();

        if unique_channels.len() < 3 {
            EvalResult::with_facts(
                self.name(),
                EvalOutcome::Fail,
                unique_channels.len() as f64 / 3.0,
                format!(
                    "Only {} unique channels found across {} strategies, need at least 3",
                    unique_channels.len(),
                    strategies.len()
                ),
                strategies.iter().map(|s| s.id.clone()).collect(),
            )
        } else {
            EvalResult::with_facts(
                self.name(),
                EvalOutcome::Pass,
                1.0,
                format!(
                    "Found {} distinct strategies across {} unique channels",
                    strategies.len(),
                    unique_channels.len()
                ),
                strategies.iter().map(|s| s.id.clone()).collect(),
            )
        }
    }
}

/// Eval: Lead qualification quality
///
/// Ensures at least 80% of leads have:
/// - A clear ICP match
/// - A justification rationale
/// - A recommended next action
///
/// This is a domain-level eval for SDR funnel outcomes.
pub struct LeadQualificationQualityEval;

impl Eval for LeadQualificationQualityEval {
    fn name(&self) -> &str {
        "lead_qualification_quality"
    }

    fn description(&self) -> &str {
        "Ensures at least 80% of leads have ICP match, rationale, and next action"
    }

    fn dependencies(&self) -> &[ContextKey] {
        &[ContextKey::Strategies, ContextKey::Evaluations]
    }

    fn evaluate(&self, ctx: &Context) -> EvalResult {
        let strategies = ctx.get(ContextKey::Strategies);

        if strategies.is_empty() {
            return EvalResult::new(
                self.name(),
                EvalOutcome::Indeterminate,
                0.0,
                "No leads found to evaluate",
            );
        }

        let mut qualified_count = 0;
        let fact_ids: Vec<String> = strategies.iter().map(|s| s.id.clone()).collect();

        for strategy in strategies {
            // Simplified: check if content contains required elements
            // In production, this would parse structured data
            let has_icp = strategy.content.contains("ICP") || strategy.content.contains("fit");
            let has_rationale = strategy.content.contains("because") || strategy.content.contains("rationale");
            let has_action = strategy.content.contains("next") || strategy.content.contains("action");

            if has_icp && has_rationale && has_action {
                qualified_count += 1;
            }
        }

        let quality_ratio = qualified_count as f64 / strategies.len() as f64;
        let threshold = 0.8;

        if quality_ratio >= threshold {
            EvalResult::with_facts(
                self.name(),
                EvalOutcome::Pass,
                quality_ratio,
                format!(
                    "{}/{} leads ({:.1}%) meet quality criteria",
                    qualified_count,
                    strategies.len(),
                    quality_ratio * 100.0
                ),
                fact_ids,
            )
        } else {
            EvalResult::with_facts(
                self.name(),
                EvalOutcome::Fail,
                quality_ratio,
                format!(
                    "Only {}/{} leads ({:.1}%) meet quality criteria, need {:.0}%",
                    qualified_count,
                    strategies.len(),
                    quality_ratio * 100.0,
                    threshold * 100.0
                ),
                fact_ids,
            )
        }
    }
}

/// Eval: Meeting schedule feasibility
///
/// Ensures all scheduled meetings respect working hours (10-16 or 9-17).
///
/// This is a domain-level eval for meeting scheduler outcomes.
pub struct MeetingScheduleFeasibilityEval;

impl Eval for MeetingScheduleFeasibilityEval {
    fn name(&self) -> &str {
        "meeting_schedule_feasibility"
    }

    fn description(&self) -> &str {
        "Ensures all scheduled meetings respect working hours"
    }

    fn dependencies(&self) -> &[ContextKey] {
        &[ContextKey::Strategies, ContextKey::Constraints]
    }

    fn evaluate(&self, ctx: &Context) -> EvalResult {
        let strategies = ctx.get(ContextKey::Strategies);
        let constraints = ctx.get(ContextKey::Constraints);

        if strategies.is_empty() {
            return EvalResult::new(
                self.name(),
                EvalOutcome::Indeterminate,
                0.0,
                "No meetings found to evaluate",
            );
        }

        // Extract working hours from constraints
        let working_hours = constraints
            .iter()
            .find(|c| c.content.contains("10-16") || c.content.contains("9-17"))
            .is_some();

        if !working_hours {
            return EvalResult::new(
                self.name(),
                EvalOutcome::Indeterminate,
                0.0,
                "Working hours constraint not found",
            );
        }

        let mut valid_count = 0;
        let fact_ids: Vec<String> = strategies.iter().map(|s| s.id.clone()).collect();

        for strategy in strategies {
            // Simplified: check if content mentions valid time slots
            // In production, this would parse structured time data
            let is_valid = strategy.content.contains("10-16") || strategy.content.contains("9-17");

            if is_valid {
                valid_count += 1;
            }
        }

        let validity_ratio = valid_count as f64 / strategies.len() as f64;

        if validity_ratio == 1.0 {
            EvalResult::with_facts(
                self.name(),
                EvalOutcome::Pass,
                1.0,
                format!("All {} meetings respect working hours", strategies.len()),
                fact_ids,
            )
        } else {
            EvalResult::with_facts(
                self.name(),
                EvalOutcome::Fail,
                validity_ratio,
                format!(
                    "Only {}/{} meetings respect working hours",
                    valid_count,
                    strategies.len()
                ),
                fact_ids,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use converge_core::{Context, Fact};

    #[test]
    fn strategy_diversity_passes_with_three_strategies() {
        let eval = StrategyDiversityEval;
        let mut ctx = Context::new();

        // Add 3 strategies with different channels
        ctx.add_fact(Fact::new(
            ContextKey::Strategies,
            "strat-1",
            "email marketing campaign",
        ))
        .unwrap();
        ctx.add_fact(Fact::new(
            ContextKey::Strategies,
            "strat-2",
            "social media outreach",
        ))
        .unwrap();
        ctx.add_fact(Fact::new(
            ContextKey::Strategies,
            "strat-3",
            "content marketing strategy",
        ))
        .unwrap();

        let result = eval.evaluate(&ctx);
        assert_eq!(result.outcome, EvalOutcome::Pass);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn strategy_diversity_fails_with_insufficient_strategies() {
        let eval = StrategyDiversityEval;
        let mut ctx = Context::new();

        ctx.add_fact(Fact::new(
            ContextKey::Strategies,
            "strat-1",
            "email campaign",
        ))
        .unwrap();

        let result = eval.evaluate(&ctx);
        assert_eq!(result.outcome, EvalOutcome::Fail);
        assert!(result.score < 1.0);
    }

    #[test]
    fn lead_qualification_quality_passes_with_high_quality() {
        let eval = LeadQualificationQualityEval;
        let mut ctx = Context::new();

        // Add high-quality leads (simplified content)
        for i in 0..10 {
            ctx.add_fact(Fact::new(
                ContextKey::Strategies,
                format!("lead-{}", i),
                format!("ICP fit: yes, because: qualified, next action: call"),
            ))
            .unwrap();
        }

        let result = eval.evaluate(&ctx);
        assert_eq!(result.outcome, EvalOutcome::Pass);
        assert!(result.score >= 0.8);
    }
}
