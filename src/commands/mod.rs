//! Commands module for TITAN-CLI
//! 
//! This module exports all available CLI commands:
//! - status: Check service health
//! - ask: Query VORTEX AI
//! - deploy: Deploy services
//! - vision: Connect to OPTICUS

pub mod ask;
pub mod deploy;
pub mod status;
pub mod vision;
