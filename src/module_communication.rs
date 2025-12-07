// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Module Communication System
//! 
//! This module provides inter-module communication between Grease modules,
//! enabling UI modules to work together seamlessly.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use crate::module_errors::ModuleError;

/// Message types for inter-module communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleMessage {
    /// UI event message
    UIEvent(UIEventMessage),

    /// Data synchronization message
    DataSync(DataSyncMessage),
    /// Status update message
    StatusUpdate(StatusUpdateMessage),
    /// Error notification message
    ErrorNotification(ErrorMessage),
}

/// UI event messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIEventMessage {
    pub event_type: UIEventType,
    pub widget_id: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

/// UI event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIEventType {
    ButtonClick,
    TextInput,
    WindowClose,
    WindowResize,
    FocusChange,
    KeyPress,
    MouseMove,
}



/// Data synchronization messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSyncMessage {
    pub sync_type: DataSyncType,
    pub data: serde_json::Value,
    pub target_module: String,
    pub source_module: String,
}

/// Data synchronization types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSyncType {
    VariableUpdate,
    StateSync,
    ConfigurationChange,
    PerformanceMetrics,
}

/// Status update messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdateMessage {
    pub status_type: StatusType,
    pub module_name: String,
    pub status: String,
    pub details: Option<serde_json::Value>,
}

/// Status types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusType {
    Initialization,
    Ready,
    Busy,
    Error,
    Shutdown,
}

/// Error notification messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error_type: ErrorType,
    pub module_name: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    RuntimeError,
    CompilationError,
    CommunicationError,
    ResourceError,
    ConfigurationError,
}

/// Module communication channel
pub struct ModuleChannel {
    pub name: String,
    pub sender: Arc<Mutex<Vec<ModuleMessage>>>,
    pub receivers: Arc<Mutex<Vec<Box<dyn MessageReceiver>>>>,
}

/// Trait for message receivers
pub trait MessageReceiver: Send + Sync {
    fn receive(&self, message: &ModuleMessage) -> Result<(), ModuleError>;
    fn can_handle(&self, message_type: &str) -> bool;
}

/// Module communication manager
pub struct ModuleCommunication {
    channels: Arc<Mutex<HashMap<String, ModuleChannel>>>,
    message_handlers: Arc<Mutex<HashMap<String, Box<dyn MessageHandler>>>>,
    performance_metrics: Arc<Mutex<PerformanceMetrics>>,
}

/// Performance metrics for communication
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_transferred: u64,
    pub average_latency_ms: f64,
    pub error_count: u64,
}

/// Trait for message handlers
pub trait MessageHandler: Send + Sync {
    fn handle_message(&mut self, message: &ModuleMessage) -> Result<(), ModuleError>;
    fn handler_name(&self) -> &str;
}

impl ModuleChannel {
    /// Create a new module channel
    pub fn new(name: String) -> Self {
        Self {
            name,
            sender: Arc::new(Mutex::new(Vec::new())),
            receivers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Send a message to all receivers
    pub fn send(&self, message: ModuleMessage) -> Result<(), ModuleError> {
        let receivers = self.receivers.lock().unwrap();
        let mut errors = Vec::new();

        for receiver in receivers.iter() {
            if let Err(e) = receiver.receive(&message) {
                errors.push(format!("Receiver error: {}", e));
            }
        }

        if !errors.is_empty() {
            Err(ModuleError::InitializationFailed {
                module: self.name.clone(),
                error: errors.join("; "),
            })
        } else {
            // Store message in sender buffer
            let mut sender = self.sender.lock().unwrap();
            sender.push(message);
            Ok(())
        }
    }

    /// Add a message receiver
    pub fn add_receiver(&self, receiver: Box<dyn MessageReceiver>) {
        let mut receivers = self.receivers.lock().unwrap();
        receivers.push(receiver);
    }

    /// Get pending messages
    pub fn get_messages(&self) -> Vec<ModuleMessage> {
        let mut sender = self.sender.lock().unwrap();
        let messages = sender.clone();
        sender.clear();
        messages
    }
}

impl ModuleCommunication {
    /// Create a new module communication manager
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
            message_handlers: Arc::new(Mutex::new(HashMap::new())),
            performance_metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
        }
    }

    /// Create a new communication channel
    pub fn create_channel(&self, name: String) -> Result<(), ModuleError> {
        let channel = ModuleChannel::new(name.clone());
        
        let mut channels = self.channels.lock().unwrap();
        if channels.contains_key(&name) {
            return Err(ModuleError::InitializationFailed {
                module: "communication".to_string(),
                error: format!("Channel '{}' already exists", name),
            });
        }

        channels.insert(name, channel);
        Ok(())
    }

    /// Send a message to a specific channel
    pub fn send_to_channel(&self, channel_name: &str, message: ModuleMessage) -> Result<(), ModuleError> {
        let channels = self.channels.lock().unwrap();
        
        match channels.get(channel_name) {
            Some(channel) => {
                // Update performance metrics
                {
                    let mut metrics = self.performance_metrics.lock().unwrap();
                    metrics.messages_sent += 1;
                    // Estimate message size (rough calculation)
                    metrics.bytes_transferred += 256; // Average message size
                }

                channel.send(message)
            }
            None => Err(ModuleError::InitializationFailed {
                module: "communication".to_string(),
                error: format!("Channel '{}' not found", channel_name),
            }),
        }
    }

    /// Register a message handler
    pub fn register_handler(&self, name: String, handler: Box<dyn MessageHandler>) -> Result<(), ModuleError> {
        let mut handlers = self.message_handlers.lock().unwrap();
        
        if handlers.contains_key(&name) {
            return Err(ModuleError::InitializationFailed {
                module: "communication".to_string(),
                error: format!("Handler '{}' already registered", name),
            });
        }

        handlers.insert(name, handler);
        Ok(())
    }

    /// Process incoming messages
    pub fn process_messages(&self) -> Result<(), ModuleError> {
        let channels = self.channels.lock().unwrap();
        let handlers = self.message_handlers.lock().unwrap();
        let mut errors = Vec::new();

        // Process messages from all channels
        for (channel_name, channel) in channels.iter() {
            let messages = channel.get_messages();
            
            for message in messages {
                // Update performance metrics
                {
                    let mut metrics = self.performance_metrics.lock().unwrap();
                    metrics.messages_received += 1;
                }

                // Find appropriate handler
                let message_type = match message {
                    ModuleMessage::UIEvent(_) => "ui_event",
                    ModuleMessage::DataSync(_) => "data_sync",
                    ModuleMessage::StatusUpdate(_) => "status_update",
                    ModuleMessage::ErrorNotification(_) => "error_notification",
                };

                if let Some(_handler) = handlers.get(message_type) {
                    // Note: This would need mutable access to handler in a real implementation
                    // For now, we'll just log the handling
                    println!("Handling message type '{}' on channel '{}'", message_type, channel_name);
                } else {
                    errors.push(format!("No handler for message type '{}' on channel '{}'", message_type, channel_name));
                }
            }
        }

        if !errors.is_empty() {
            Err(ModuleError::InitializationFailed {
                module: "communication".to_string(),
                error: errors.join("; "),
            })
        } else {
            Ok(())
        }
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        let metrics = self.performance_metrics.lock().unwrap();
        PerformanceMetrics {
            messages_sent: metrics.messages_sent,
            messages_received: metrics.messages_received,
            bytes_transferred: metrics.bytes_transferred,
            average_latency_ms: metrics.average_latency_ms,
            error_count: metrics.error_count,
        }
    }

    /// Reset performance metrics
    pub fn reset_metrics(&self) {
        let mut metrics = self.performance_metrics.lock().unwrap();
        *metrics = PerformanceMetrics::default();
    }

    /// Broadcast message to all channels
    pub fn broadcast(&self, message: ModuleMessage) -> Result<(), ModuleError> {
        let channels = self.channels.lock().unwrap();
        let mut errors = Vec::new();

        for (channel_name, channel) in channels.iter() {
            if let Err(e) = channel.send(message.clone()) {
                errors.push(format!("Channel '{}' error: {}", channel_name, e));
            }
        }

        if !errors.is_empty() {
            Err(ModuleError::InitializationFailed {
                module: "communication".to_string(),
                error: errors.join("; "),
            })
        } else {
            Ok(())
        }
    }
}

impl Default for ModuleCommunication {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReceiver {
        name: String,
        received_messages: Arc<Mutex<Vec<ModuleMessage>>>,
    }

    impl TestReceiver {
        fn new(name: String) -> Self {
            Self {
                name,
                received_messages: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl MessageReceiver for TestReceiver {
        fn receive(&self, message: &ModuleMessage) -> Result<(), ModuleError> {
            let mut messages = self.received_messages.lock().unwrap();
            messages.push(message.clone());
            Ok(())
        }

        fn can_handle(&self, _message_type: &str) -> bool {
            true // Test receiver handles all messages
        }
    }

    struct TestHandler {
        name: String,
        handled_count: Arc<Mutex<u32>>,
    }

    impl TestHandler {
        fn new(name: String) -> Self {
            Self {
                name,
                handled_count: Arc::new(Mutex::new(0)),
            }
        }
    }

    impl MessageHandler for TestHandler {
        fn handle_message(&mut self, _message: &ModuleMessage) -> Result<(), ModuleError> {
            let mut count = self.handled_count.lock().unwrap();
            *count += 1;
            Ok(())
        }

        fn handler_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_module_channel_creation() {
        let channel = ModuleChannel::new("test_channel".to_string());
        assert_eq!(channel.name, "test_channel");
    }

    #[test]
    fn test_module_channel_send_receive() {
        let channel = ModuleChannel::new("test_channel".to_string());
        let receiver = TestReceiver::new("test_receiver".to_string());
        channel.add_receiver(Box::new(receiver));

        let message = ModuleMessage::StatusUpdate(StatusUpdateMessage {
            status_type: StatusType::Ready,
            module_name: "test_module".to_string(),
            status: "ready".to_string(),
            details: None,
        });

        assert!(channel.send(message).is_ok());
        
        let messages = channel.get_messages();
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_module_communication_creation() {
        let comm = ModuleCommunication::new();
        let metrics = comm.get_performance_metrics();
        assert_eq!(metrics.messages_sent, 0);
        assert_eq!(metrics.messages_received, 0);
    }

    #[test]
    fn test_channel_creation() {
        let comm = ModuleCommunication::new();
        assert!(comm.create_channel("test_channel".to_string()).is_ok());
        
        // Should fail to create duplicate
        assert!(comm.create_channel("test_channel".to_string()).is_err());
    }

    #[test]
    fn test_handler_registration() {
        let comm = ModuleCommunication::new();
        let handler = TestHandler::new("test_handler".to_string());
        
        assert!(comm.register_handler("test_handler".to_string(), Box::new(handler)).is_ok());
        
        // Should fail to register duplicate
        let handler2 = TestHandler::new("test_handler2".to_string());
        assert!(comm.register_handler("test_handler".to_string(), Box::new(handler2)).is_err());
    }

    #[test]
    fn test_send_to_channel() {
        let comm = ModuleCommunication::new();
        comm.create_channel("test_channel".to_string()).unwrap();
        
        let message = ModuleMessage::StatusUpdate(StatusUpdateMessage {
            status_type: StatusType::Ready,
            module_name: "test".to_string(),
            status: "ready".to_string(),
            details: None,
        });

        // Should succeed - channel exists (even without receivers, message is stored)
        assert!(comm.send_to_channel("test_channel", message.clone()).is_ok());
        
        // Should fail - channel doesn't exist
        assert!(comm.send_to_channel("nonexistent", message.clone()).is_err());
    }

    #[test]
    fn test_performance_metrics() {
        let comm = ModuleCommunication::new();
        let initial_metrics = comm.get_performance_metrics();
        assert_eq!(initial_metrics.messages_sent, 0);
        
        comm.reset_metrics();
        let reset_metrics = comm.get_performance_metrics();
        assert_eq!(reset_metrics.messages_sent, 0);
        assert_eq!(reset_metrics.messages_received, 0);
    }

    #[test]
    fn test_message_serialization() {
        let message = ModuleMessage::UIEvent(UIEventMessage {
            event_type: UIEventType::ButtonClick,
            widget_id: "test_button".to_string(),
            data: serde_json::json!({"clicked": true}),
            timestamp: 1234567890,
        });

        // Test that message can be serialized to JSON
        let json = serde_json::to_string(&message);
        assert!(json.is_ok());
    }

    #[test]
    fn test_broadcast() {
        let comm = ModuleCommunication::new();
        comm.create_channel("channel1".to_string()).unwrap();
        comm.create_channel("channel2".to_string()).unwrap();
        
        let message = ModuleMessage::StatusUpdate(StatusUpdateMessage {
            status_type: StatusType::Ready,
            module_name: "test".to_string(),
            status: "ready".to_string(),
            details: None,
        });

        // Should succeed even without receivers
        assert!(comm.broadcast(message).is_ok());
    }
}