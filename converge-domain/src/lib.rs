// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Domain-specific agents and examples for Converge.
//!
//! This crate contains applied use cases that demonstrate
//! Converge's capabilities in real domains.
//!
//! # Available Domains
//!
//! - [`growth_strategy`]: Growth strategy pipeline for market analysis
//! - [`meeting_scheduler`]: Meeting scheduling with constraint satisfaction
//! - [`resource_routing`]: Resource allocation and routing optimization
//! - [`release_readiness`]: Engineering dependency and release quality gates
//! - [`supply_chain`]: Supply chain re-planning with parallel optimization tracks
//! - [`inventory_rebalancing`]: Multi-region inventory rebalancing with forecasting
//! - [`strategic_sourcing`]: Strategic sourcing and vendor selection
//! - [`catalog_enrichment`]: Catalog update and enrichment from multiple feeds
//! - [`crm_account_health`]: CRM account health and growth strategy
//! - [`compliance_monitoring`]: Continuous compliance monitoring

pub mod catalog_enrichment;
pub mod compliance_monitoring;
pub mod crm_account_health;
pub mod growth_strategy;
pub mod inventory_rebalancing;
pub mod meeting_scheduler;
pub mod release_readiness;
pub mod resource_routing;
pub mod strategic_sourcing;
pub mod supply_chain;

pub mod llm_utils;

// LLM-enabled versions of use cases
pub mod catalog_enrichment_llm;
pub mod compliance_monitoring_llm;
pub mod crm_account_health_llm;
pub mod growth_strategy_llm;
pub mod inventory_rebalancing_llm;
pub mod meeting_scheduler_llm;
pub mod strategic_sourcing_llm;

#[cfg(test)]
mod stress_tests;

pub use growth_strategy::{
    // Invariants
    BrandSafetyInvariant,
    // Agents
    CompetitorAgent,
    EvaluationAgent,
    MarketSignalAgent,
    RequireEvaluationRationale,
    RequireMultipleStrategies,
    RequireStrategyEvaluations,
    StrategyAgent,
};

pub use meeting_scheduler::{
    // Agents
    AvailabilityRetrievalAgent,
    ConflictDetectionAgent,
    // Invariants
    RequireParticipantAvailability,
    RequirePositiveDuration,
    RequireValidSlot,
    SlotOptimizationAgent,
    TimeZoneNormalizationAgent,
    WorkingHoursConstraintAgent,
};

pub use resource_routing::{
    // Agents
    ConstraintValidationAgent,
    FeasibilityAgent,
    // Invariants
    RequireAllTasksAssigned,
    RequireCapacityRespected,
    RequireValidDefinitions,
    ResourceRetrievalAgent,
    SolverAgent,
    TaskRetrievalAgent,
};

pub use release_readiness::{
    // Agents
    DependencyGraphAgent,
    DocumentationAgent,
    PerformanceRegressionAgent,
    ReleaseReadyAgent,
    // Invariants
    RequireAllChecksComplete,
    RequireMinimumCoverage,
    RequireNoCriticalVulnerabilities,
    RiskSummaryAgent,
    SecurityScanAgent,
    TestCoverageAgent,
};

pub use supply_chain::{
    // Agents
    ConsolidationAgent,
    CostEstimationAgent,
    DemandSnapshotAgent,
    InventoryStateAgent,
    // Invariants
    RequireCompleteAssessments,
    RequireFeasiblePlan,
    RequireSLACompliance,
    RiskAssessmentAgent,
    RouteGenerationAgent,
    SLAValidationAgent,
    SupplierStatusAgent,
};

pub use inventory_rebalancing::{
    // Agents
    CapacityConstraintAgent,
    FinancialImpactAgent,
    ForecastAgent,
    InventoryAgent,
    RebalanceDecisionAgent,
    // Invariants
    RequireBudgetCompliance,
    RequireCompleteForecasts,
    RequireSafetyStock,
    SalesVelocityAgent,
    TransferOptimizationAgent,
};

pub use strategic_sourcing::{
    // Agents
    ComplianceAgent,
    ESGScoringAgent,
    PriceBenchmarkAgent,
    // Invariants
    RequireCompleteAssessments as RequireSourcingAssessments,
    RequireCompliantVendor,
    RequireShortlistCompliance,
    RiskModelAgent,
    SourcingStrategyAgent,
    SupplierDiscoveryAgent,
    VendorRankingAgent,
};

pub use catalog_enrichment::{
    // Agents
    AttributeNormalizationAgent,
    CategoryInferenceAgent,
    DeduplicationAgent,
    FeedIngestionAgent,
    PricingValidationAgent,
    ProductReadyAgent,
    // Invariants
    RequireNoDuplicates,
    RequireRequiredAttributes,
    RequireValidPrices,
    SchemaInvariantAgent,
};

pub use crm_account_health::{
    // Agents
    ActionPrioritizationAgent,
    ChurnRiskAgent,
    // Invariants
    RequireChurnActionPlan,
    RequireCompleteAnalysis,
    RevenueTrendAgent,
    SupportTicketAgent,
    UpsellOpportunityAgent,
    UsageSignalAgent,
};

pub use compliance_monitoring::{
    // Agents
    EvidenceCollectorAgent,
    PolicyRuleAgent,
    RegulationParserAgent,
    RemediationProposalAgent,
    // Invariants
    RequireEvidenceForAllRegulations,
    RequireRemediationPlans,
    ViolationDetectorAgent,
};
